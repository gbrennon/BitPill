use crate::presentation::tui::{
    app::App,
    handlers::{
        medication_form_navigation::{
            NavigationState, navigate_down, navigate_left, navigate_right, navigate_up,
            remove_custom_slot,
        },
        port::{Handler, HandlerResult},
        time_slot_parser::{frequency_str, parse_slots},
    },
    input::Key,
    presenters::validation_error_presenter::ValidationErrorPresenter,
    screen::Screen,
};

fn extract_error_messages(e: &crate::application::errors::ApplicationError) -> Vec<String> {
    match e {
        crate::application::errors::ApplicationError::MultipleDomainErrors { errors } => {
            errors.iter().map(|err| err.to_string()).collect()
        }
        _ => vec![e.to_string()],
    }
}

pub struct CreateMedicationHandler;

impl Default for CreateMedicationHandler {
    fn default() -> Self {
        CreateMedicationHandler
    }
}

impl Handler for CreateMedicationHandler {
    fn handle(&mut self, app: &mut App, key: Key) -> HandlerResult {
        let screen = match &app.current_screen {
            Screen::CreateMedication { .. } => app.current_screen.clone(),
            _ => return HandlerResult::Continue,
        };

        let Screen::CreateMedication {
            name,
            amount_mg,
            selected_frequency,
            scheduled_time,
            scheduled_idx,
            focused_field,
            insert_mode,
        } = screen
        else {
            return HandlerResult::Continue;
        };

        let vim_enabled = app.is_vim_mode();
        let nav = NavigationState {
            focused_field,
            scheduled_time,
            scheduled_idx,
        };

        if !vim_enabled {
            match key {
                Key::Char('n') => {
                    let new_nav = navigate_down(nav, selected_frequency);
                    update_screen(
                        app,
                        name,
                        amount_mg,
                        selected_frequency,
                        new_nav,
                        insert_mode,
                    );
                    return HandlerResult::Continue;
                }
                Key::Char('p') => {
                    let new_nav = navigate_up(nav);
                    update_screen(
                        app,
                        name,
                        amount_mg,
                        selected_frequency,
                        new_nav,
                        insert_mode,
                    );
                    return HandlerResult::Continue;
                }
                Key::Char('f') => {
                    let (new_freq, new_nav) = navigate_right(nav, selected_frequency);
                    update_screen(app, name, amount_mg, new_freq, new_nav, insert_mode);
                    return HandlerResult::Continue;
                }
                Key::Char('b') => {
                    let (new_freq, new_nav) = navigate_left(nav, selected_frequency);
                    update_screen(app, name, amount_mg, new_freq, new_nav, insert_mode);
                    return HandlerResult::Continue;
                }
                Key::Char('j') | Key::Char('k') | Key::Char('h') | Key::Char('l') => {
                    return HandlerResult::Continue;
                }
                Key::Char(c) => {
                    let mut new_name = name;
                    let mut new_amount = amount_mg;
                    let mut new_scheduled_time = nav.scheduled_time;
                    let focused = nav.focused_field;
                    let scheduled_idx = nav.scheduled_idx;
                    match focused {
                        0 => new_name.push(c),
                        1 => new_amount.push(c),
                        3 => {
                            while new_scheduled_time.len() <= scheduled_idx {
                                new_scheduled_time.push(String::new());
                            }
                            new_scheduled_time[scheduled_idx].push(c);
                        }
                        _ => {}
                    }
                    let new_nav = NavigationState {
                        focused_field: focused,
                        scheduled_time: new_scheduled_time,
                        scheduled_idx,
                    };
                    update_screen(
                        app,
                        new_name,
                        new_amount,
                        selected_frequency,
                        new_nav,
                        insert_mode,
                    );
                    return HandlerResult::Continue;
                }
                Key::Backspace => {
                    let mut new_name = name;
                    let mut new_amount = amount_mg;
                    let mut new_scheduled_time = nav.scheduled_time;
                    let focused = nav.focused_field;
                    let scheduled_idx = nav.scheduled_idx;
                    match focused {
                        0 => {
                            new_name.pop();
                        }
                        1 => {
                            new_amount.pop();
                        }
                        3 if new_scheduled_time.len() > scheduled_idx => {
                            new_scheduled_time[scheduled_idx].pop();
                        }
                        _ => {}
                    }
                    let new_nav = NavigationState {
                        focused_field: focused,
                        scheduled_time: new_scheduled_time,
                        scheduled_idx,
                    };
                    update_screen(
                        app,
                        new_name,
                        new_amount,
                        selected_frequency,
                        new_nav,
                        insert_mode,
                    );
                    return HandlerResult::Continue;
                }
                Key::Enter => {
                    return submit_form(
                        app,
                        name,
                        amount_mg,
                        selected_frequency,
                        nav.scheduled_time,
                    );
                }
                _ => {}
            }
            return HandlerResult::Continue;
        }

        if !insert_mode {
            match key {
                Key::Esc => {
                    app.current_screen = Screen::ConfirmCancel {
                        previous: Box::new(app.current_screen.clone()),
                    };
                }
                Key::Tab | Key::Right | Key::Char('l') => {
                    let (new_freq, new_nav) = navigate_right(nav, selected_frequency);
                    update_screen(app, name, amount_mg, new_freq, new_nav, false);
                }
                Key::Char('j') | Key::Down => {
                    let new_nav = navigate_down(nav, selected_frequency);
                    update_screen(app, name, amount_mg, selected_frequency, new_nav, false);
                }
                Key::Char('h') | Key::Left => {
                    let (new_freq, new_nav) = navigate_left(nav, selected_frequency);
                    update_screen(app, name, amount_mg, new_freq, new_nav, false);
                }
                Key::Char('k') | Key::Up => {
                    let new_nav = navigate_up(nav);
                    update_screen(app, name, amount_mg, selected_frequency, new_nav, false);
                }
                Key::Char('d')
                    if nav.focused_field == 3
                        && selected_frequency == 3
                        && nav.scheduled_time.len() > 1 =>
                {
                    let (new_slots, new_idx) =
                        remove_custom_slot(nav.scheduled_time, nav.scheduled_idx);
                    let new_nav = NavigationState {
                        focused_field: nav.focused_field,
                        scheduled_time: new_slots,
                        scheduled_idx: new_idx,
                    };
                    update_screen(app, name, amount_mg, selected_frequency, new_nav, false);
                }
                Key::Enter => {
                    return submit_form(
                        app,
                        name,
                        amount_mg,
                        selected_frequency,
                        nav.scheduled_time,
                    );
                }
                Key::Char('i') => {
                    update_screen(app, name, amount_mg, selected_frequency, nav, true);
                }
                _ => {}
            }
            return HandlerResult::Continue;
        }

        match key {
            Key::Esc => {
                update_screen(app, name, amount_mg, selected_frequency, nav, false);
            }
            Key::Backspace => {
                let mut new_name = name;
                let mut new_amount = amount_mg;
                let mut new_scheduled_time = nav.scheduled_time;
                let focused = nav.focused_field;
                let scheduled_idx = nav.scheduled_idx;
                match focused {
                    0 => {
                        new_name.pop();
                    }
                    1 => {
                        new_amount.pop();
                    }
                    3 if new_scheduled_time.len() > scheduled_idx => {
                        new_scheduled_time[scheduled_idx].pop();
                    }
                    _ => {}
                }
                let new_nav = NavigationState {
                    focused_field: focused,
                    scheduled_time: new_scheduled_time,
                    scheduled_idx,
                };
                update_screen(app, new_name, new_amount, selected_frequency, new_nav, true);
            }
            Key::Enter => {
                return submit_form(app, name, amount_mg, selected_frequency, nav.scheduled_time);
            }
            Key::Char(c) => {
                let mut new_name = name;
                let mut new_amount = amount_mg;
                let mut new_scheduled_time = nav.scheduled_time;
                let focused = nav.focused_field;
                let scheduled_idx = nav.scheduled_idx;
                match focused {
                    0 => new_name.push(c),
                    1 => new_amount.push(c),
                    3 => {
                        while new_scheduled_time.len() <= scheduled_idx {
                            new_scheduled_time.push(String::new());
                        }
                        new_scheduled_time[scheduled_idx].push(c);
                    }
                    _ => {}
                }
                let new_nav = NavigationState {
                    focused_field: focused,
                    scheduled_time: new_scheduled_time,
                    scheduled_idx,
                };
                update_screen(app, new_name, new_amount, selected_frequency, new_nav, true);
            }
            _ => {}
        }
        HandlerResult::Continue
    }
}

fn update_screen(
    app: &mut App,
    name: String,
    amount_mg: String,
    selected_frequency: usize,
    nav: NavigationState,
    insert_mode: bool,
) {
    app.current_screen = Screen::CreateMedication {
        name,
        amount_mg,
        selected_frequency,
        scheduled_time: nav.scheduled_time,
        scheduled_idx: nav.scheduled_idx,
        focused_field: nav.focused_field,
        insert_mode,
    };
}

fn submit_form(
    app: &mut App,
    name: String,
    amount_mg: String,
    selected_frequency: usize,
    scheduled_time: Vec<String>,
) -> HandlerResult {
    let presenter = ValidationErrorPresenter;
    let parsed_amount: Result<u32, _> = amount_mg.trim().parse();
    if parsed_amount.is_err() {
        presenter.present(
            app,
            vec!["Invalid dosage: please enter a number".to_string()],
        );
        return HandlerResult::Continue;
    }
    let amount = parsed_amount.unwrap();

    let parsed_slots = parse_slots(&scheduled_time);
    let parsed_times = match parsed_slots {
        Ok(p) => p.times,
        Err(e) => {
            presenter.present(app, vec![e.to_string()]);
            return HandlerResult::Continue;
        }
    };

    let request = crate::application::dtos::requests::CreateMedicationRequest::new(
        name,
        amount,
        parsed_times,
        frequency_str(selected_frequency).to_string(),
    );

    match app.services.create_medication.execute(request) {
        Ok(_) => {
            app.load_medications();
            app.selected_index = 0;
            app.set_status("Medication created successfully", 3000);
            app.current_screen = Screen::HomeScreen;
        }
        Err(e) => {
            let messages = extract_error_messages(&e);
            presenter.present(app, messages);
        }
    }
    HandlerResult::Continue
}

#[cfg(test)]
mod tests {
    use crossterm::event::KeyCode;

    use super::*;
    use crate::presentation::tui::handlers::port::Handler;

    fn make_screen(insert_mode: bool) -> Screen {
        Screen::CreateMedication {
            name: "A".into(),
            amount_mg: "100".into(),
            selected_frequency: 1,
            scheduled_time: vec!["08:00".into()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode,
        }
    }

    fn press(app: &mut App, code: KeyCode) {
        CreateMedicationHandler.handle(app, crate::presentation::tui::input::from_code(code));
    }

    fn new_app() -> App {
        App::new(crate::presentation::tui::app_services::AppServices::fake())
    }

    #[test]
    fn handle_esc_in_normal_mode_opens_confirm_cancel() {
        let mut app = new_app();
        app.current_screen = make_screen(false);

        press(&mut app, KeyCode::Esc);

        assert!(matches!(app.current_screen, Screen::ConfirmCancel { .. }));
    }

    #[test]
    fn handle_esc_in_insert_mode_exits_insert_mode() {
        let mut app = new_app();
        app.current_screen = make_screen(true);

        press(&mut app, KeyCode::Esc);

        assert!(matches!(
            app.current_screen,
            Screen::CreateMedication {
                insert_mode: false,
                ..
            }
        ));
    }

    #[test]
    fn handle_i_in_normal_mode_enters_insert_mode() {
        let mut app = new_app();
        app.current_screen = make_screen(false);

        press(&mut app, KeyCode::Char('i'));

        assert!(matches!(
            app.current_screen,
            Screen::CreateMedication {
                insert_mode: true,
                ..
            }
        ));
    }

    #[test]
    fn handle_j_moves_focus_to_next_field() {
        let mut app = new_app();
        app.current_screen = make_screen(false);

        press(&mut app, KeyCode::Char('j'));

        assert!(matches!(
            app.current_screen,
            Screen::CreateMedication {
                focused_field: 1,
                ..
            }
        ));
    }

    #[test]
    fn handle_k_moves_focus_to_previous_field() {
        let mut app = new_app();
        app.current_screen = make_screen(false);
        if let Screen::CreateMedication { focused_field, .. } = &mut app.current_screen {
            *focused_field = 1;
        }

        press(&mut app, KeyCode::Char('k'));

        assert!(matches!(
            app.current_screen,
            Screen::CreateMedication {
                focused_field: 0,
                ..
            }
        ));
    }

    #[test]
    fn handle_char_in_insert_mode_appends_to_name_field() {
        let mut app = new_app();
        app.current_screen = make_screen(true);

        press(&mut app, KeyCode::Char('X'));

        if let Screen::CreateMedication { name, .. } = &app.current_screen {
            assert_eq!(name, "AX");
        } else {
            panic!("unexpected screen");
        }
    }

    #[test]
    fn handle_backspace_in_insert_mode_removes_last_char_from_name() {
        let mut app = new_app();
        app.current_screen = make_screen(true);

        press(&mut app, KeyCode::Backspace);

        if let Screen::CreateMedication { name, .. } = &app.current_screen {
            assert_eq!(name, "");
        } else {
            panic!("unexpected screen");
        }
    }

    #[test]
    fn handle_dispatches_correctly_through_trait_object() {
        let mut app = new_app();
        app.current_screen = make_screen(false);
        let mut handler: Box<dyn Handler> = Box::new(CreateMedicationHandler);
        handler.handle(
            &mut app,
            crate::presentation::tui::input::from_code(KeyCode::Esc),
        );
        assert!(matches!(app.current_screen, Screen::ConfirmCancel { .. }));
    }

    #[test]
    fn handle_on_wrong_screen_returns_continue() {
        let mut app = new_app();
        app.current_screen = Screen::HomeScreen;

        let result = CreateMedicationHandler.handle(
            &mut app,
            crate::presentation::tui::input::from_code(KeyCode::Enter),
        );

        assert!(matches!(result, HandlerResult::Continue));
        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn handle_l_on_frequency_field_advances_frequency() {
        let mut app = new_app();
        app.current_screen = Screen::CreateMedication {
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
            Screen::CreateMedication {
                selected_frequency: 1,
                ..
            }
        ));
    }

    #[test]
    fn handle_j_on_custom_last_slot_appends_new_slot() {
        let mut app = new_app();
        app.current_screen = Screen::CreateMedication {
            name: "A".into(),
            amount_mg: "100".into(),
            selected_frequency: 3,
            scheduled_time: vec!["08:00".into()],
            scheduled_idx: 0,
            focused_field: 3,
            insert_mode: false,
        };

        press(&mut app, KeyCode::Char('j'));

        if let Screen::CreateMedication {
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
        app.current_screen = Screen::CreateMedication {
            name: "A".into(),
            amount_mg: "100".into(),
            selected_frequency: 3,
            scheduled_time: vec!["08:00".into(), "12:00".into()],
            scheduled_idx: 0,
            focused_field: 3,
            insert_mode: false,
        };

        press(&mut app, KeyCode::Char('d'));

        if let Screen::CreateMedication { scheduled_time, .. } = &app.current_screen {
            assert_eq!(scheduled_time.len(), 1);
        } else {
            panic!("unexpected screen");
        }
    }

    #[test]
    fn handle_down_arrow_moves_focus_to_next_field() {
        let mut app = new_app();
        app.current_screen = make_screen(false);

        press(&mut app, KeyCode::Down);

        assert!(matches!(
            app.current_screen,
            Screen::CreateMedication {
                focused_field: 1,
                ..
            }
        ));
    }

    #[test]
    fn handle_h_in_normal_mode_navigates_left() {
        let mut app = new_app();
        app.current_screen = Screen::CreateMedication {
            name: "A".into(),
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
            Screen::CreateMedication {
                selected_frequency: 0,
                focused_field: 2,
                ..
            }
        ));
    }
}
