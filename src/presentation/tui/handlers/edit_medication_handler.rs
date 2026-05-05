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

pub struct EditMedicationHandler;

impl Default for EditMedicationHandler {
    fn default() -> Self {
        EditMedicationHandler
    }
}

impl Handler for EditMedicationHandler {
    fn handle(&mut self, app: &mut App, key: Key) -> HandlerResult {
        let screen = match &app.current_screen {
            Screen::EditMedication { .. } => app.current_screen.clone(),
            _ => return HandlerResult::Continue,
        };

        let Screen::EditMedication {
            id,
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
                        id,
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
                        id,
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
                    update_screen(app, id, name, amount_mg, new_freq, new_nav, insert_mode);
                    return HandlerResult::Continue;
                }
                Key::Char('b') => {
                    let (new_freq, new_nav) = navigate_left(nav, selected_frequency);
                    update_screen(app, id, name, amount_mg, new_freq, new_nav, insert_mode);
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
                        id,
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
                        id,
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
                        id,
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
                    Self::handle_esc_normal_mode(
                        app,
                        &id,
                        &name,
                        &amount_mg,
                        selected_frequency,
                        &nav.scheduled_time,
                    );
                }
                Key::Tab | Key::Right | Key::Char('l') => {
                    let (new_freq, new_nav) = navigate_right(nav, selected_frequency);
                    update_screen(app, id, name, amount_mg, new_freq, new_nav, false);
                }
                Key::Char('j') | Key::Down => {
                    let new_nav = navigate_down(nav, selected_frequency);
                    update_screen(app, id, name, amount_mg, selected_frequency, new_nav, false);
                }
                Key::Char('h') | Key::Left => {
                    let (new_freq, new_nav) = navigate_left(nav, selected_frequency);
                    update_screen(app, id, name, amount_mg, new_freq, new_nav, false);
                }
                Key::Char('k') | Key::Up => {
                    let new_nav = navigate_up(nav);
                    update_screen(app, id, name, amount_mg, selected_frequency, new_nav, false);
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
                    update_screen(app, id, name, amount_mg, selected_frequency, new_nav, false);
                }
                Key::Enter => {
                    return submit_form(
                        app,
                        id,
                        name,
                        amount_mg,
                        selected_frequency,
                        nav.scheduled_time,
                    );
                }
                Key::Char('i') => {
                    update_screen(app, id, name, amount_mg, selected_frequency, nav, true);
                }
                _ => {}
            }
            return HandlerResult::Continue;
        }

        match key {
            Key::Esc => {
                update_screen(app, id, name, amount_mg, selected_frequency, nav, false);
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
                    id,
                    new_name,
                    new_amount,
                    selected_frequency,
                    new_nav,
                    true,
                );
            }
            Key::Enter => {
                return submit_form(
                    app,
                    id,
                    name,
                    amount_mg,
                    selected_frequency,
                    nav.scheduled_time,
                );
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
                    id,
                    new_name,
                    new_amount,
                    selected_frequency,
                    new_nav,
                    true,
                );
            }
            _ => {}
        }
        HandlerResult::Continue
    }
}

impl EditMedicationHandler {
    fn handle_esc_normal_mode(
        app: &mut App,
        id: &str,
        name: &str,
        amount_mg: &str,
        selected_frequency: usize,
        scheduled_time: &[String],
    ) {
        use crate::application::{
            dtos::requests::GetMedicationRequest,
            ports::inbound::get_medication_port::GetMedicationPort,
        };

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

fn update_screen(
    app: &mut App,
    id: String,
    name: String,
    amount_mg: String,
    selected_frequency: usize,
    nav: NavigationState,
    insert_mode: bool,
) {
    app.current_screen = Screen::EditMedication {
        id,
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
    id: String,
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

    let request = crate::application::dtos::requests::UpdateMedicationRequest::new(
        id,
        name,
        amount,
        parsed_times,
        frequency_str(selected_frequency).to_string(),
    );

    match app.services.update_medication.execute(request) {
        Ok(_) => {
            app.load_medications();
            app.set_status("Medication updated successfully", 3000);
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
    use crate::{application::dtos::responses::MedicationDto, presentation::tui::input::Key};

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

    struct FakeUpdateOk;
    impl crate::application::ports::inbound::update_medication_port::UpdateMedicationPort
        for FakeUpdateOk
    {
        fn execute(
            &self,
            _: crate::application::dtos::requests::UpdateMedicationRequest,
        ) -> Result<
            crate::application::dtos::responses::UpdateMedicationResponse,
            crate::application::errors::ApplicationError,
        > {
            Ok(crate::application::dtos::responses::UpdateMedicationResponse { id: "ok".into() })
        }
    }

    struct FakeUpdateErr;
    impl crate::application::ports::inbound::update_medication_port::UpdateMedicationPort
        for FakeUpdateErr
    {
        fn execute(
            &self,
            _: crate::application::dtos::requests::UpdateMedicationRequest,
        ) -> Result<
            crate::application::dtos::responses::UpdateMedicationResponse,
            crate::application::errors::ApplicationError,
        > {
            Err(crate::application::errors::ApplicationError::NotFound(
                crate::application::errors::NotFoundError,
            ))
        }
    }

    struct FakeGetOk;
    impl crate::application::ports::inbound::get_medication_port::GetMedicationPort for FakeGetOk {
        fn execute(
            &self,
            _: crate::application::dtos::requests::GetMedicationRequest,
        ) -> Result<
            crate::application::dtos::responses::GetMedicationResponse,
            crate::application::errors::ApplicationError,
        > {
            Ok(crate::application::dtos::responses::GetMedicationResponse {
                medication: MedicationDto {
                    id: "e1".into(),
                    name: "Original".into(),
                    amount_mg: 100,
                    scheduled_time: vec![(8, 0)],
                    dose_frequency: "OnceDaily".into(),
                    taken_today: 0,
                    scheduled_today: 1,
                },
            })
        }
    }

    struct FakeGetErr;
    impl crate::application::ports::inbound::get_medication_port::GetMedicationPort for FakeGetErr {
        fn execute(
            &self,
            _: crate::application::dtos::requests::GetMedicationRequest,
        ) -> Result<
            crate::application::dtos::responses::GetMedicationResponse,
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
        app.services.update_medication = Arc::new(FakeUpdateOk);
        app.current_screen = screen;
        app
    }

    fn make_app_vim(screen: Screen) -> App {
        let mut app = App::default();
        app.services.get_settings = Arc::new(FakeSettings("vi"));
        app.services.update_medication = Arc::new(FakeUpdateOk);
        app.current_screen = screen;
        app
    }

    fn edit_screen() -> Screen {
        Screen::EditMedication {
            id: "e1".into(),
            name: String::new(),
            amount_mg: String::new(),
            selected_frequency: 0,
            scheduled_time: vec![String::new()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: false,
        }
    }

    fn edit_screen_with_values(name: &str, amount: &str) -> Screen {
        Screen::EditMedication {
            id: "e1".into(),
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
    fn default_constructs() {
        let _h = EditMedicationHandler::default();
    }

    #[test]
    fn non_edit_screen_returns_continue() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_vim(Screen::HomeScreen);
        let r = h.handle(&mut a, Key::Char('x'));
        assert!(matches!(r, HandlerResult::Continue));
        assert!(matches!(a.current_screen, Screen::HomeScreen));
    }

    // --- Emacs mode ---
    #[test]
    fn emacs_n_navigates() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_emacs(edit_screen());
        h.handle(&mut a, Key::Char('n'));
        assert!(matches!(a.current_screen, Screen::EditMedication { .. }));
    }
    #[test]
    fn emacs_p_navigates() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_emacs(edit_screen());
        h.handle(&mut a, Key::Char('p'));
        assert!(matches!(a.current_screen, Screen::EditMedication { .. }));
    }
    #[test]
    fn emacs_f_navigates() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_emacs(edit_screen());
        h.handle(&mut a, Key::Char('f'));
        assert!(matches!(a.current_screen, Screen::EditMedication { .. }));
    }
    #[test]
    fn emacs_b_navigates() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_emacs(edit_screen());
        h.handle(&mut a, Key::Char('b'));
        assert!(matches!(a.current_screen, Screen::EditMedication { .. }));
    }
    #[test]
    fn emacs_skip_vim_keys() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_emacs(edit_screen());
        for key in [
            Key::Char('j'),
            Key::Char('k'),
            Key::Char('h'),
            Key::Char('l'),
        ] {
            assert!(matches!(h.handle(&mut a, key), HandlerResult::Continue));
        }
    }
    #[test]
    fn emacs_char_focused_name() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_emacs(Screen::EditMedication {
            id: "e1".into(),
            name: String::new(),
            amount_mg: String::new(),
            selected_frequency: 0,
            scheduled_time: vec![String::new()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: false,
        });
        h.handle(&mut a, Key::Char('A'));
        if let Screen::EditMedication { name, .. } = &a.current_screen {
            assert_eq!(name, "A");
        }
    }
    #[test]
    fn emacs_char_focused_amount() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_emacs(Screen::EditMedication {
            id: "e1".into(),
            name: String::new(),
            amount_mg: String::new(),
            selected_frequency: 0,
            scheduled_time: vec![String::new()],
            scheduled_idx: 0,
            focused_field: 1,
            insert_mode: false,
        });
        h.handle(&mut a, Key::Char('5'));
        if let Screen::EditMedication { amount_mg, .. } = &a.current_screen {
            assert_eq!(amount_mg, "5");
        }
    }
    #[test]
    fn emacs_char_scheduled_time() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_emacs(Screen::EditMedication {
            id: "e1".into(),
            name: String::new(),
            amount_mg: String::new(),
            selected_frequency: 0,
            scheduled_time: vec![String::new()],
            scheduled_idx: 0,
            focused_field: 3,
            insert_mode: false,
        });
        h.handle(&mut a, Key::Char('1'));
        if let Screen::EditMedication { scheduled_time, .. } = &a.current_screen {
            assert_eq!(scheduled_time[0], "1");
        }
    }
    #[test]
    fn emacs_backspace_name() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_emacs(Screen::EditMedication {
            id: "e1".into(),
            name: "AB".into(),
            amount_mg: String::new(),
            selected_frequency: 0,
            scheduled_time: vec![String::new()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: false,
        });
        h.handle(&mut a, Key::Backspace);
        if let Screen::EditMedication { name, .. } = &a.current_screen {
            assert_eq!(name, "A");
        }
    }
    #[test]
    fn emacs_backspace_amount() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_emacs(Screen::EditMedication {
            id: "e1".into(),
            name: String::new(),
            amount_mg: "12".into(),
            selected_frequency: 0,
            scheduled_time: vec![String::new()],
            scheduled_idx: 0,
            focused_field: 1,
            insert_mode: false,
        });
        h.handle(&mut a, Key::Backspace);
        if let Screen::EditMedication { amount_mg, .. } = &a.current_screen {
            assert_eq!(amount_mg, "1");
        }
    }
    #[test]
    fn emacs_backspace_scheduled_time() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_emacs(Screen::EditMedication {
            id: "e1".into(),
            name: String::new(),
            amount_mg: String::new(),
            selected_frequency: 0,
            scheduled_time: vec!["12:3".into()],
            scheduled_idx: 0,
            focused_field: 3,
            insert_mode: false,
        });
        h.handle(&mut a, Key::Backspace);
        if let Screen::EditMedication { scheduled_time, .. } = &a.current_screen {
            assert_eq!(scheduled_time[0], "12:");
        }
    }
    #[test]
    fn emacs_enter_submit() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_emacs(edit_screen_with_values("Test", "100"));
        h.handle(&mut a, Key::Enter);
        assert!(matches!(a.current_screen, Screen::HomeScreen));
    }

    // --- Vim normal mode ---
    #[test]
    fn vim_normal_esc_unchanged_returns_home() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_vim(Screen::EditMedication {
            id: "e1".into(),
            name: "Original".into(),
            amount_mg: "100".into(),
            selected_frequency: 0,
            scheduled_time: vec!["08:00".into()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: false,
        });
        a.services.get_medication = Arc::new(FakeGetOk);
        h.handle(&mut a, Key::Esc);
        assert!(matches!(a.current_screen, Screen::HomeScreen));
    }
    #[test]
    fn vim_normal_esc_changed_shows_confirm_cancel() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_vim(edit_screen_with_values("Changed", "200"));
        a.services.get_medication = Arc::new(FakeGetOk);
        h.handle(&mut a, Key::Esc);
        assert!(matches!(a.current_screen, Screen::ConfirmCancel { .. }));
    }
    #[test]
    fn vim_normal_esc_get_err_shows_confirm_cancel() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_vim(edit_screen_with_values("X", "100"));
        a.services.get_medication = Arc::new(FakeGetErr);
        h.handle(&mut a, Key::Esc);
        assert!(matches!(a.current_screen, Screen::ConfirmCancel { .. }));
    }
    #[test]
    fn vim_normal_tab_navigates() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_vim(edit_screen());
        h.handle(&mut a, Key::Tab);
        assert!(matches!(a.current_screen, Screen::EditMedication { .. }));
    }
    #[test]
    fn vim_normal_right_navigates() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_vim(edit_screen());
        h.handle(&mut a, Key::Right);
        assert!(matches!(a.current_screen, Screen::EditMedication { .. }));
    }
    #[test]
    fn vim_normal_l_navigates() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_vim(edit_screen());
        h.handle(&mut a, Key::Char('l'));
        assert!(matches!(a.current_screen, Screen::EditMedication { .. }));
    }
    #[test]
    fn vim_normal_j_navigates() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_vim(edit_screen());
        h.handle(&mut a, Key::Char('j'));
        assert!(matches!(a.current_screen, Screen::EditMedication { .. }));
    }
    #[test]
    fn vim_normal_down_navigates() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_vim(edit_screen());
        h.handle(&mut a, Key::Down);
        assert!(matches!(a.current_screen, Screen::EditMedication { .. }));
    }
    #[test]
    fn vim_normal_h_navigates() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_vim(edit_screen());
        h.handle(&mut a, Key::Char('h'));
        assert!(matches!(a.current_screen, Screen::EditMedication { .. }));
    }
    #[test]
    fn vim_normal_left_navigates() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_vim(edit_screen());
        h.handle(&mut a, Key::Left);
        assert!(matches!(a.current_screen, Screen::EditMedication { .. }));
    }
    #[test]
    fn vim_normal_k_navigates() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_vim(edit_screen());
        h.handle(&mut a, Key::Char('k'));
        assert!(matches!(a.current_screen, Screen::EditMedication { .. }));
    }
    #[test]
    fn vim_normal_up_navigates() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_vim(edit_screen());
        h.handle(&mut a, Key::Up);
        assert!(matches!(a.current_screen, Screen::EditMedication { .. }));
    }
    #[test]
    fn vim_normal_d_removes_custom_slot() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_vim(Screen::EditMedication {
            id: "e1".into(),
            name: String::new(),
            amount_mg: String::new(),
            selected_frequency: 3,
            scheduled_time: vec!["08:00".into(), "12:00".into()],
            scheduled_idx: 0,
            focused_field: 3,
            insert_mode: false,
        });
        h.handle(&mut a, Key::Char('d'));
        if let Screen::EditMedication { scheduled_time, .. } = &a.current_screen {
            assert_eq!(scheduled_time.len(), 1);
        }
    }
    #[test]
    fn vim_normal_i_enters_insert_mode() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_vim(edit_screen());
        h.handle(&mut a, Key::Char('i'));
        if let Screen::EditMedication { insert_mode, .. } = &a.current_screen {
            assert!(insert_mode);
        }
    }
    #[test]
    fn vim_normal_enter_submit() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_vim(edit_screen_with_values("Test", "100"));
        h.handle(&mut a, Key::Enter);
        assert!(matches!(a.current_screen, Screen::HomeScreen));
    }

    // --- Vim insert mode ---
    #[test]
    fn vim_insert_esc_exits() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_vim(Screen::EditMedication {
            id: "e1".into(),
            name: String::new(),
            amount_mg: String::new(),
            selected_frequency: 0,
            scheduled_time: vec![String::new()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: true,
        });
        h.handle(&mut a, Key::Esc);
        if let Screen::EditMedication { insert_mode, .. } = &a.current_screen {
            assert!(!insert_mode);
        }
    }
    #[test]
    fn vim_insert_backspace_name() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_vim(Screen::EditMedication {
            id: "e1".into(),
            name: "AB".into(),
            amount_mg: String::new(),
            selected_frequency: 0,
            scheduled_time: vec![String::new()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: true,
        });
        h.handle(&mut a, Key::Backspace);
        if let Screen::EditMedication { name, .. } = &a.current_screen {
            assert_eq!(name, "A");
        }
    }
    #[test]
    fn vim_insert_backspace_amount() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_vim(Screen::EditMedication {
            id: "e1".into(),
            name: String::new(),
            amount_mg: "12".into(),
            selected_frequency: 0,
            scheduled_time: vec![String::new()],
            scheduled_idx: 0,
            focused_field: 1,
            insert_mode: true,
        });
        h.handle(&mut a, Key::Backspace);
        if let Screen::EditMedication { amount_mg, .. } = &a.current_screen {
            assert_eq!(amount_mg, "1");
        }
    }
    #[test]
    fn vim_insert_char_name() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_vim(Screen::EditMedication {
            id: "e1".into(),
            name: String::new(),
            amount_mg: String::new(),
            selected_frequency: 0,
            scheduled_time: vec![String::new()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: true,
        });
        h.handle(&mut a, Key::Char('X'));
        if let Screen::EditMedication { name, .. } = &a.current_screen {
            assert_eq!(name, "X");
        }
    }
    #[test]
    fn vim_insert_char_amount() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_vim(Screen::EditMedication {
            id: "e1".into(),
            name: String::new(),
            amount_mg: String::new(),
            selected_frequency: 0,
            scheduled_time: vec![String::new()],
            scheduled_idx: 0,
            focused_field: 1,
            insert_mode: true,
        });
        h.handle(&mut a, Key::Char('9'));
        if let Screen::EditMedication { amount_mg, .. } = &a.current_screen {
            assert_eq!(amount_mg, "9");
        }
    }
    #[test]
    fn vim_insert_char_scheduled_time() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_vim(Screen::EditMedication {
            id: "e1".into(),
            name: String::new(),
            amount_mg: String::new(),
            selected_frequency: 0,
            scheduled_time: vec![String::new()],
            scheduled_idx: 0,
            focused_field: 3,
            insert_mode: true,
        });
        h.handle(&mut a, Key::Char('0'));
        if let Screen::EditMedication { scheduled_time, .. } = &a.current_screen {
            assert_eq!(scheduled_time[0], "0");
        }
    }
    #[test]
    fn vim_insert_enter_submit() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_vim(Screen::EditMedication {
            id: "e1".into(),
            name: "Test".into(),
            amount_mg: "100".into(),
            selected_frequency: 0,
            scheduled_time: vec!["12:00".into()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: true,
        });
        h.handle(&mut a, Key::Enter);
        assert!(matches!(a.current_screen, Screen::HomeScreen));
    }

    // --- Error paths ---
    #[test]
    fn submit_invalid_amount_shows_error() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_vim(Screen::EditMedication {
            id: "e1".into(),
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
    fn submit_invalid_time_shows_error() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_vim(Screen::EditMedication {
            id: "e1".into(),
            name: "Test".into(),
            amount_mg: "100".into(),
            selected_frequency: 0,
            scheduled_time: vec!["bad".into()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: false,
        });
        h.handle(&mut a, Key::Enter);
        assert!(matches!(a.current_screen, Screen::ValidationError { .. }));
    }
    #[test]
    fn submit_service_error_shows_error() {
        let mut h = EditMedicationHandler;
        let mut a = make_app_vim(Screen::EditMedication {
            id: "e1".into(),
            name: "Test".into(),
            amount_mg: "100".into(),
            selected_frequency: 0,
            scheduled_time: vec!["12:00".into()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: false,
        });
        a.services.update_medication = Arc::new(FakeUpdateErr);
        h.handle(&mut a, Key::Enter);
        assert!(matches!(a.current_screen, Screen::ValidationError { .. }));
    }
}
