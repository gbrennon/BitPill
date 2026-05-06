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
    use std::sync::Arc;

    use super::*;
    use crate::presentation::tui::input::Key;

    struct FakeSettings(&'static str);
    impl crate::application::ports::inbound::get_settings_port::GetSettingsPort for FakeSettings {
        fn execute(
            &self,
            _: crate::application::dtos::requests::GetSettingsRequest,
        ) -> Result<
            crate::application::dtos::responses::GetSettingsResponse,
            crate::application::errors::ApplicationError,
        > {
            Ok(crate::application::dtos::responses::GetSettingsResponse {
                navigation_mode: self.0.into(),
            })
        }
    }

    struct FakeCreateOk;
    impl crate::application::ports::inbound::create_medication_port::CreateMedicationPort
        for FakeCreateOk
    {
        fn execute(
            &self,
            _: crate::application::dtos::requests::CreateMedicationRequest,
        ) -> Result<
            crate::application::dtos::responses::CreateMedicationResponse,
            crate::application::errors::ApplicationError,
        > {
            Ok(crate::application::dtos::responses::CreateMedicationResponse { id: "ok".into() })
        }
    }

    struct FakeCreateErr;
    impl crate::application::ports::inbound::create_medication_port::CreateMedicationPort
        for FakeCreateErr
    {
        fn execute(
            &self,
            _: crate::application::dtos::requests::CreateMedicationRequest,
        ) -> Result<
            crate::application::dtos::responses::CreateMedicationResponse,
            crate::application::errors::ApplicationError,
        > {
            Err(crate::application::errors::ApplicationError::NotFound(
                crate::application::errors::NotFoundError,
            ))
        }
    }

    fn make_app_emacs(screen: Screen) -> App {
        let mut app = App::default();
        app.services.get_settings = Arc::new(FakeSettings("emacs"));
        app.current_screen = screen;
        app
    }

    fn make_app_vim(screen: Screen) -> App {
        let mut app = App::default();
        app.services.get_settings = Arc::new(FakeSettings("vi"));
        app.current_screen = screen;
        app
    }

    fn create_screen() -> Screen {
        Screen::CreateMedication {
            name: String::new(),
            amount_mg: String::new(),
            selected_frequency: 0,
            scheduled_time: vec![String::new()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: false,
        }
    }

    fn create_screen_with_values(name: &str, amount: &str) -> Screen {
        Screen::CreateMedication {
            name: name.into(),
            amount_mg: amount.into(),
            selected_frequency: 0,
            scheduled_time: vec!["12:00".into()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: false,
        }
    }

    #[test]
    fn non_create_screen_returns_continue() {
        let mut h = CreateMedicationHandler;
        let mut app = make_app_vim(Screen::HomeScreen);
        let r = h.handle(&mut app, Key::Char('x'));
        assert!(matches!(r, HandlerResult::Continue));
        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }
    #[test]
    fn default_constructs() {
        let _h = CreateMedicationHandler::default();
    }

    // --- Emacs mode navigation ---
    #[test]
    fn emacs_n_navigates_down() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_emacs(create_screen());
        h.handle(&mut a, Key::Char('n'));
        assert!(matches!(a.current_screen, Screen::CreateMedication { .. }));
    }
    #[test]
    fn emacs_p_navigates_up() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_emacs(create_screen());
        h.handle(&mut a, Key::Char('p'));
        assert!(matches!(a.current_screen, Screen::CreateMedication { .. }));
    }
    #[test]
    fn emacs_f_navigates_right() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_emacs(create_screen());
        h.handle(&mut a, Key::Char('f'));
        assert!(matches!(a.current_screen, Screen::CreateMedication { .. }));
    }
    #[test]
    fn emacs_b_navigates_left() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_emacs(create_screen());
        h.handle(&mut a, Key::Char('b'));
        assert!(matches!(a.current_screen, Screen::CreateMedication { .. }));
    }
    #[test]
    fn emacs_skip_vim_keys() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_emacs(create_screen());
        for key in [
            Key::Char('j'),
            Key::Char('k'),
            Key::Char('h'),
            Key::Char('l'),
        ] {
            assert!(matches!(h.handle(&mut a, key), HandlerResult::Continue));
        }
    }

    // --- Emacs char typing ---
    #[test]
    fn emacs_char_focused_name() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_emacs(Screen::CreateMedication {
            name: String::new(),
            amount_mg: String::new(),
            selected_frequency: 0,
            scheduled_time: vec![String::new()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: false,
        });
        h.handle(&mut a, Key::Char('A'));
        if let Screen::CreateMedication { name, .. } = &a.current_screen {
            assert_eq!(name, "A");
        }
    }
    #[test]
    fn emacs_char_focused_amount() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_emacs(Screen::CreateMedication {
            name: String::new(),
            amount_mg: String::new(),
            selected_frequency: 0,
            scheduled_time: vec![String::new()],
            scheduled_idx: 0,
            focused_field: 1,
            insert_mode: false,
        });
        h.handle(&mut a, Key::Char('5'));
        if let Screen::CreateMedication { amount_mg, .. } = &a.current_screen {
            assert_eq!(amount_mg, "5");
        }
    }
    #[test]
    fn emacs_char_focused_scheduled_time() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_emacs(Screen::CreateMedication {
            name: String::new(),
            amount_mg: String::new(),
            selected_frequency: 0,
            scheduled_time: vec![String::new()],
            scheduled_idx: 0,
            focused_field: 3,
            insert_mode: false,
        });
        h.handle(&mut a, Key::Char('1'));
        if let Screen::CreateMedication { scheduled_time, .. } = &a.current_screen {
            assert_eq!(scheduled_time[0], "1");
        }
    }
    #[test]
    fn emacs_char_focused_other_noop() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_emacs(Screen::CreateMedication {
            name: String::new(),
            amount_mg: String::new(),
            selected_frequency: 0,
            scheduled_time: vec![String::new()],
            scheduled_idx: 0,
            focused_field: 2,
            insert_mode: false,
        });
        h.handle(&mut a, Key::Char('x'));
        assert!(matches!(a.current_screen, Screen::CreateMedication { .. }));
    }

    // --- Emacs backspace ---
    #[test]
    fn emacs_backspace_focused_name() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_emacs(Screen::CreateMedication {
            name: "AB".into(),
            amount_mg: String::new(),
            selected_frequency: 0,
            scheduled_time: vec![String::new()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: false,
        });
        h.handle(&mut a, Key::Backspace);
        if let Screen::CreateMedication { name, .. } = &a.current_screen {
            assert_eq!(name, "A");
        }
    }
    #[test]
    fn emacs_backspace_focused_amount() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_emacs(Screen::CreateMedication {
            name: String::new(),
            amount_mg: "12".into(),
            selected_frequency: 0,
            scheduled_time: vec![String::new()],
            scheduled_idx: 0,
            focused_field: 1,
            insert_mode: false,
        });
        h.handle(&mut a, Key::Backspace);
        if let Screen::CreateMedication { amount_mg, .. } = &a.current_screen {
            assert_eq!(amount_mg, "1");
        }
    }
    #[test]
    fn emacs_backspace_focused_scheduled_time() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_emacs(Screen::CreateMedication {
            name: String::new(),
            amount_mg: String::new(),
            selected_frequency: 0,
            scheduled_time: vec!["12:3".into()],
            scheduled_idx: 0,
            focused_field: 3,
            insert_mode: false,
        });
        h.handle(&mut a, Key::Backspace);
        if let Screen::CreateMedication { scheduled_time, .. } = &a.current_screen {
            assert_eq!(scheduled_time[0], "12:");
        }
    }
    #[test]
    fn emacs_enter_submit_form() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_emacs(create_screen_with_values("Test", "100"));
        a.services.create_medication = Arc::new(FakeCreateOk);
        h.handle(&mut a, Key::Enter);
        assert!(matches!(a.current_screen, Screen::HomeScreen));
    }

    // --- Vim normal mode ---
    #[test]
    fn vim_normal_esc_opens_confirm_cancel() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_vim(create_screen());
        h.handle(&mut a, Key::Esc);
        assert!(matches!(a.current_screen, Screen::ConfirmCancel { .. }));
    }
    #[test]
    fn vim_normal_tab_navigates_right() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_vim(create_screen());
        h.handle(&mut a, Key::Tab);
        assert!(matches!(a.current_screen, Screen::CreateMedication { .. }));
    }
    #[test]
    fn vim_normal_right_navigates() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_vim(create_screen());
        h.handle(&mut a, Key::Right);
        assert!(matches!(a.current_screen, Screen::CreateMedication { .. }));
    }
    #[test]
    fn vim_normal_l_navigates_right() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_vim(create_screen());
        h.handle(&mut a, Key::Char('l'));
        assert!(matches!(a.current_screen, Screen::CreateMedication { .. }));
    }
    #[test]
    fn vim_normal_j_navigates_down() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_vim(create_screen());
        h.handle(&mut a, Key::Char('j'));
        assert!(matches!(a.current_screen, Screen::CreateMedication { .. }));
    }
    #[test]
    fn vim_normal_down_navigates() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_vim(create_screen());
        h.handle(&mut a, Key::Down);
        assert!(matches!(a.current_screen, Screen::CreateMedication { .. }));
    }
    #[test]
    fn vim_normal_h_navigates_left() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_vim(create_screen());
        h.handle(&mut a, Key::Char('h'));
        assert!(matches!(a.current_screen, Screen::CreateMedication { .. }));
    }
    #[test]
    fn vim_normal_left_navigates() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_vim(create_screen());
        h.handle(&mut a, Key::Left);
        assert!(matches!(a.current_screen, Screen::CreateMedication { .. }));
    }
    #[test]
    fn vim_normal_k_navigates_up() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_vim(create_screen());
        h.handle(&mut a, Key::Char('k'));
        assert!(matches!(a.current_screen, Screen::CreateMedication { .. }));
    }
    #[test]
    fn vim_normal_up_navigates() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_vim(create_screen());
        h.handle(&mut a, Key::Up);
        assert!(matches!(a.current_screen, Screen::CreateMedication { .. }));
    }
    #[test]
    fn vim_normal_d_removes_custom_slot() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_vim(Screen::CreateMedication {
            name: String::new(),
            amount_mg: String::new(),
            selected_frequency: 3,
            scheduled_time: vec!["08:00".into(), "12:00".into()],
            scheduled_idx: 0,
            focused_field: 3,
            insert_mode: false,
        });
        h.handle(&mut a, Key::Char('d'));
        if let Screen::CreateMedication { scheduled_time, .. } = &a.current_screen {
            assert_eq!(scheduled_time.len(), 1);
        }
    }
    #[test]
    fn vim_normal_d_noop_when_not_custom() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_vim(Screen::CreateMedication {
            name: String::new(),
            amount_mg: String::new(),
            selected_frequency: 0,
            scheduled_time: vec!["08:00".into(), "12:00".into()],
            scheduled_idx: 0,
            focused_field: 3,
            insert_mode: false,
        });
        h.handle(&mut a, Key::Char('d'));
        assert!(matches!(a.current_screen, Screen::CreateMedication { .. }));
    }
    #[test]
    fn vim_normal_i_enters_insert_mode() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_vim(create_screen());
        h.handle(&mut a, Key::Char('i'));
        if let Screen::CreateMedication { insert_mode, .. } = &a.current_screen {
            assert!(insert_mode);
        }
    }
    #[test]
    fn vim_normal_enter_submit_form() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_vim(create_screen_with_values("Test", "100"));
        a.services.create_medication = Arc::new(FakeCreateOk);
        h.handle(&mut a, Key::Enter);
        assert!(matches!(a.current_screen, Screen::HomeScreen));
    }

    // --- Vim insert mode ---
    #[test]
    fn vim_insert_esc_exits() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_vim(Screen::CreateMedication {
            name: String::new(),
            amount_mg: String::new(),
            selected_frequency: 0,
            scheduled_time: vec![String::new()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: true,
        });
        h.handle(&mut a, Key::Esc);
        if let Screen::CreateMedication { insert_mode, .. } = &a.current_screen {
            assert!(!insert_mode);
        }
    }
    #[test]
    fn vim_insert_backspace_name() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_vim(Screen::CreateMedication {
            name: "AB".into(),
            amount_mg: String::new(),
            selected_frequency: 0,
            scheduled_time: vec![String::new()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: true,
        });
        h.handle(&mut a, Key::Backspace);
        if let Screen::CreateMedication { name, .. } = &a.current_screen {
            assert_eq!(name, "A");
        }
    }
    #[test]
    fn vim_insert_backspace_amount() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_vim(Screen::CreateMedication {
            name: String::new(),
            amount_mg: "12".into(),
            selected_frequency: 0,
            scheduled_time: vec![String::new()],
            scheduled_idx: 0,
            focused_field: 1,
            insert_mode: true,
        });
        h.handle(&mut a, Key::Backspace);
        if let Screen::CreateMedication { amount_mg, .. } = &a.current_screen {
            assert_eq!(amount_mg, "1");
        }
    }
    #[test]
    fn vim_insert_backspace_scheduled_time() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_vim(Screen::CreateMedication {
            name: String::new(),
            amount_mg: String::new(),
            selected_frequency: 3,
            scheduled_time: vec!["12:3".into()],
            scheduled_idx: 0,
            focused_field: 3,
            insert_mode: true,
        });
        h.handle(&mut a, Key::Backspace);
        if let Screen::CreateMedication { scheduled_time, .. } = &a.current_screen {
            assert_eq!(scheduled_time[0], "12:");
        }
    }
    #[test]
    fn vim_insert_enter_submit() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_vim(Screen::CreateMedication {
            name: "Test".into(),
            amount_mg: "100".into(),
            selected_frequency: 0,
            scheduled_time: vec!["12:00".into()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: true,
        });
        a.services.create_medication = Arc::new(FakeCreateOk);
        h.handle(&mut a, Key::Enter);
        assert!(matches!(a.current_screen, Screen::HomeScreen));
    }
    #[test]
    fn vim_insert_char_name() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_vim(Screen::CreateMedication {
            name: String::new(),
            amount_mg: String::new(),
            selected_frequency: 0,
            scheduled_time: vec![String::new()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: true,
        });
        h.handle(&mut a, Key::Char('X'));
        if let Screen::CreateMedication { name, .. } = &a.current_screen {
            assert_eq!(name, "X");
        }
    }
    #[test]
    fn vim_insert_char_amount() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_vim(Screen::CreateMedication {
            name: String::new(),
            amount_mg: String::new(),
            selected_frequency: 0,
            scheduled_time: vec![String::new()],
            scheduled_idx: 0,
            focused_field: 1,
            insert_mode: true,
        });
        h.handle(&mut a, Key::Char('9'));
        if let Screen::CreateMedication { amount_mg, .. } = &a.current_screen {
            assert_eq!(amount_mg, "9");
        }
    }
    #[test]
    fn vim_insert_char_scheduled_time() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_vim(Screen::CreateMedication {
            name: String::new(),
            amount_mg: String::new(),
            selected_frequency: 0,
            scheduled_time: vec![String::new()],
            scheduled_idx: 0,
            focused_field: 3,
            insert_mode: true,
        });
        h.handle(&mut a, Key::Char('0'));
        if let Screen::CreateMedication { scheduled_time, .. } = &a.current_screen {
            assert_eq!(scheduled_time[0], "0");
        }
    }

    // --- Error paths ---
    #[test]
    fn submit_form_invalid_amount_shows_validation_error() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_vim(Screen::CreateMedication {
            name: "Test".into(),
            amount_mg: "abc".into(),
            selected_frequency: 0,
            scheduled_time: vec!["12:00".into()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: false,
        });
        h.handle(&mut a, Key::Enter);
        assert!(matches!(a.current_screen, Screen::ValidationError { .. }));
    }
    #[test]
    fn submit_form_invalid_time_slot_shows_validation_error() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_vim(Screen::CreateMedication {
            name: "Test".into(),
            amount_mg: "100".into(),
            selected_frequency: 0,
            scheduled_time: vec!["invalid".into()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: false,
        });
        h.handle(&mut a, Key::Enter);
        assert!(matches!(a.current_screen, Screen::ValidationError { .. }));
    }
    #[test]
    fn submit_form_service_error_shows_validation_error() {
        let mut h = CreateMedicationHandler;
        let mut a = make_app_vim(Screen::CreateMedication {
            name: "Test".into(),
            amount_mg: "100".into(),
            selected_frequency: 0,
            scheduled_time: vec!["12:00".into()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: false,
        });
        a.services.create_medication = Arc::new(FakeCreateErr);
        h.handle(&mut a, Key::Enter);
        assert!(matches!(a.current_screen, Screen::ValidationError { .. }));
    }
    #[test]
    fn extract_error_messages_multiple() {
        use crate::{application::errors::ApplicationError, domain::errors::DomainError};
        let err = ApplicationError::MultipleDomainErrors {
            errors: vec![DomainError::InvalidDosage, DomainError::EmptyMedicationName],
        };
        let msgs = extract_error_messages(&err);
        assert_eq!(msgs.len(), 2);
        assert!(msgs[0].contains("dosage"));
    }
    #[test]
    fn extract_error_messages_single() {
        use crate::application::errors::ApplicationError;
        let err = ApplicationError::NotFound(crate::application::errors::NotFoundError);
        let msgs = extract_error_messages(&err);
        assert_eq!(msgs.len(), 1);
    }
}
