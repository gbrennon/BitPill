use crate::application::dtos::requests::EditMedicationRequest;
use crate::presentation::tui::app::App;
use crate::presentation::tui::handlers::medication_form_navigation::{
    NavigationState, navigate_down, navigate_left, navigate_right, navigate_up, remove_custom_slot,
};
use crate::presentation::tui::handlers::port::{Handler, HandlerResult};
use crate::presentation::tui::handlers::time_slot_parser::{
    frequency_str, parse_slots, validate_slot_count,
};
use crate::presentation::tui::screen::Screen;
use crossterm::event::KeyEvent;

pub struct EditMedicationHandler;

impl Default for EditMedicationHandler {
    fn default() -> Self {
        EditMedicationHandler
    }
}

impl Handler for EditMedicationHandler {
    fn handle(&mut self, app: &mut App, key: KeyEvent) -> HandlerResult {
        let (
            id,
            name,
            amount_mg,
            selected_frequency,
            scheduled_time,
            scheduled_idx,
            focused_field,
            insert_mode,
        ) = match &app.current_screen {
            Screen::EditMedication {
                id,
                name,
                amount_mg,
                selected_frequency,
                scheduled_time,
                scheduled_idx,
                focused_field,
                insert_mode,
            } => (
                id.clone(),
                name.clone(),
                amount_mg.clone(),
                *selected_frequency,
                scheduled_time.clone(),
                *scheduled_idx,
                *focused_field,
                *insert_mode,
            ),
            _ => return HandlerResult::Continue,
        };

        let nav = NavigationState {
            focused_field,
            scheduled_time,
            scheduled_idx,
        };

        let set_screen = |app: &mut App,
                          id: String,
                          name: String,
                          amount_mg: String,
                          sel_freq: usize,
                          nav: NavigationState,
                          insert_mode: bool| {
            app.current_screen = Screen::EditMedication {
                id,
                name,
                amount_mg,
                selected_frequency: sel_freq,
                scheduled_time: nav.scheduled_time,
                scheduled_idx: nav.scheduled_idx,
                focused_field: nav.focused_field,
                insert_mode,
            };
        };

        match key.code {
            crossterm::event::KeyCode::Esc => {
                if insert_mode {
                    set_screen(app, id, name, amount_mg, selected_frequency, nav, false);
                } else {
                    Self::handle_esc_normal_mode(
                        app,
                        &id,
                        &name,
                        &amount_mg,
                        selected_frequency,
                        &nav.scheduled_time,
                    );
                }
            }
            crossterm::event::KeyCode::Tab
            | crossterm::event::KeyCode::Right
            | crossterm::event::KeyCode::Char('l')
                if !insert_mode =>
            {
                let (sel, new_nav) = navigate_right(nav, selected_frequency);
                set_screen(app, id, name, amount_mg, sel, new_nav, insert_mode);
            }
            crossterm::event::KeyCode::Char('j') | crossterm::event::KeyCode::Down
                if !insert_mode =>
            {
                let new_nav = navigate_down(nav, selected_frequency);
                set_screen(
                    app,
                    id,
                    name,
                    amount_mg,
                    selected_frequency,
                    new_nav,
                    insert_mode,
                );
            }
            crossterm::event::KeyCode::Char('h') | crossterm::event::KeyCode::Left
                if !insert_mode =>
            {
                let (sel, new_nav) = navigate_left(nav, selected_frequency);
                set_screen(app, id, name, amount_mg, sel, new_nav, insert_mode);
            }
            crossterm::event::KeyCode::Char('k') | crossterm::event::KeyCode::Up
                if !insert_mode =>
            {
                let new_nav = navigate_up(nav);
                set_screen(
                    app,
                    id,
                    name,
                    amount_mg,
                    selected_frequency,
                    new_nav,
                    insert_mode,
                );
            }
            crossterm::event::KeyCode::Char('d')
                if !insert_mode
                    && focused_field == 3
                    && selected_frequency == 3
                    && nav.scheduled_time.len() > 1 =>
            {
                let (new_slots, new_idx) =
                    remove_custom_slot(nav.scheduled_time, nav.scheduled_idx);
                let new_nav = NavigationState {
                    focused_field,
                    scheduled_time: new_slots,
                    scheduled_idx: new_idx,
                };
                set_screen(
                    app,
                    id,
                    name,
                    amount_mg,
                    selected_frequency,
                    new_nav,
                    insert_mode,
                );
            }
            crossterm::event::KeyCode::Enter => {
                let parsed_amount: u32 = match amount_mg.trim().parse() {
                    Ok(v) => v,
                    Err(_) => {
                        app.current_screen = Screen::ValidationError {
                            message: "Invalid amount_mg value".into(),
                            previous: Box::new(app.current_screen.clone()),
                        };
                        return HandlerResult::Continue;
                    }
                };

                match parse_slots(&nav.scheduled_time) {
                    Err(e) => {
                        app.current_screen = Screen::ValidationError {
                            message: e.to_string(),
                            previous: Box::new(app.current_screen.clone()),
                        };
                        set_screen(
                            app,
                            id,
                            name,
                            amount_mg,
                            selected_frequency,
                            nav,
                            insert_mode,
                        );
                        return HandlerResult::Continue;
                    }
                    Ok(parsed) => {
                        if let Err(msg) =
                            validate_slot_count(selected_frequency, parsed.times.len())
                        {
                            app.current_screen = Screen::ValidationError {
                                message: msg.clone(),
                                previous: Box::new(app.current_screen.clone()),
                            };
                            let new_nav = NavigationState {
                                focused_field,
                                scheduled_time: parsed.normalized,
                                scheduled_idx,
                            };
                            set_screen(
                                app,
                                id,
                                name,
                                amount_mg,
                                selected_frequency,
                                new_nav,
                                insert_mode,
                            );
                            return HandlerResult::Continue;
                        }

                        match app.services.edit_medication.execute(EditMedicationRequest {
                            id: id.clone(),
                            name: name.clone(),
                            amount_mg: parsed_amount,
                            scheduled_time: parsed.times,
                            dose_frequency: frequency_str(selected_frequency).to_string(),
                        }) {
                            Ok(_) => {
                                app.load_medications();
                                app.set_status("Medication updated successfully", 3000);
                                app.current_screen = Screen::HomeScreen;
                            }
                            Err(e) => {
                                app.status_message = Some(format!("Update error: {e}"));
                                app.current_screen = Screen::HomeScreen;
                            }
                        }
                    }
                }
            }
            crossterm::event::KeyCode::Backspace if insert_mode => {
                let NavigationState {
                    focused_field,
                    mut scheduled_time,
                    scheduled_idx,
                } = nav;
                let mut name = name;
                let mut amount_mg = amount_mg;
                match focused_field {
                    0 => {
                        name.pop();
                    }
                    1 => {
                        amount_mg.pop();
                    }
                    3 => {
                        if scheduled_time.len() > scheduled_idx {
                            scheduled_time[scheduled_idx].pop();
                        }
                    }
                    _ => {}
                }
                let new_nav = NavigationState {
                    focused_field,
                    scheduled_time,
                    scheduled_idx,
                };
                set_screen(
                    app,
                    id,
                    name,
                    amount_mg,
                    selected_frequency,
                    new_nav,
                    insert_mode,
                );
            }
            crossterm::event::KeyCode::Char('i') if !insert_mode => {
                set_screen(app, id, name, amount_mg, selected_frequency, nav, true);
            }
            crossterm::event::KeyCode::Char(c) if insert_mode => {
                let NavigationState {
                    focused_field,
                    mut scheduled_time,
                    scheduled_idx,
                } = nav;
                let mut name = name;
                let mut amount_mg = amount_mg;
                match focused_field {
                    0 => name.push(c),
                    1 => amount_mg.push(c),
                    3 => {
                        while scheduled_time.len() <= scheduled_idx {
                            scheduled_time.push(String::new());
                        }
                        scheduled_time[scheduled_idx].push(c);
                    }
                    _ => {}
                }
                let new_nav = NavigationState {
                    focused_field,
                    scheduled_time,
                    scheduled_idx,
                };
                set_screen(
                    app,
                    id,
                    name,
                    amount_mg,
                    selected_frequency,
                    new_nav,
                    insert_mode,
                );
            }
            _ => {}
        }

        HandlerResult::Continue
    }
}

impl EditMedicationHandler {
    /// Checks whether the form values differ from the persisted medication.
    /// If changed, shows ConfirmCancel; otherwise navigates directly to HomeScreen.
    fn handle_esc_normal_mode(
        app: &mut App,
        id: &str,
        name: &str,
        amount_mg: &str,
        selected_frequency: usize,
        scheduled_time: &[String],
    ) {
        use crate::application::dtos::requests::GetMedicationRequest;
        use crate::application::ports::inbound::get_medication_port::GetMedicationPort;
        let changed = match GetMedicationPort::execute(
            &*app.services.get_medication,
            GetMedicationRequest { id: id.to_string() },
        ) {
            Ok(resp) => {
                let orig = resp.medication;
                let parsed_amount = amount_mg.trim().parse::<u32>().ok();
                let mut parsed_slots: Vec<(u32, u32)> = Vec::new();
                let mut parse_error = false;
                for slot in scheduled_time {
                    let part = slot.trim();
                    if part.is_empty() {
                        continue;
                    }
                    let mut iter = part.split(':');
                    let h = iter.next().and_then(|s| s.parse::<u32>().ok());
                    let m = iter.next().and_then(|s| s.parse::<u32>().ok());
                    match (h, m) {
                        (Some(h), Some(m)) => parsed_slots.push((h, m)),
                        _ => {
                            parse_error = true;
                            break;
                        }
                    }
                }
                parse_error
                    || orig.name != name
                    || (parsed_amount != Some(orig.amount_mg))
                    || orig.dose_frequency != frequency_str(selected_frequency)
                    || parsed_slots.len() != orig.scheduled_time.len()
                    || parsed_slots
                        .iter()
                        .zip(orig.scheduled_time.iter())
                        .any(|(a, b)| a.0 != b.0 || a.1 != b.1)
            }
            Err(_) => true,
        };

        if changed {
            app.current_screen = Screen::ConfirmCancel {
                previous: Box::new(app.current_screen.clone()),
            };
        } else {
            app.current_screen = Screen::HomeScreen;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    fn make_screen(focused_field: u8, insert_mode: bool) -> Screen {
        Screen::EditMedication {
            id: "test-id".into(),
            name: "Aspirin".into(),
            amount_mg: "100".into(),
            selected_frequency: 0,
            scheduled_time: vec!["08:00".into()],
            scheduled_idx: 0,
            focused_field,
            insert_mode,
        }
    }

    fn press(app: &mut App, code: KeyCode) {
        EditMedicationHandler.handle(app, KeyEvent::new(code, KeyModifiers::NONE));
    }

    fn new_app() -> App {
        App::new(crate::presentation::tui::app_services::AppServices::fake())
    }

    // --- Esc ---

    #[test]
    fn handle_esc_in_insert_mode_exits_insert_mode() {
        let mut app = new_app();
        app.current_screen = make_screen(0, true);

        press(&mut app, KeyCode::Esc);

        assert!(matches!(
            app.current_screen,
            Screen::EditMedication {
                insert_mode: false,
                ..
            }
        ));
    }

    // --- insert mode entry ---

    #[test]
    fn handle_i_in_normal_mode_enters_insert_mode() {
        let mut app = new_app();
        app.current_screen = make_screen(0, false);

        press(&mut app, KeyCode::Char('i'));

        assert!(matches!(
            app.current_screen,
            Screen::EditMedication {
                insert_mode: true,
                ..
            }
        ));
    }

    // --- navigation ---

    #[test]
    fn handle_j_moves_focus_to_next_field() {
        let mut app = new_app();
        app.current_screen = make_screen(0, false);

        press(&mut app, KeyCode::Char('j'));

        assert!(matches!(
            app.current_screen,
            Screen::EditMedication {
                focused_field: 1,
                ..
            }
        ));
    }

    #[test]
    fn handle_k_moves_focus_to_previous_field() {
        let mut app = new_app();
        app.current_screen = make_screen(1, false);

        press(&mut app, KeyCode::Char('k'));

        assert!(matches!(
            app.current_screen,
            Screen::EditMedication {
                focused_field: 0,
                ..
            }
        ));
    }

    #[test]
    fn handle_down_arrow_moves_focus_to_next_field() {
        let mut app = new_app();
        app.current_screen = make_screen(0, false);

        press(&mut app, KeyCode::Down);

        assert!(matches!(
            app.current_screen,
            Screen::EditMedication {
                focused_field: 1,
                ..
            }
        ));
    }

    // --- frequency navigation ---

    #[test]
    fn handle_l_on_frequency_field_advances_frequency() {
        let mut app = new_app();
        app.current_screen = Screen::EditMedication {
            id: "id".into(),
            name: "A".into(),
            amount_mg: "100".into(),
            selected_frequency: 0,
            scheduled_time: vec!["08:00".into()],
            scheduled_idx: 0,
            focused_field: 2,
            insert_mode: false,
        };

        press(&mut app, KeyCode::Char('l'));

        assert!(matches!(
            app.current_screen,
            Screen::EditMedication {
                selected_frequency: 1,
                ..
            }
        ));
    }

    // --- custom slot management ---

    #[test]
    fn handle_j_on_custom_last_slot_appends_new_slot() {
        let mut app = new_app();
        app.current_screen = Screen::EditMedication {
            id: "id".into(),
            name: "A".into(),
            amount_mg: "100".into(),
            selected_frequency: 3,
            scheduled_time: vec!["08:00".into()],
            scheduled_idx: 0,
            focused_field: 3,
            insert_mode: false,
        };

        press(&mut app, KeyCode::Char('j'));

        if let Screen::EditMedication {
            scheduled_time,
            scheduled_idx,
            ..
        } = &app.current_screen
        {
            assert_eq!(scheduled_time.len(), 2);
            assert_eq!(*scheduled_idx, 1);
        } else {
            panic!("unexpected screen");
        }
    }

    #[test]
    fn handle_d_on_custom_removes_slot() {
        let mut app = new_app();
        app.current_screen = Screen::EditMedication {
            id: "id".into(),
            name: "A".into(),
            amount_mg: "100".into(),
            selected_frequency: 3,
            scheduled_time: vec!["08:00".into(), "12:00".into()],
            scheduled_idx: 0,
            focused_field: 3,
            insert_mode: false,
        };

        press(&mut app, KeyCode::Char('d'));

        if let Screen::EditMedication { scheduled_time, .. } = &app.current_screen {
            assert_eq!(scheduled_time.len(), 1);
        } else {
            panic!("unexpected screen");
        }
    }

    // --- text input ---

    #[test]
    fn handle_char_in_insert_mode_appends_to_name_field() {
        let mut app = new_app();
        app.current_screen = make_screen(0, true);

        press(&mut app, KeyCode::Char('X'));

        if let Screen::EditMedication { name, .. } = &app.current_screen {
            assert_eq!(name, "AspirinX");
        } else {
            panic!("unexpected screen");
        }
    }

    #[test]
    fn handle_backspace_in_insert_mode_removes_last_char_from_name() {
        let mut app = new_app();
        app.current_screen = make_screen(0, true);

        press(&mut app, KeyCode::Backspace);

        if let Screen::EditMedication { name, .. } = &app.current_screen {
            assert_eq!(name, "Aspiri");
        } else {
            panic!("unexpected screen");
        }
    }

    /// Verify trait-object dispatch compiles and works.
    #[test]
    fn handle_dispatches_correctly_through_trait_object() {
        use crate::presentation::tui::handlers::port::Handler;
        let mut app = new_app();
        app.current_screen = make_screen(0, true);
        let mut handler: Box<dyn Handler> = Box::new(EditMedicationHandler);
        handler.handle(&mut app, KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        assert!(matches!(
            app.current_screen,
            Screen::EditMedication {
                insert_mode: false,
                ..
            }
        ));
    }

    // --- Esc normal mode: change detection ---

    fn new_app_with_tempdir() -> (App, tempfile::TempDir) {
        use crate::presentation::tui::app_services::AppServices;
        let dir = tempfile::tempdir().unwrap();
        let container = crate::infrastructure::container::Container::new_with_paths(
            dir.path().join("meds.json"),
            dir.path().join("records.json"),
            dir.path().join("settings.json"),
        );
        let services = AppServices {
            list_all_medications: container.list_all_medications_service.clone(),
            create_medication: container.create_medication_service.clone(),
            edit_medication: container.edit_medication_service.clone(),
            delete_medication: container.delete_medication_service.clone(),
            get_medication: container.get_medication_service.clone(),
            list_dose_records: container.list_dose_records_service.clone(),
            mark_dose_taken: container.mark_dose_taken_service.clone(),
            settings: container.settings_service.clone(),
        };
        (App::new(services), dir)
    }

    fn save_medication(app: &App) -> String {
        use crate::application::dtos::requests::CreateMedicationRequest;
        let req = CreateMedicationRequest::new("Aspirin", 100, vec![(8, 0)], "OnceDaily");
        app.services.create_medication.execute(req).unwrap().id
    }

    #[test]
    fn handle_esc_in_normal_mode_without_changes_goes_to_home_screen() {
        let (mut app, _dir) = new_app_with_tempdir();
        let id = save_medication(&app);
        app.current_screen = Screen::EditMedication {
            id: id.clone(),
            name: "Aspirin".into(),
            amount_mg: "100".into(),
            selected_frequency: 0,
            scheduled_time: vec!["08:00".into()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: false,
        };

        press(&mut app, KeyCode::Esc);

        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn handle_esc_in_normal_mode_with_changes_shows_confirm_cancel() {
        let (mut app, _dir) = new_app_with_tempdir();
        let id = save_medication(&app);
        app.current_screen = Screen::EditMedication {
            id: id.clone(),
            name: "Changed".into(),
            amount_mg: "100".into(),
            selected_frequency: 0,
            scheduled_time: vec!["08:00".into()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: false,
        };

        press(&mut app, KeyCode::Esc);

        assert!(matches!(app.current_screen, Screen::ConfirmCancel { .. }));
    }

    #[test]
    fn handle_on_wrong_screen_returns_continue() {
        let mut app = new_app();
        app.current_screen = Screen::HomeScreen;

        let result = EditMedicationHandler
            .handle(&mut app, KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

        assert!(matches!(result, HandlerResult::Continue));
        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn handle_h_in_normal_mode_navigates_left() {
        let mut app = new_app();
        // focused_field=2 with frequency=1 → navigate_left decrements frequency
        app.current_screen = Screen::EditMedication {
            id: "test-id".into(),
            name: "Aspirin".into(),
            amount_mg: "100".into(),
            selected_frequency: 1,
            scheduled_time: vec!["08:00".into()],
            scheduled_idx: 0,
            focused_field: 2,
            insert_mode: false,
        };

        press(&mut app, KeyCode::Char('h'));

        assert!(matches!(
            app.current_screen,
            Screen::EditMedication {
                selected_frequency: 0,
                focused_field: 2,
                ..
            }
        ));
    }

    #[test]
    fn handle_enter_with_invalid_amount_shows_validation_error() {
        let mut app = new_app();
        app.current_screen = Screen::EditMedication {
            id: "test-id".into(),
            name: "Aspirin".into(),
            amount_mg: "not-a-number".into(),
            selected_frequency: 0,
            scheduled_time: vec!["08:00".into()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: false,
        };

        press(&mut app, KeyCode::Enter);

        assert!(matches!(app.current_screen, Screen::ValidationError { .. }));
    }

    #[test]
    fn handle_enter_with_invalid_time_slot_preserves_edit_screen() {
        // parse_slots fails → ValidationError is set then overwritten by set_screen
        let mut app = new_app();
        app.current_screen = Screen::EditMedication {
            id: "test-id".into(),
            name: "Aspirin".into(),
            amount_mg: "100".into(),
            selected_frequency: 0,
            scheduled_time: vec!["not-a-time".into()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: false,
        };

        press(&mut app, KeyCode::Enter);

        assert!(matches!(app.current_screen, Screen::EditMedication { .. }));
    }

    #[test]
    fn handle_enter_with_wrong_slot_count_preserves_edit_screen() {
        // TwiceDaily (selected_frequency=1) with only 1 slot → slot count mismatch
        let mut app = new_app();
        app.current_screen = Screen::EditMedication {
            id: "test-id".into(),
            name: "Aspirin".into(),
            amount_mg: "100".into(),
            selected_frequency: 1,
            scheduled_time: vec!["08:00".into()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: false,
        };

        press(&mut app, KeyCode::Enter);

        assert!(matches!(app.current_screen, Screen::EditMedication { .. }));
    }
}
