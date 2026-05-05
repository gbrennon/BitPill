use chrono::Datelike;

use crate::{
    application::{
        dtos::requests::SaveSettingsRequest, ports::inbound::save_settings_port::SaveSettingsPort,
    },
    domain::value_objects::navigation_mode::NavigationModeVariant,
    presentation::tui::{
        app::App,
        handlers::{
            create_medication_handler::CreateMedicationHandler,
            mark_dose_handler::MarkDoseHandler,
            medication_list_handler::MedicationListHandler,
            port::{Handler, HandlerResult},
        },
        input::Key,
        screen::Screen,
    },
};

#[derive(Default)]
pub struct EventHandler {
    medication_list_handler: MedicationListHandler,
    create_medication_handler: CreateMedicationHandler,
    edit_medication_handler:
        crate::presentation::tui::handlers::edit_medication_handler::EditMedicationHandler,
    mark_dose_handler: MarkDoseHandler,
}

impl Handler for EventHandler {
    fn handle(&mut self, app: &mut App, key: Key) -> HandlerResult {
        // Handle SettingsHelp screen specially - any key closes it (before global handlers)
        if let Screen::SettingsHelp { previous, .. } = &app.current_screen {
            app.current_screen = *previous.clone();
            return HandlerResult::Continue;
        }

        // Global quit: pressing 'q' anywhere opens a quit confirmation modal.
        if let Key::Char('q') = key
            && !matches!(app.current_screen, Screen::ConfirmQuit { .. })
        {
            app.current_screen = Screen::ConfirmQuit {
                previous: Box::new(app.current_screen.clone()),
            };
            return HandlerResult::Continue;
        }

        if let Key::Char('?') = key
            && let Some(variant) = app.get_navigation_mode()
        {
            let help_text = variant.help_text().to_string();
            let selected_index = NavigationModeVariant::variants()
                .iter()
                .position(|v| v == &variant)
                .unwrap_or(0);
            app.current_screen = Screen::SettingsHelp {
                vim_enabled: variant.is_vi(),
                selected_index,
                help_text,
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
                match key {
                    Key::Esc => {
                        app.current_screen = Screen::HomeScreen;
                    }
                    Key::Char('e') => {
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
                                "Custom" => 3,
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
                    Key::Char('m') => {
                        // open selection of today's registered dose records AND scheduled slots to mark as taken
                        if let Screen::MedicationDetails { id } = &app.current_screen
                            && let Some(m) = app.medications.iter().find(|m| m.id == *id)
                        {
                            use chrono::Local;

                            use crate::application::{
                                dtos::{
                                    requests::ListDoseRecordsRequest, responses::DoseRecordDto,
                                },
                                ports::inbound::list_dose_records_port::ListDoseRecordsPort,
                            };

                            let today = Local::now().date_naive();
                            // fetch ALL dose records for this medication (not just today's)
                            let all_records: Vec<DoseRecordDto> = match ListDoseRecordsPort::execute(
                                &*app.services.list_dose_records,
                                ListDoseRecordsRequest {
                                    medication_id: m.id.clone(),
                                },
                            ) {
                                Ok(resp) => resp.records,
                                Err(_) => Vec::new(),
                            };

                            // Only include untaken records
                            let mut records: Vec<DoseRecordDto> = all_records
                                .iter()
                                .filter(|r| r.taken_at.is_none())
                                .cloned()
                                .collect();

                            // append synthetic scheduled slots only if there isn't a record (taken or untaken) matching that slot
                            for (i, (h, mm)) in m.scheduled_time.iter().enumerate() {
                                let scheduled_dt = chrono::NaiveDate::from_ymd_opt(
                                    today.year(),
                                    today.month(),
                                    today.day(),
                                )
                                .and_then(|d| d.and_hms_opt(*h, *mm, 0))
                                .unwrap_or(Local::now().naive_local());

                                // Check if there's ANY record (taken or not) with scheduled time near this slot
                                let record_exists = all_records.iter().any(|r| {
                                    let diff = (r.scheduled_at - scheduled_dt).num_minutes().abs();
                                    diff <= 15
                                });
                                if record_exists {
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

                            let med_id = id.clone();
                            if records.is_empty() {
                                app.set_status("No doses to mark as taken", 3000);
                                app.current_screen = Screen::MedicationDetails { id: med_id };
                            } else {
                                app.current_screen = Screen::MarkDose {
                                    medication_id: med_id,
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
            Screen::Settings {
                vim_enabled,
                selected_index,
            } => {
                let mode_count = NavigationModeVariant::count();

                // Emacs mode: n/p for navigation
                if !app.is_vim_mode() {
                    if let Key::Char('n') = key {
                        let new_index = (*selected_index + 1) % mode_count;
                        let new_vim = NavigationModeVariant::from_index(new_index)
                            .map(|v| v.is_vi())
                            .unwrap_or(false);
                        app.current_screen = Screen::Settings {
                            vim_enabled: new_vim,
                            selected_index: new_index,
                        };
                        return HandlerResult::Continue;
                    }
                    if let Key::Char('p') = key {
                        let new_index = selected_index.saturating_sub(1);
                        let new_vim = NavigationModeVariant::from_index(new_index)
                            .map(|v| v.is_vi())
                            .unwrap_or(false);
                        app.current_screen = Screen::Settings {
                            vim_enabled: new_vim,
                            selected_index: new_index,
                        };
                        return HandlerResult::Continue;
                    }
                    if let Key::Char('f') = key {
                        let new_index = (*selected_index + 1) % mode_count;
                        let new_vim = NavigationModeVariant::from_index(new_index)
                            .map(|v| v.is_vi())
                            .unwrap_or(false);
                        app.current_screen = Screen::Settings {
                            vim_enabled: new_vim,
                            selected_index: new_index,
                        };
                        return HandlerResult::Continue;
                    }
                    if let Key::Char('b') = key {
                        let new_index = selected_index.saturating_sub(1);
                        let new_vim = NavigationModeVariant::from_index(new_index)
                            .map(|v| v.is_vi())
                            .unwrap_or(false);
                        app.current_screen = Screen::Settings {
                            vim_enabled: new_vim,
                            selected_index: new_index,
                        };
                        return HandlerResult::Continue;
                    }
                    // Skip vim keys in emacs mode
                    if matches!(
                        key,
                        Key::Char('j') | Key::Char('k') | Key::Char('h') | Key::Char('l')
                    ) {
                        return HandlerResult::Continue;
                    }
                }

                match key {
                    Key::Char('?') => {
                        let help_text = NavigationModeVariant::from_index(*selected_index)
                            .map(|v| v.help_text())
                            .unwrap_or("No help available")
                            .to_string();
                        app.current_screen = Screen::SettingsHelp {
                            vim_enabled: *vim_enabled,
                            selected_index: *selected_index,
                            help_text,
                            previous: Box::new(app.current_screen.clone()),
                        };
                    }
                    Key::Char(' ') => {
                        let new_index = (selected_index + 1) % mode_count;
                        let new_vim = NavigationModeVariant::from_index(new_index)
                            .map(|v| v.is_vi())
                            .unwrap_or(false);
                        app.current_screen = Screen::Settings {
                            vim_enabled: new_vim,
                            selected_index: new_index,
                        };
                    }
                    Key::Char('j') | Key::Down | Key::Char('l') | Key::Right => {
                        if !app.is_vim_mode() {
                            return HandlerResult::Continue;
                        }
                        let new_index = (selected_index + 1) % mode_count;
                        let new_vim = NavigationModeVariant::from_index(new_index)
                            .map(|v| v.is_vi())
                            .unwrap_or(false);
                        app.current_screen = Screen::Settings {
                            vim_enabled: new_vim,
                            selected_index: new_index,
                        };
                    }
                    Key::Char('k') | Key::Up | Key::Char('h') | Key::Left => {
                        if !app.is_vim_mode() {
                            return HandlerResult::Continue;
                        }
                        let new_index = selected_index.saturating_sub(1);
                        let new_vim = NavigationModeVariant::from_index(new_index)
                            .map(|v| v.is_vi())
                            .unwrap_or(false);
                        app.current_screen = Screen::Settings {
                            vim_enabled: new_vim,
                            selected_index: new_index,
                        };
                    }
                    Key::Char('s') | Key::Enter => {
                        if let Some(variant) = NavigationModeVariant::from_index(*selected_index) {
                            match SaveSettingsPort::execute(
                                &*app.services.save_settings,
                                SaveSettingsRequest::new(variant.as_str()),
                            ) {
                                Ok(_) => {
                                    app.set_status("Settings saved", 2000);
                                }
                                Err(e) => {
                                    app.set_status(format!("Save error: {e}"), 3000);
                                }
                            }
                        }
                        app.current_screen = Screen::HomeScreen;
                    }
                    Key::Esc => {
                        app.current_screen = Screen::HomeScreen;
                    }
                    _ => {}
                }
                HandlerResult::Continue
            }
            Screen::SettingsHelp { .. } => {
                if let Some(variant) = app.get_navigation_mode() {
                    let help_text = variant.help_text().to_string();
                    let selected_index = NavigationModeVariant::variants()
                        .iter()
                        .position(|v| v == &variant)
                        .unwrap_or(0);
                    app.current_screen = Screen::SettingsHelp {
                        vim_enabled: variant.is_vi(),
                        selected_index,
                        help_text,
                        previous: Box::new(app.current_screen.clone()),
                    };
                } else {
                    app.current_screen = Screen::HomeScreen;
                }
                HandlerResult::Continue
            }
            Screen::ConfirmDelete { .. } => {
                match key {
                    Key::Char('y') | Key::Char('Y') => {
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
                    Key::Char('n') | Key::Char('N') | Key::Esc => {
                        app.current_screen = Screen::HomeScreen;
                    }
                    _ => {}
                }
                HandlerResult::Continue
            }
            Screen::ConfirmCancel { previous } => {
                match key {
                    Key::Char('y') | Key::Char('Y') => {
                        app.current_screen = Screen::HomeScreen;
                    }
                    Key::Char('n') | Key::Char('N') | Key::Esc => {
                        // return to previous view
                        app.current_screen = *previous.clone();
                    }
                    _ => {}
                }
                HandlerResult::Continue
            }
            Screen::ConfirmQuit { previous } => {
                match key {
                    Key::Char('y') | Key::Char('Y') => {
                        app.should_quit = true;
                    }
                    Key::Char('n') | Key::Char('N') | Key::Esc => {
                        // return to previous view
                        app.current_screen = *previous.clone();
                    }
                    _ => {}
                }
                HandlerResult::Continue
            }
            Screen::ValidationError { previous, .. } => {
                // any key dismisses the modal
                app.current_screen = *previous.clone();
                HandlerResult::Continue
            }
            Screen::MarkDose { .. } => self.mark_dose_handler.handle(app, key),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::presentation::tui::{app::App, input::Key};

    struct FakeGetSettings;
    impl crate::application::ports::inbound::get_settings_port::GetSettingsPort for FakeGetSettings {
        fn execute(
            &self,
            _: crate::application::dtos::requests::GetSettingsRequest,
        ) -> Result<
            crate::application::dtos::responses::GetSettingsResponse,
            crate::application::errors::ApplicationError,
        > {
            Ok(crate::application::dtos::responses::GetSettingsResponse {
                navigation_mode: "vi".into(),
            })
        }
    }

    #[test]
    fn event_handler_smoke_exercises_branches() {
        let mut h = EventHandler::default();
        let mut app = App::default();

        // SettingsHelp dismissal
        app.current_screen = Screen::SettingsHelp {
            vim_enabled: true,
            selected_index: 0,
            help_text: "".into(),
            previous: Box::new(Screen::HomeScreen),
        };
        assert!(matches!(
            h.handle(&mut app, Key::Char('x')),
            HandlerResult::Continue
        ));
        assert!(matches!(app.current_screen, Screen::HomeScreen));

        // Global quit 'q'
        app.current_screen = Screen::HomeScreen;
        h.handle(&mut app, Key::Char('q'));
        assert!(matches!(app.current_screen, Screen::ConfirmQuit { .. }));

        // '?' opens SettingsHelp when GetSettings returns a mode
        app.services.get_settings = Arc::new(FakeGetSettings);
        app.current_screen = Screen::HomeScreen;
        h.handle(&mut app, Key::Char('?'));
        assert!(matches!(app.current_screen, Screen::SettingsHelp { .. }));

        // ConfirmDelete 'y' -> goes home (delete service default returns Ok)
        app.current_screen = Screen::ConfirmDelete {
            id: "id".into(),
            name: "n".into(),
        };
        h.handle(&mut app, Key::Char('y'));
        assert!(matches!(app.current_screen, Screen::HomeScreen));

        // ConfirmCancel 'n' -> returns to home
        app.current_screen = Screen::ConfirmCancel {
            previous: Box::new(Screen::HomeScreen),
        };
        h.handle(&mut app, Key::Char('n'));
        assert!(matches!(app.current_screen, Screen::HomeScreen));

        // ConfirmQuit 'y' -> sets should_quit
        app.current_screen = Screen::ConfirmQuit {
            previous: Box::new(Screen::HomeScreen),
        };
        app.should_quit = false;
        h.handle(&mut app, Key::Char('y'));
        assert!(app.should_quit);

        // ValidationError dismissal
        app.current_screen = Screen::ValidationError {
            messages: vec!["e".into()],
            previous: Box::new(Screen::HomeScreen),
        };
        h.handle(&mut app, Key::Char('x'));
        assert!(matches!(app.current_screen, Screen::HomeScreen));

        // MedicationDetails 'e' and 'm' paths: set a medication and ensure transitions
        let med_id = "med1".to_string();
        let med = crate::application::dtos::responses::MedicationDto {
            id: med_id.clone(),
            name: "Name".into(),
            amount_mg: 10,
            scheduled_time: vec![(12, 0)],
            dose_frequency: "OnceDaily".into(),
            taken_today: 0,
            scheduled_today: 0,
        };
        app.medications = vec![med];
        app.current_screen = Screen::MedicationDetails { id: med_id.clone() };
        h.handle(&mut app, Key::Char('e'));
        assert!(matches!(app.current_screen, Screen::EditMedication { .. }));

        // open mark-dose from MedicationDetails
        app.current_screen = Screen::MedicationDetails { id: med_id.clone() };
        h.handle(&mut app, Key::Char('m'));
        // either MarkDose or MedicationDetails (if no records) - ensure no panic
        assert!(matches!(
            app.current_screen,
            Screen::MarkDose { .. } | Screen::MedicationDetails { .. } | Screen::HomeScreen
        ));

        // Exercise CreateMedicationHandler via EventHandler by making CreateMedication screen and sending keys
        app.current_screen = Screen::CreateMedication {
            name: String::new(),
            amount_mg: String::new(),
            selected_frequency: 0,
            scheduled_time: vec![String::new()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: false,
        };
        // simulate typing a char
        h.handle(&mut app, Key::Char('a'));
        // simulate backspace
        h.handle(&mut app, Key::Backspace);
        // invalid submit -> should show ValidationError
        app.current_screen = Screen::CreateMedication {
            name: "Name".into(),
            amount_mg: "abc".into(),
            selected_frequency: 0,
            scheduled_time: vec!["12:00".into()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: false,
        };
        h.handle(&mut app, Key::Enter);
        assert!(matches!(app.current_screen, Screen::ValidationError { .. }));

        // valid submit
        app.current_screen = Screen::CreateMedication {
            name: "Name".into(),
            amount_mg: "10".into(),
            selected_frequency: 0,
            scheduled_time: vec!["12:00".into()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: false,
        };
        h.handle(&mut app, Key::Enter);
        assert!(matches!(app.current_screen, Screen::HomeScreen));

        // Exercise EditMedicationHandler via EventHandler
        let edit_id = "e1".to_string();
        app.current_screen = Screen::EditMedication {
            id: edit_id.clone(),
            name: "Name".into(),
            amount_mg: "10".into(),
            selected_frequency: 0,
            scheduled_time: vec!["12:00".into()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: false,
        };
        h.handle(&mut app, Key::Enter);
        assert!(matches!(
            app.current_screen,
            Screen::HomeScreen | Screen::ValidationError { .. }
        ));
    }

    struct FakeSettingsEmacs;
    impl crate::application::ports::inbound::get_settings_port::GetSettingsPort for FakeSettingsEmacs {
        fn execute(
            &self,
            _: crate::application::dtos::requests::GetSettingsRequest,
        ) -> Result<
            crate::application::dtos::responses::GetSettingsResponse,
            crate::application::errors::ApplicationError,
        > {
            Ok(crate::application::dtos::responses::GetSettingsResponse {
                navigation_mode: "emacs".into(),
            })
        }
    }

    #[test]
    fn settings_emacs_n_toggles_mode() {
        let mut h = EventHandler::default();
        let mut app = App::default();
        app.services.get_settings = Arc::new(FakeSettingsEmacs);
        app.current_screen = Screen::Settings {
            vim_enabled: false,
            selected_index: 0,
        };
        h.handle(&mut app, Key::Char('n'));
        assert!(matches!(app.current_screen, Screen::Settings { .. }));
    }

    #[test]
    fn settings_emacs_p_moves_up() {
        let mut h = EventHandler::default();
        let mut app = App::default();
        app.services.get_settings = Arc::new(FakeSettingsEmacs);
        app.current_screen = Screen::Settings {
            vim_enabled: false,
            selected_index: 1,
        };
        h.handle(&mut app, Key::Char('p'));
        if let Screen::Settings { selected_index, .. } = &app.current_screen {
            assert_eq!(*selected_index, 0);
        }
    }

    #[test]
    fn settings_emacs_f_toggles() {
        let mut h = EventHandler::default();
        let mut app = App::default();
        app.services.get_settings = Arc::new(FakeSettingsEmacs);
        app.current_screen = Screen::Settings {
            vim_enabled: false,
            selected_index: 0,
        };
        h.handle(&mut app, Key::Char('f'));
        assert!(matches!(app.current_screen, Screen::Settings { .. }));
    }

    #[test]
    fn settings_emacs_b_moves_up() {
        let mut h = EventHandler::default();
        let mut app = App::default();
        app.services.get_settings = Arc::new(FakeSettingsEmacs);
        app.current_screen = Screen::Settings {
            vim_enabled: false,
            selected_index: 1,
        };
        h.handle(&mut app, Key::Char('b'));
        if let Screen::Settings { selected_index, .. } = &app.current_screen {
            assert_eq!(*selected_index, 0);
        }
    }

    #[test]
    fn settings_emacs_skip_vim_keys() {
        let mut h = EventHandler::default();
        let mut app = App::default();
        app.services.get_settings = Arc::new(FakeSettingsEmacs);
        app.current_screen = Screen::Settings {
            vim_enabled: false,
            selected_index: 0,
        };
        for key in [
            Key::Char('j'),
            Key::Char('k'),
            Key::Char('h'),
            Key::Char('l'),
        ] {
            assert!(matches!(h.handle(&mut app, key), HandlerResult::Continue));
        }
    }

    #[test]
    fn settings_enter_saves() {
        let mut h = EventHandler::default();
        let mut app = App::default();
        app.services.get_settings = Arc::new(FakeGetSettings);
        app.current_screen = Screen::Settings {
            vim_enabled: true,
            selected_index: 0,
        };
        h.handle(&mut app, Key::Enter);
        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn confirm_delete_n_returns_home() {
        let mut h = EventHandler::default();
        let mut app = App::default();
        app.current_screen = Screen::ConfirmDelete {
            id: "x".into(),
            name: "n".into(),
        };
        h.handle(&mut app, Key::Char('n'));
        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn confirm_delete_esc_returns_home() {
        let mut h = EventHandler::default();
        let mut app = App::default();
        app.current_screen = Screen::ConfirmDelete {
            id: "x".into(),
            name: "n".into(),
        };
        h.handle(&mut app, Key::Esc);
        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn confirm_cancel_y_returns_home() {
        let mut h = EventHandler::default();
        let mut app = App::default();
        app.current_screen = Screen::ConfirmCancel {
            previous: Box::new(Screen::HomeScreen),
        };
        h.handle(&mut app, Key::Char('y'));
        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn confirm_cancel_esc_returns_previous() {
        let mut h = EventHandler::default();
        let mut app = App::default();
        let prev = Box::new(Screen::HomeScreen);
        app.current_screen = Screen::ConfirmCancel { previous: prev };
        h.handle(&mut app, Key::Esc);
        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn confirm_quit_n_returns_previous() {
        let mut h = EventHandler::default();
        let mut app = App::default();
        let prev = Box::new(Screen::HomeScreen);
        app.current_screen = Screen::ConfirmQuit { previous: prev };
        h.handle(&mut app, Key::Char('n'));
        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn confirm_quit_esc_returns_previous() {
        let mut h = EventHandler::default();
        let mut app = App::default();
        let prev = Box::new(Screen::HomeScreen);
        app.current_screen = Screen::ConfirmQuit { previous: prev };
        h.handle(&mut app, Key::Esc);
        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn home_screen_routes_to_list_handler() {
        let mut h = EventHandler::default();
        let mut app = App::default();
        app.services.get_settings = Arc::new(FakeGetSettings);
        app.current_screen = Screen::HomeScreen;
        app.medications = vec![crate::application::dtos::responses::MedicationDto {
            id: "m1".into(),
            name: "A".into(),
            amount_mg: 10,
            scheduled_time: vec![(8, 0)],
            dose_frequency: "OnceDaily".into(),
            taken_today: 0,
            scheduled_today: 0,
        }];
        h.handle(&mut app, Key::Char('c'));
        assert!(matches!(
            app.current_screen,
            Screen::CreateMedication { .. }
        ));
    }

    #[test]
    fn medication_details_esc_returns_home() {
        let mut h = EventHandler::default();
        let mut app = App::default();
        app.current_screen = Screen::MedicationDetails { id: "m1".into() };
        h.handle(&mut app, Key::Esc);
        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn mark_dose_routes_to_handler() {
        let mut h = EventHandler::default();
        let mut app = App::default();
        app.services.get_settings = Arc::new(FakeGetSettings);
        app.current_screen = Screen::MarkDose {
            medication_id: "m1".into(),
            records: vec![],
            selected_index: 0,
        };
        h.handle(&mut app, Key::Esc);
        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn settings_help_route_exercised() {
        let mut h = EventHandler::default();
        let mut app = App::default();
        app.services.get_settings = Arc::new(FakeGetSettings);
        app.current_screen = Screen::SettingsHelp {
            vim_enabled: true,
            selected_index: 0,
            help_text: "h".into(),
            previous: Box::new(Screen::HomeScreen),
        };
        // any key closes SettingsHelp (already tested in smoke) but re-opening from SettingsHelp itself
        h.handle(&mut app, Key::Char('x'));
        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }
}
