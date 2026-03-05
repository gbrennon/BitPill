use crate::application::ports::create_medication_port::CreateMedicationRequest;
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

pub struct CreateMedicationHandler;

impl Default for CreateMedicationHandler {
    fn default() -> Self {
        CreateMedicationHandler
    }
}

impl Handler for CreateMedicationHandler {
    fn handle(&mut self, app: &mut App, key: KeyEvent) -> HandlerResult {
        let (
            name,
            amount_mg,
            selected_frequency,
            scheduled_time,
            scheduled_idx,
            focused_field,
            insert_mode,
        ) = match &app.current_screen {
            Screen::CreateMedication {
                name,
                amount_mg,
                selected_frequency,
                scheduled_time,
                scheduled_idx,
                focused_field,
                insert_mode,
            } => (
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
                          name: String,
                          amount_mg: String,
                          sel_freq: usize,
                          nav: NavigationState,
                          insert_mode: bool| {
            app.current_screen = Screen::CreateMedication {
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
                    set_screen(app, name, amount_mg, selected_frequency, nav, false);
                } else {
                    app.current_screen = Screen::ConfirmCancel {
                        previous: Box::new(app.current_screen.clone()),
                    };
                }
            }
            crossterm::event::KeyCode::Tab
            | crossterm::event::KeyCode::Right
            | crossterm::event::KeyCode::Char('l')
                if !insert_mode =>
            {
                let (sel, new_nav) = navigate_right(nav, selected_frequency);
                set_screen(app, name, amount_mg, sel, new_nav, insert_mode);
            }
            crossterm::event::KeyCode::Char('j') | crossterm::event::KeyCode::Down
                if !insert_mode =>
            {
                let new_nav = navigate_down(nav, selected_frequency);
                set_screen(
                    app,
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
                set_screen(app, name, amount_mg, sel, new_nav, insert_mode);
            }
            crossterm::event::KeyCode::Char('k') | crossterm::event::KeyCode::Up
                if !insert_mode =>
            {
                let new_nav = navigate_up(nav);
                set_screen(
                    app,
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
                        set_screen(app, name, amount_mg, selected_frequency, nav, insert_mode);
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
                                name,
                                amount_mg,
                                selected_frequency,
                                new_nav,
                                insert_mode,
                            );
                            return HandlerResult::Continue;
                        }

                        let request = CreateMedicationRequest::new(
                            name,
                            parsed_amount,
                            parsed.times,
                            frequency_str(selected_frequency),
                        );
                        use crate::application::ports::inbound::create_medication_port::CreateMedicationPort;
                        match app.container.create_medication_service.execute(request) {
                            Ok(_) => {
                                app.load_medications();
                                app.selected_index = 0;
                                app.set_status("Medication created successfully", 3000);
                                app.current_screen = Screen::HomeScreen;
                            }
                            Err(e) => {
                                app.status_message = Some(format!("Create error: {e}"));
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
                    name,
                    amount_mg,
                    selected_frequency,
                    new_nav,
                    insert_mode,
                );
            }
            crossterm::event::KeyCode::Char('i') if !insert_mode => {
                set_screen(app, name, amount_mg, selected_frequency, nav, true);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    fn make_screen(focused_field: u8, insert_mode: bool) -> Screen {
        Screen::CreateMedication {
            name: "A".into(),
            amount_mg: "100".into(),
            selected_frequency: 0,
            scheduled_time: vec!["08:00".into()],
            scheduled_idx: 0,
            focused_field,
            insert_mode,
        }
    }

    fn press(app: &mut App, code: KeyCode) {
        CreateMedicationHandler.handle(app, KeyEvent::new(code, KeyModifiers::NONE));
    }

    fn new_app() -> App {
        App::new(std::sync::Arc::new(
            crate::infrastructure::container::Container::new(),
        ))
    }

    // --- Esc ---

    #[test]
    fn handle_esc_in_normal_mode_opens_confirm_cancel() {
        let mut app = new_app();
        app.current_screen = make_screen(0, false);

        press(&mut app, KeyCode::Esc);

        assert!(matches!(app.current_screen, Screen::ConfirmCancel { .. }));
    }

    #[test]
    fn handle_esc_in_insert_mode_exits_insert_mode() {
        let mut app = new_app();
        app.current_screen = make_screen(0, true);

        press(&mut app, KeyCode::Esc);

        assert!(matches!(
            app.current_screen,
            Screen::CreateMedication {
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
            Screen::CreateMedication {
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
            Screen::CreateMedication {
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
            Screen::CreateMedication {
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
            Screen::CreateMedication {
                focused_field: 1,
                ..
            }
        ));
    }

    // --- frequency navigation ---

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

    // --- custom slot management ---

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

    // --- text input ---

    #[test]
    fn handle_char_in_insert_mode_appends_to_name_field() {
        let mut app = new_app();
        app.current_screen = make_screen(0, true);

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
        app.current_screen = make_screen(0, true);

        press(&mut app, KeyCode::Backspace);

        if let Screen::CreateMedication { name, .. } = &app.current_screen {
            assert_eq!(name, "");
        } else {
            panic!("unexpected screen");
        }
    }

    /// Verify trait-object dispatch compiles and works.
    #[test]
    fn handle_dispatches_correctly_through_trait_object() {
        use crate::presentation::tui::handlers::port::Handler;
        let mut app = new_app();
        app.current_screen = make_screen(0, false);
        let mut handler: Box<dyn Handler> = Box::new(CreateMedicationHandler);
        handler.handle(&mut app, KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        assert!(matches!(app.current_screen, Screen::ConfirmCancel { .. }));
    }
}
