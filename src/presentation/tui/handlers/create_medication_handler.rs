use crate::application::dtos::requests::CreateMedicationRequest;
use crate::presentation::tui::app::App;
use crate::presentation::tui::handlers::medication_form_navigation::{
    NavigationState, navigate_down, navigate_left, navigate_right, navigate_up, remove_custom_slot,
};
use crate::presentation::tui::handlers::port::{Handler, HandlerResult};
use crate::presentation::tui::handlers::time_slot_parser::{
    frequency_str, parse_slots, validate_slot_count,
};
use crate::presentation::tui::input::Key;
use crate::presentation::tui::screen::Screen;

pub struct CreateMedicationHandler;

impl Default for CreateMedicationHandler {
    fn default() -> Self {
        CreateMedicationHandler
    }
}

struct CreateFormState {
    name: String,
    amount_mg: String,
    selected_frequency: usize,
    nav: NavigationState,
    insert_mode: bool,
}

impl CreateFormState {
    fn from_screen(screen: &Screen) -> Option<Self> {
        match screen {
            Screen::CreateMedication {
                name,
                amount_mg,
                selected_frequency,
                scheduled_time,
                scheduled_idx,
                focused_field,
                insert_mode,
            } => Some(Self {
                name: name.clone(),
                amount_mg: amount_mg.clone(),
                selected_frequency: *selected_frequency,
                nav: NavigationState {
                    focused_field: *focused_field,
                    scheduled_time: scheduled_time.clone(),
                    scheduled_idx: *scheduled_idx,
                },
                insert_mode: *insert_mode,
            }),
            _ => None,
        }
    }

    fn apply_to(self, app: &mut App) {
        app.current_screen = Screen::CreateMedication {
            name: self.name,
            amount_mg: self.amount_mg,
            selected_frequency: self.selected_frequency,
            scheduled_time: self.nav.scheduled_time,
            scheduled_idx: self.nav.scheduled_idx,
            focused_field: self.nav.focused_field,
            insert_mode: self.insert_mode,
        };
    }

    fn with_nav(self, nav: NavigationState) -> Self {
        Self { nav, ..self }
    }

    fn with_insert_mode(self, mode: bool) -> Self {
        Self {
            insert_mode: mode,
            ..self
        }
    }

    fn apply_navigate_right(self) -> Self {
        let (freq, nav) = navigate_right(self.nav, self.selected_frequency);
        Self {
            nav,
            selected_frequency: freq,
            ..self
        }
    }

    fn apply_navigate_left(self) -> Self {
        let (freq, nav) = navigate_left(self.nav, self.selected_frequency);
        Self {
            nav,
            selected_frequency: freq,
            ..self
        }
    }

    fn apply_navigate_down(self) -> Self {
        let nav = navigate_down(self.nav, self.selected_frequency);
        Self { nav, ..self }
    }

    fn apply_navigate_up(self) -> Self {
        let nav = navigate_up(self.nav);
        Self { nav, ..self }
    }

    fn delete_slot(mut self) -> Self {
        let slots = std::mem::take(&mut self.nav.scheduled_time);
        let (new_slots, new_idx) = remove_custom_slot(slots, self.nav.scheduled_idx);
        self.nav.scheduled_time = new_slots;
        self.nav.scheduled_idx = new_idx;
        self
    }

    fn type_char(mut self, c: char) -> Self {
        match self.nav.focused_field {
            0 => self.name.push(c),
            1 => self.amount_mg.push(c),
            3 => {
                while self.nav.scheduled_time.len() <= self.nav.scheduled_idx {
                    self.nav.scheduled_time.push(String::new());
                }
                self.nav.scheduled_time[self.nav.scheduled_idx].push(c);
            }
            _ => {}
        }
        self
    }

    fn delete_char(mut self) -> Self {
        match self.nav.focused_field {
            0 => {
                self.name.pop();
            }
            1 => {
                self.amount_mg.pop();
            }
            3 => {
                if self.nav.scheduled_time.len() > self.nav.scheduled_idx {
                    self.nav.scheduled_time[self.nav.scheduled_idx].pop();
                }
            }
            _ => {}
        }
        self
    }
}

fn handle_enter_create(app: &mut App, state: CreateFormState) {
    let CreateFormState {
        name,
        amount_mg,
        selected_frequency,
        nav,
        insert_mode,
    } = state;

    let parsed_amount: u32 = match amount_mg.trim().parse() {
        Ok(v) => v,
        Err(_) => {
            app.current_screen = Screen::ValidationError {
                message: "Invalid amount_mg value".into(),
                previous: Box::new(app.current_screen.clone()),
            };
            return;
        }
    };

    match parse_slots(&nav.scheduled_time) {
        Err(e) => {
            app.current_screen = Screen::ValidationError {
                message: e.to_string(),
                previous: Box::new(app.current_screen.clone()),
            };
            CreateFormState {
                name,
                amount_mg,
                selected_frequency,
                nav,
                insert_mode,
            }
            .apply_to(app);
        }
        Ok(parsed) => {
            if let Err(msg) = validate_slot_count(selected_frequency, parsed.times.len()) {
                app.current_screen = Screen::ValidationError {
                    message: msg.clone(),
                    previous: Box::new(app.current_screen.clone()),
                };
                let new_nav = NavigationState {
                    focused_field: nav.focused_field,
                    scheduled_time: parsed.normalized,
                    scheduled_idx: nav.scheduled_idx,
                };
                CreateFormState {
                    name,
                    amount_mg,
                    selected_frequency,
                    nav,
                    insert_mode,
                }
                .with_nav(new_nav)
                .apply_to(app);
                return;
            }
            let request = CreateMedicationRequest::new(
                name,
                parsed_amount,
                parsed.times,
                frequency_str(selected_frequency),
            );
            match app.services.create_medication.execute(request) {
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

impl Handler for CreateMedicationHandler {
    fn handle(&mut self, app: &mut App, key: Key) -> HandlerResult {
        let state = match CreateFormState::from_screen(&app.current_screen) {
            Some(s) => s,
            None => return HandlerResult::Continue,
        };
        match key {
            Key::Esc => {
                if state.insert_mode {
                    state.with_insert_mode(false).apply_to(app);
                } else {
                    app.current_screen = Screen::ConfirmCancel {
                        previous: Box::new(app.current_screen.clone()),
                    };
                }
            }
            Key::Tab | Key::Right | Key::Char('l') if !state.insert_mode => {
                state.apply_navigate_right().apply_to(app);
            }
            Key::Char('j') | Key::Down if !state.insert_mode => {
                state.apply_navigate_down().apply_to(app);
            }
            Key::Char('h') | Key::Left if !state.insert_mode => {
                state.apply_navigate_left().apply_to(app);
            }
            Key::Char('k') | Key::Up if !state.insert_mode => {
                state.apply_navigate_up().apply_to(app);
            }
            Key::Char('d')
                if !state.insert_mode
                    && state.nav.focused_field == 3
                    && state.selected_frequency == 3
                    && state.nav.scheduled_time.len() > 1 =>
            {
                state.delete_slot().apply_to(app);
            }
            Key::Enter => {
                handle_enter_create(app, state);
            }
            Key::Backspace if state.insert_mode => {
                state.delete_char().apply_to(app);
            }
            Key::Char('i') if !state.insert_mode => {
                state.with_insert_mode(true).apply_to(app);
            }
            Key::Char(c) if state.insert_mode => {
                state.type_char(c).apply_to(app);
            }
            _ => {}
        }
        HandlerResult::Continue
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::presentation::tui::handlers::port::Handler;
    use crossterm::event::KeyCode;

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
        CreateMedicationHandler.handle(app, crate::presentation::tui::input::from_code(code));
    }

    fn new_app() -> App {
        App::new(crate::presentation::tui::app_services::AppServices::fake())
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
        let mut app = new_app();
        app.current_screen = make_screen(0, false);
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
    fn handle_h_in_normal_mode_navigates_left() {
        // focused_field=2 with frequency=1 → navigate_left decrements frequency
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

    #[test]
    fn handle_enter_with_invalid_amount_shows_validation_error() {
        let mut app = new_app();
        app.current_screen = Screen::CreateMedication {
            name: "A".into(),
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
    fn handle_enter_with_invalid_time_slot_preserves_create_screen() {
        // parse_slots fails → ValidationError is set then overwritten by set_screen
        let mut app = new_app();
        app.current_screen = Screen::CreateMedication {
            name: "A".into(),
            amount_mg: "100".into(),
            selected_frequency: 0,
            scheduled_time: vec!["not-a-time".into()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: false,
        };

        press(&mut app, KeyCode::Enter);

        // set_screen overwrites the ValidationError back to CreateMedication
        assert!(matches!(
            app.current_screen,
            Screen::CreateMedication { .. }
        ));
    }

    #[test]
    fn handle_enter_with_wrong_slot_count_preserves_create_screen() {
        // TwiceDaily (selected_frequency=1) with only 1 slot → slot count mismatch
        // ValidationError is set then overwritten by set_screen (preserving nav state)
        let mut app = new_app();
        app.current_screen = Screen::CreateMedication {
            name: "A".into(),
            amount_mg: "100".into(),
            selected_frequency: 1,
            scheduled_time: vec!["08:00".into()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: false,
        };

        press(&mut app, KeyCode::Enter);

        assert!(matches!(
            app.current_screen,
            Screen::CreateMedication { .. }
        ));
    }

    #[test]
    fn handle_enter_with_valid_data_creates_medication_and_goes_home() {
        let mut app = new_app();
        app.current_screen = Screen::CreateMedication {
            name: "Aspirin".into(),
            amount_mg: "100".into(),
            selected_frequency: 0,
            scheduled_time: vec!["08:00".into()],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: false,
        };

        press(&mut app, KeyCode::Enter);

        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }
}
