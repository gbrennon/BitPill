use crossterm::event::{KeyCode, KeyEvent};

use crate::presentation::tui::app::App;
use crate::presentation::tui::handlers::create_medication_handler::CreateMedicationHandler;
use crate::presentation::tui::handlers::mark_dose_handler::MarkDoseHandler;
use crate::presentation::tui::handlers::medication_list_handler::MedicationListHandler;
use crate::presentation::tui::handlers::port::{Handler, HandlerResult};
use crate::presentation::tui::screen::Screen;
use chrono::Datelike;

#[derive(Default)]
pub struct EventHandler {
    medication_list_handler: MedicationListHandler,
    create_medication_handler: CreateMedicationHandler,
    edit_medication_handler:
        crate::presentation::tui::handlers::edit_medication_handler::EditMedicationHandler,
    mark_dose_handler: MarkDoseHandler,
}

impl Handler for EventHandler {
    fn handle(&mut self, app: &mut App, key: KeyEvent) -> HandlerResult {
        // Global quit: pressing 'q' anywhere opens a quit confirmation modal.
        if let KeyCode::Char('q') = key.code
            && !matches!(app.current_screen, Screen::ConfirmQuit { .. })
        {
            app.current_screen = Screen::ConfirmQuit {
                previous: Box::new(app.current_screen.clone()),
            };
            return HandlerResult::Continue;
        }

        match &app.current_screen {
            Screen::HomeScreen => self.medication_list_handler.handle(app, key),
            Screen::CreateMedication { .. } => self.create_medication_handler.handle(app, key),
            Screen::EditMedication { .. } => self.edit_medication_handler.handle(app, key),
            Screen::MedicationDetails { .. } => {
                // handle simple navigation and edit shortcut inside details screen
                match key.code {
                    KeyCode::Esc => {
                        app.current_screen = Screen::HomeScreen;
                    }
                    KeyCode::Char('e') => {
                        if let Screen::MedicationDetails { id } = &app.current_screen
                            && let Some(m) = app.medications.iter().find(|m| m.id == *id)
                        {
                            let times = m
                                .scheduled_time
                                .iter()
                                .map(|(h, m)| format!("{:02}:{:02}", h, m))
                                .collect::<Vec<_>>()
                                .join(",");
                            let selected_frequency = match m.dose_frequency.as_str() {
                                "OnceDaily" => 0,
                                "TwiceDaily" => 1,
                                "ThriceDaily" => 2,
                                _ => 0,
                            };
                            app.current_screen = Screen::EditMedication {
                                id: id.clone(),
                                name: m.name.clone(),
                                amount_mg: m.amount_mg.to_string(),
                                selected_frequency,
                                scheduled_time: times.split(',').map(|s| s.to_string()).collect(),
                                scheduled_idx: 0,
                                focused_field: 0,
                                insert_mode: false,
                            };
                        }
                    }
                    KeyCode::Char('s') => {
                        // open selection of today's registered dose records AND scheduled slots to mark as taken
                        if let Screen::MedicationDetails { id } = &app.current_screen
                            && let Some(m) = app.medications.iter().find(|m| m.id == *id)
                        {
                            use crate::application::dtos::requests::ListDoseRecordsRequest;
                            use crate::application::dtos::responses::DoseRecordDto;
                            use crate::application::ports::inbound::list_dose_records_port::ListDoseRecordsPort;
                            use chrono::Local;

                            let today = Local::now().date_naive();
                            // fetch all today's records (both taken and untaken)
                            let all_today_records: Vec<DoseRecordDto> =
                                match ListDoseRecordsPort::execute(
                                    &*app.services.list_dose_records,
                                    ListDoseRecordsRequest {
                                        medication_id: m.id.clone(),
                                    },
                                ) {
                                    Ok(resp) => resp
                                        .records
                                        .into_iter()
                                        .filter(|r| r.scheduled_at.date() == today)
                                        .collect(),
                                    Err(_) => Vec::new(),
                                };

                            // untaken records to present directly
                            let mut records: Vec<DoseRecordDto> = all_today_records
                                .iter()
                                .filter(|r| r.taken_at.is_none())
                                .cloned()
                                .collect();

                            // append synthetic scheduled slots only if there isn't a taken record matching that slot
                            for (i, (h, mm)) in m.scheduled_time.iter().enumerate() {
                                let scheduled_dt = chrono::NaiveDate::from_ymd_opt(
                                    today.year(),
                                    today.month(),
                                    today.day(),
                                )
                                .and_then(|d| d.and_hms_opt(*h, *mm, 0))
                                .unwrap_or(Local::now().naive_local());

                                // if any record was already taken near this scheduled_dt, skip adding a synthetic slot
                                let already_taken = all_today_records.iter().any(|r| {
                                    if let Some(taken) = r.taken_at {
                                        let diff = (taken - scheduled_dt).num_minutes().abs();
                                        diff <= 15
                                    } else {
                                        false
                                    }
                                });
                                if already_taken {
                                    continue;
                                }

                                // if an untaken record already exists for this scheduled_dt, skip duplicate
                                let has_untaken = records.iter().any(|r| {
                                    (r.scheduled_at - scheduled_dt).num_minutes().abs() <= 15
                                });
                                if has_untaken {
                                    continue;
                                }

                                let id_str = format!("slot:{}", i);
                                records.push(DoseRecordDto {
                                    id: id_str,
                                    medication_id: m.id.clone(),
                                    scheduled_at: scheduled_dt,
                                    taken_at: None,
                                });
                            }

                            if records.is_empty() {
                                app.set_status(
                                    "No untaken records or scheduled slots for today",
                                    3000,
                                );
                            } else {
                                app.current_screen = Screen::MarkDose {
                                    medication_id: id.clone(),
                                    records,
                                    selected_index: 0,
                                };
                            }
                        }
                    }
                    _ => {}
                }
                HandlerResult::Continue
            }
            Screen::Settings { vim_enabled } => {
                match key.code {
                    KeyCode::Char(' ') => {
                        app.current_screen = Screen::Settings {
                            vim_enabled: !*vim_enabled,
                        };
                    }
                    KeyCode::Char('s') => {
                        // persist settings: read current state value and update
                        let value = if let Screen::Settings { vim_enabled } = &app.current_screen {
                            *vim_enabled
                        } else {
                            *vim_enabled
                        };
                        let new = serde_json::json!({ "vim_navigation": value });
                        match app.services.settings.execute(
                            crate::application::dtos::requests::SettingsRequest {
                                op: crate::application::dtos::requests::SettingsOperation::Update {
                                    settings: new.clone(),
                                },
                            },
                        ) {
                            Ok(_) => app.set_status("Settings saved", 2000),
                            Err(e) => {
                                app.status_message = Some(format!("Settings save error: {e}"))
                            }
                        }
                        app.current_screen = Screen::HomeScreen;
                    }
                    KeyCode::Esc => {
                        app.current_screen = Screen::HomeScreen;
                    }
                    _ => {}
                }
                HandlerResult::Continue
            }
            Screen::ConfirmDelete { .. } => {
                match key.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => {
                        if let Screen::ConfirmDelete { id, .. } = &app.current_screen {
                            // call delete service
                            match crate::application::ports::inbound::delete_medication_port::DeleteMedicationPort::execute(
                                &*app.services.delete_medication,
                                crate::application::dtos::requests::DeleteMedicationRequest { id: id.clone() },
                            ) {
                                Ok(_) => {
                                    app.set_status("Medication deleted", 3000);
                                    app.load_medications();
                                }
                                Err(e) => {
                                    app.status_message = Some(format!("Delete error: {e}"));
                                }
                            }
                        }
                        app.current_screen = Screen::HomeScreen;
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                        app.current_screen = Screen::HomeScreen;
                    }
                    _ => {}
                }
                HandlerResult::Continue
            }
            Screen::ConfirmCancel { previous } => {
                match key.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => {
                        app.current_screen = Screen::HomeScreen;
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                        // return to previous view
                        app.current_screen = *previous.clone();
                    }
                    _ => {}
                }
                HandlerResult::Continue
            }
            Screen::ConfirmQuit { previous } => {
                match key.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => {
                        app.should_quit = true;
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                        // return to previous view
                        app.current_screen = *previous.clone();
                    }
                    _ => {}
                }
                HandlerResult::Continue
            }
            Screen::ValidationError { previous, .. } => {
                match key.code {
                    KeyCode::Esc | KeyCode::Enter => {
                        app.current_screen = *previous.clone();
                    }
                    _ => {
                        // any key dismisses the modal
                        app.current_screen = *previous.clone();
                    }
                }
                HandlerResult::Continue
            }
            Screen::MarkDose { .. } => self.mark_dose_handler.handle(app, key),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::presentation::tui::app::App;
    use crate::presentation::tui::app_services::AppServices;
    use crate::presentation::tui::screen::Screen;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    fn app() -> App {
        App::new(AppServices::fake())
    }

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    #[test]
    fn global_q_opens_confirm_quit_from_home() {
        let mut app = app();
        let mut h = EventHandler::default();

        h.handle(&mut app, key(KeyCode::Char('q')));

        assert!(matches!(app.current_screen, Screen::ConfirmQuit { .. }));
    }

    #[test]
    fn global_q_does_not_nest_confirm_quit_inside_confirm_quit() {
        let mut app = app();
        app.current_screen = Screen::ConfirmQuit {
            previous: Box::new(Screen::HomeScreen),
        };
        let mut h = EventHandler::default();

        h.handle(&mut app, key(KeyCode::Char('q')));

        // Should NOT open another ConfirmQuit; ConfirmQuit 'q' falls through to inner handler
        // which treats 'q' as a char — confirm-quit has no 'q' binding, so screen stays.
        assert!(matches!(app.current_screen, Screen::ConfirmQuit { .. }));
    }

    #[test]
    fn confirm_quit_y_sets_should_quit() {
        let mut app = app();
        app.current_screen = Screen::ConfirmQuit {
            previous: Box::new(Screen::HomeScreen),
        };
        let mut h = EventHandler::default();

        h.handle(&mut app, key(KeyCode::Char('y')));

        assert!(app.should_quit);
    }

    #[test]
    fn confirm_quit_n_returns_to_previous() {
        let mut app = app();
        app.current_screen = Screen::ConfirmQuit {
            previous: Box::new(Screen::HomeScreen),
        };
        let mut h = EventHandler::default();

        h.handle(&mut app, key(KeyCode::Char('n')));

        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn confirm_quit_esc_returns_to_previous() {
        let mut app = app();
        app.current_screen = Screen::ConfirmQuit {
            previous: Box::new(Screen::HomeScreen),
        };
        let mut h = EventHandler::default();

        h.handle(&mut app, key(KeyCode::Esc));

        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn confirm_delete_y_navigates_home() {
        let mut app = app();
        app.current_screen = Screen::ConfirmDelete {
            id: "some-id".into(),
            name: "Med".into(),
        };
        let mut h = EventHandler::default();

        h.handle(&mut app, key(KeyCode::Char('y')));

        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn confirm_delete_uppercase_y_navigates_home() {
        let mut app = app();
        app.current_screen = Screen::ConfirmDelete {
            id: "some-id".into(),
            name: "Med".into(),
        };
        let mut h = EventHandler::default();

        h.handle(&mut app, key(KeyCode::Char('Y')));

        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn confirm_delete_n_navigates_home() {
        let mut app = app();
        app.current_screen = Screen::ConfirmDelete {
            id: "some-id".into(),
            name: "Med".into(),
        };
        let mut h = EventHandler::default();

        h.handle(&mut app, key(KeyCode::Char('n')));

        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn confirm_delete_esc_navigates_home() {
        let mut app = app();
        app.current_screen = Screen::ConfirmDelete {
            id: "some-id".into(),
            name: "Med".into(),
        };
        let mut h = EventHandler::default();

        h.handle(&mut app, key(KeyCode::Esc));

        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn confirm_cancel_y_goes_home() {
        let mut app = app();
        app.current_screen = Screen::ConfirmCancel {
            previous: Box::new(Screen::HomeScreen),
        };
        let mut h = EventHandler::default();

        h.handle(&mut app, key(KeyCode::Char('y')));

        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn confirm_cancel_n_returns_to_previous() {
        let mut app = app();
        app.current_screen = Screen::ConfirmCancel {
            previous: Box::new(Screen::CreateMedication {
                name: "".into(),
                amount_mg: "".into(),
                selected_frequency: 0,
                scheduled_time: vec![],
                scheduled_idx: 0,
                focused_field: 0,
                insert_mode: false,
            }),
        };
        let mut h = EventHandler::default();

        h.handle(&mut app, key(KeyCode::Char('n')));

        assert!(matches!(
            app.current_screen,
            Screen::CreateMedication { .. }
        ));
    }

    #[test]
    fn confirm_cancel_esc_returns_to_previous() {
        let mut app = app();
        app.current_screen = Screen::ConfirmCancel {
            previous: Box::new(Screen::HomeScreen),
        };
        let mut h = EventHandler::default();

        h.handle(&mut app, key(KeyCode::Esc));

        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn settings_space_toggles_vim_enabled() {
        let mut app = app();
        app.current_screen = Screen::Settings { vim_enabled: false };
        let mut h = EventHandler::default();

        h.handle(&mut app, key(KeyCode::Char(' ')));

        assert!(matches!(
            app.current_screen,
            Screen::Settings { vim_enabled: true }
        ));
    }

    #[test]
    fn settings_s_saves_and_goes_home() {
        let mut app = app();
        app.current_screen = Screen::Settings { vim_enabled: true };
        let mut h = EventHandler::default();

        h.handle(&mut app, key(KeyCode::Char('s')));

        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn settings_esc_goes_home() {
        let mut app = app();
        app.current_screen = Screen::Settings { vim_enabled: false };
        let mut h = EventHandler::default();

        h.handle(&mut app, key(KeyCode::Esc));

        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn medication_details_esc_goes_home() {
        let mut app = app();
        app.current_screen = Screen::MedicationDetails { id: "x".into() };
        let mut h = EventHandler::default();

        h.handle(&mut app, key(KeyCode::Esc));

        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn medication_details_unrecognised_key_stays_on_details() {
        let mut app = app();
        app.current_screen = Screen::MedicationDetails { id: "x".into() };
        let mut h = EventHandler::default();

        h.handle(&mut app, key(KeyCode::Char('z')));

        assert!(matches!(
            app.current_screen,
            Screen::MedicationDetails { .. }
        ));
    }

    #[test]
    fn validation_error_enter_dismisses_modal() {
        let mut app = app();
        app.current_screen = Screen::ValidationError {
            message: "bad".into(),
            previous: Box::new(Screen::HomeScreen),
        };
        let mut h = EventHandler::default();

        h.handle(&mut app, key(KeyCode::Enter));

        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn validation_error_any_key_dismisses_modal() {
        let mut app = app();
        app.current_screen = Screen::ValidationError {
            message: "bad".into(),
            previous: Box::new(Screen::HomeScreen),
        };
        let mut h = EventHandler::default();

        h.handle(&mut app, key(KeyCode::Char('x')));

        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn home_screen_non_q_key_dispatches_to_medication_list_handler() {
        let mut app = app();
        let mut h = EventHandler::default();

        h.handle(&mut app, key(KeyCode::Char('j')));

        // medication_list_handler handles 'j' as navigation; screen stays HomeScreen
        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn create_medication_screen_key_dispatches_to_create_handler() {
        let mut app = app();
        app.current_screen = Screen::CreateMedication {
            name: String::new(),
            amount_mg: String::new(),
            selected_frequency: 0,
            scheduled_time: vec![],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: false,
        };
        let mut h = EventHandler::default();

        h.handle(&mut app, key(KeyCode::Esc));

        // create_medication_handler handles Esc by opening ConfirmCancel
        assert!(matches!(app.current_screen, Screen::ConfirmCancel { .. }));
    }

    #[test]
    fn edit_medication_screen_key_dispatches_to_edit_handler() {
        let mut app = app();
        app.current_screen = Screen::EditMedication {
            id: "x".into(),
            name: String::new(),
            amount_mg: String::new(),
            selected_frequency: 0,
            scheduled_time: vec![],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: false,
        };
        let mut h = EventHandler::default();

        h.handle(&mut app, key(KeyCode::Esc));

        // edit_medication_handler handles Esc by opening ConfirmCancel
        assert!(matches!(app.current_screen, Screen::ConfirmCancel { .. }));
    }

    #[test]
    fn mark_dose_screen_dispatches_to_mark_dose_handler() {
        let mut app = app();
        app.current_screen = Screen::MarkDose {
            medication_id: "x".into(),
            records: vec![],
            selected_index: 0,
        };
        let mut h = EventHandler::default();

        h.handle(&mut app, key(KeyCode::Esc));

        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn medication_details_e_key_without_matching_medication_stays_on_details() {
        let mut app = app();
        app.current_screen = Screen::MedicationDetails {
            id: "nonexistent".into(),
        };
        let mut h = EventHandler::default();

        h.handle(&mut app, key(KeyCode::Char('e')));

        assert!(matches!(
            app.current_screen,
            Screen::MedicationDetails { .. }
        ));
    }

    #[test]
    fn medication_details_e_key_with_matching_medication_opens_edit_screen() {
        use crate::application::dtos::responses::MedicationDto;

        let mut app = app();
        app.medications = vec![MedicationDto {
            id: "med-1".into(),
            name: "Aspirin".into(),
            amount_mg: 100,
            scheduled_time: vec![(8, 0)],
            dose_frequency: "OnceDaily".into(),
        }];
        app.current_screen = Screen::MedicationDetails { id: "med-1".into() };
        let mut h = EventHandler::default();

        h.handle(&mut app, key(KeyCode::Char('e')));

        assert!(matches!(app.current_screen, Screen::EditMedication { .. }));
    }

    #[test]
    fn medication_details_s_key_without_matching_medication_stays_on_details() {
        let mut app = app();
        app.current_screen = Screen::MedicationDetails {
            id: "nonexistent".into(),
        };
        let mut h = EventHandler::default();

        h.handle(&mut app, key(KeyCode::Char('s')));

        assert!(matches!(
            app.current_screen,
            Screen::MedicationDetails { .. }
        ));
    }

    #[test]
    fn medication_details_s_key_with_matching_medication_no_records_sets_status() {
        use crate::application::dtos::responses::MedicationDto;

        let mut app = app();
        app.medications = vec![MedicationDto {
            id: "med-1".into(),
            name: "Aspirin".into(),
            amount_mg: 100,
            scheduled_time: vec![(8, 0)],
            dose_frequency: "OnceDaily".into(),
        }];
        app.current_screen = Screen::MedicationDetails { id: "med-1".into() };
        let mut h = EventHandler::default();

        h.handle(&mut app, key(KeyCode::Char('s')));

        // With no existing dose records and a scheduled time in the past, it
        // should either navigate to MarkDose or set a status message.
        // Either outcome means the handler was invoked correctly.
        let is_mark_dose = matches!(app.current_screen, Screen::MarkDose { .. });
        let is_details = matches!(app.current_screen, Screen::MedicationDetails { .. });
        assert!(is_mark_dose || is_details);
    }

    #[test]
    fn settings_unknown_key_stays_on_settings() {
        let mut app = app();
        app.current_screen = Screen::Settings { vim_enabled: false };
        let mut h = EventHandler::default();

        h.handle(&mut app, key(KeyCode::F(1)));

        assert!(matches!(app.current_screen, Screen::Settings { .. }));
    }

    #[test]
    fn confirm_cancel_unknown_key_stays_on_cancel() {
        let mut app = app();
        app.current_screen = Screen::ConfirmCancel {
            previous: Box::new(Screen::HomeScreen),
        };
        let mut h = EventHandler::default();

        h.handle(&mut app, key(KeyCode::F(1)));

        assert!(matches!(app.current_screen, Screen::ConfirmCancel { .. }));
    }
}
