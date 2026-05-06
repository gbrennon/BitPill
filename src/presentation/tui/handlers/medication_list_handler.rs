use crate::presentation::tui::{
    app::App,
    handlers::port::{Handler, HandlerResult},
    input::Key,
    screen::Screen,
};

pub struct MedicationListHandler;

impl Default for MedicationListHandler {
    fn default() -> Self {
        MedicationListHandler
    }
}

impl Handler for MedicationListHandler {
    fn handle(&mut self, app: &mut App, key: Key) -> HandlerResult {
        let vim_enabled = app.is_vim_mode();

        // Emacs mode: n/p for navigation
        if !vim_enabled {
            if let Key::Char('n') = key {
                if !app.medications.is_empty() {
                    app.selected_index =
                        (app.selected_index + 1).min(app.medications.len().saturating_sub(1));
                }
                return HandlerResult::Continue;
            }
            if let Key::Char('p') = key {
                app.selected_index = app.selected_index.saturating_sub(1);
                return HandlerResult::Continue;
            }
            if let Key::Char('f') = key {
                if !app.medications.is_empty() {
                    app.selected_index =
                        (app.selected_index + 1).min(app.medications.len().saturating_sub(1));
                }
                return HandlerResult::Continue;
            }
            if let Key::Char('b') = key {
                app.selected_index = app.selected_index.saturating_sub(1);
                return HandlerResult::Continue;
            }
            // Emacs mode: skip vim keys but allow other keys to pass through
            if matches!(
                key,
                Key::Char('j') | Key::Char('k') | Key::Char('h') | Key::Char('l')
            ) {
                return HandlerResult::Continue;
            }
        }

        // Vim mode: j/k/l/h for navigation
        match key {
            Key::Char('j') | Key::Char('l') if !app.medications.is_empty() => {
                app.selected_index =
                    (app.selected_index + 1).min(app.medications.len().saturating_sub(1));
            }
            Key::Down if !app.medications.is_empty() => {
                app.selected_index =
                    (app.selected_index + 1).min(app.medications.len().saturating_sub(1));
            }
            Key::Char('k') | Key::Char('h') => {
                app.selected_index = app.selected_index.saturating_sub(1);
            }
            Key::Up => {
                app.selected_index = app.selected_index.saturating_sub(1);
            }
            Key::Char('c') => {
                app.current_screen = Screen::CreateMedication {
                    name: String::new(),
                    amount_mg: String::new(),
                    selected_frequency: 0,
                    scheduled_time: vec![String::new()],
                    scheduled_idx: 0,
                    focused_field: 0,
                    insert_mode: false,
                };
            }
            Key::Char('s') => {
                let vim_enabled = match app
                    .services
                    .get_settings
                    .execute(crate::application::dtos::requests::GetSettingsRequest {})
                {
                    Ok(settings) => settings.navigation_mode == "vi",
                    Err(_) => false,
                };
                let selected_index = if vim_enabled { 0 } else { 1 };
                app.current_screen = Screen::Settings {
                    vim_enabled,
                    selected_index,
                };
            }
            Key::Char('v') if !app.medications.is_empty() => {
                let med = &app.medications[app.selected_index];
                app.current_screen = Screen::MedicationDetails { id: med.id.clone() };
            }
            Key::Char('m') if !app.medications.is_empty() => {
                let med = &app.medications[app.selected_index];
                match crate::application::ports::inbound::list_dose_records_port::ListDoseRecordsPort::execute(
                        &*app.services.list_dose_records,
                        crate::application::dtos::requests::ListDoseRecordsRequest {
                            medication_id: med.id.clone(),
                        },
                    ) {
                        Ok(resp) => {
                            app.current_screen = Screen::MarkDose {
                                medication_id: med.id.clone(),
                                records: resp.records,
                                selected_index: 0,
                            };
                        }
                        Err(e) => {
                            app.status_message = Some(format!("Error listing records: {e}"));
                        }
                    }
            }
            Key::Char('d') if !app.medications.is_empty() => {
                let med = &app.medications[app.selected_index];
                app.current_screen = Screen::ConfirmDelete {
                    id: med.id.clone(),
                    name: med.name.clone(),
                };
            }
            Key::Char('e') if !app.medications.is_empty() => {
                let med = &app.medications[app.selected_index];
                let times = med
                    .scheduled_time
                    .iter()
                    .map(|(h, m)| format!("{:02}:{:02}", h, m))
                    .collect::<Vec<_>>()
                    .join(",");
                let selected_frequency = match med.dose_frequency.as_str() {
                    "OnceDaily" => 0,
                    "TwiceDaily" => 1,
                    "ThriceDaily" => 2,
                    "Custom" => 3,
                    _ => 0,
                };
                app.current_screen = Screen::EditMedication {
                    id: med.id.clone(),
                    name: med.name.clone(),
                    amount_mg: med.amount_mg.to_string(),
                    selected_frequency,
                    scheduled_time: times.split(',').map(|s| s.to_string()).collect(),
                    scheduled_idx: 0,
                    focused_field: 0,
                    insert_mode: false,
                };
            }
            Key::Esc => {
                app.load_medications();
            }
            Key::Char('q') => {
                app.current_screen = Screen::ConfirmQuit {
                    previous: Box::new(app.current_screen.clone()),
                };
            }
            Key::Enter if !app.medications.is_empty() => {
                let med = &app.medications[app.selected_index];
                app.current_screen = Screen::MedicationDetails { id: med.id.clone() };
            }
            _ => {}
        }
        HandlerResult::Continue
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::presentation::tui::{app::App, input::Key};

    #[test]
    fn medication_list_navigation_and_actions() {
        let mut h = MedicationListHandler::default();
        let mut app = App::default();

        let med1 = crate::application::dtos::responses::MedicationDto {
            id: "m1".into(),
            name: "Name".into(),
            amount_mg: 10,
            scheduled_time: vec![(12, 0)],
            dose_frequency: "OnceDaily".into(),
            taken_today: 0,
            scheduled_today: 0,
        };
        let med2 = crate::application::dtos::responses::MedicationDto {
            id: "m2".into(),
            name: "Name2".into(),
            amount_mg: 5,
            scheduled_time: vec![(8, 0)],
            dose_frequency: "OnceDaily".into(),
            taken_today: 0,
            scheduled_today: 0,
        };
        app.medications = vec![med1, med2];

        // Vim navigation j
        h.handle(&mut app, Key::Char('j'));
        assert!(app.selected_index <= 1);

        // Enter -> MedicationDetails
        h.handle(&mut app, Key::Enter);
        assert!(matches!(
            app.current_screen,
            Screen::MedicationDetails { .. }
        ));

        // Create new
        app.current_screen = Screen::HomeScreen;
        h.handle(&mut app, Key::Char('c'));
        assert!(matches!(
            app.current_screen,
            Screen::CreateMedication { .. }
        ));

        // Settings 's' uses get_settings; default service returns empty -> no panic
        app.current_screen = Screen::HomeScreen;
        h.handle(&mut app, Key::Char('s'));
        assert!(matches!(app.current_screen, Screen::Settings { .. }));
    }
}

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

fn make_med(id: &str, name: &str) -> crate::application::dtos::responses::MedicationDto {
    crate::application::dtos::responses::MedicationDto {
        id: id.into(),
        name: name.into(),
        amount_mg: 100,
        scheduled_time: vec![(8, 0)],
        dose_frequency: "OnceDaily".into(),
        taken_today: 0,
        scheduled_today: 0,
    }
}

// --- Emacs mode ---
#[test]
fn emacs_n_moves_down() {
    let mut h = MedicationListHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("emacs"));
    a.medications = vec![make_med("m1", "A"), make_med("m2", "B")];
    h.handle(&mut a, Key::Char('n'));
    assert_eq!(a.selected_index, 1);
}
#[test]
fn emacs_p_moves_up() {
    let mut h = MedicationListHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("emacs"));
    a.medications = vec![make_med("m1", "A"), make_med("m2", "B")];
    a.selected_index = 1;
    h.handle(&mut a, Key::Char('p'));
    assert_eq!(a.selected_index, 0);
}
#[test]
fn emacs_f_moves_down() {
    let mut h = MedicationListHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("emacs"));
    a.medications = vec![make_med("m1", "A"), make_med("m2", "B")];
    h.handle(&mut a, Key::Char('f'));
    assert_eq!(a.selected_index, 1);
}
#[test]
fn emacs_b_moves_up() {
    let mut h = MedicationListHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("emacs"));
    a.medications = vec![make_med("m1", "A"), make_med("m2", "B")];
    a.selected_index = 1;
    h.handle(&mut a, Key::Char('b'));
    assert_eq!(a.selected_index, 0);
}
#[test]
fn emacs_skip_vim_keys() {
    let mut h = MedicationListHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("emacs"));
    a.medications = vec![make_med("m1", "A")];
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
fn emacs_empty_list_n_noop() {
    let mut h = MedicationListHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("emacs"));
    h.handle(&mut a, Key::Char('n'));
}
#[test]
fn emacs_empty_list_f_noop() {
    let mut h = MedicationListHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("emacs"));
    h.handle(&mut a, Key::Char('f'));
}

// --- Vim mode ---
#[test]
fn vim_j_moves_down() {
    let mut h = MedicationListHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("vi"));
    a.medications = vec![make_med("m1", "A"), make_med("m2", "B")];
    h.handle(&mut a, Key::Char('j'));
    assert_eq!(a.selected_index, 1);
}
#[test]
fn vim_k_moves_up() {
    let mut h = MedicationListHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("vi"));
    a.medications = vec![make_med("m1", "A"), make_med("m2", "B")];
    a.selected_index = 1;
    h.handle(&mut a, Key::Char('k'));
    assert_eq!(a.selected_index, 0);
}
#[test]
fn vim_l_moves_down() {
    let mut h = MedicationListHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("vi"));
    a.medications = vec![make_med("m1", "A"), make_med("m2", "B")];
    h.handle(&mut a, Key::Char('l'));
    assert_eq!(a.selected_index, 1);
}
#[test]
fn vim_h_moves_up() {
    let mut h = MedicationListHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("vi"));
    a.medications = vec![make_med("m1", "A"), make_med("m2", "B")];
    a.selected_index = 1;
    h.handle(&mut a, Key::Char('h'));
    assert_eq!(a.selected_index, 0);
}
#[test]
fn vim_down_moves_down() {
    let mut h = MedicationListHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("vi"));
    a.medications = vec![make_med("m1", "A"), make_med("m2", "B")];
    h.handle(&mut a, Key::Down);
    assert_eq!(a.selected_index, 1);
}
#[test]
fn vim_up_moves_up() {
    let mut h = MedicationListHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("vi"));
    a.medications = vec![make_med("m1", "A"), make_med("m2", "B")];
    a.selected_index = 1;
    h.handle(&mut a, Key::Up);
    assert_eq!(a.selected_index, 0);
}
#[test]
fn vim_j_empty_list_noop() {
    let mut h = MedicationListHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("vi"));
    h.handle(&mut a, Key::Char('j'));
    assert_eq!(a.selected_index, 0);
}
#[test]
fn vim_down_empty_list_noop() {
    let mut h = MedicationListHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("vi"));
    h.handle(&mut a, Key::Down);
    assert_eq!(a.selected_index, 0);
}
#[test]
fn vim_c_opens_create() {
    let mut h = MedicationListHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("vi"));
    h.handle(&mut a, Key::Char('c'));
    assert!(matches!(a.current_screen, Screen::CreateMedication { .. }));
}
#[test]
fn vim_s_opens_settings() {
    let mut h = MedicationListHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("vi"));
    h.handle(&mut a, Key::Char('s'));
    assert!(matches!(a.current_screen, Screen::Settings { .. }));
}
#[test]
fn vim_v_opens_details() {
    let mut h = MedicationListHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("vi"));
    a.medications = vec![make_med("m1", "A")];
    h.handle(&mut a, Key::Char('v'));
    assert!(matches!(a.current_screen, Screen::MedicationDetails { .. }));
}
#[test]
fn vim_m_opens_mark_dose() {
    let mut h = MedicationListHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("vi"));
    a.medications = vec![make_med("m1", "A")];
    h.handle(&mut a, Key::Char('m'));
    assert!(matches!(a.current_screen, Screen::MarkDose { .. }));
}
#[test]
fn vim_d_opens_confirm_delete() {
    let mut h = MedicationListHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("vi"));
    a.medications = vec![make_med("m1", "A")];
    h.handle(&mut a, Key::Char('d'));
    h.handle(&mut a, Key::Char('d'));
    assert!(matches!(a.current_screen, Screen::ConfirmDelete { .. }));
}
#[test]
fn vim_e_opens_edit() {
    let mut h = MedicationListHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("vi"));
    a.medications = vec![make_med("m1", "A")];
    h.handle(&mut a, Key::Char('e'));
    assert!(matches!(a.current_screen, Screen::EditMedication { .. }));
}
#[test]
fn vim_esc_reloads() {
    let mut h = MedicationListHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("vi"));
    h.handle(&mut a, Key::Esc);
}
#[test]
fn vim_q_opens_confirm_quit() {
    let mut h = MedicationListHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("vi"));
    h.handle(&mut a, Key::Char('q'));
    assert!(matches!(a.current_screen, Screen::ConfirmQuit { .. }));
}
#[test]
fn vim_enter_opens_details() {
    let mut h = MedicationListHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("vi"));
    a.medications = vec![make_med("m1", "A")];
    h.handle(&mut a, Key::Enter);
    assert!(matches!(a.current_screen, Screen::MedicationDetails { .. }));
}
#[test]
fn vim_unknown_key_noop() {
    let mut h = MedicationListHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("vi"));
    a.medications = vec![make_med("m1", "A")];
    a.selected_index = 0;
    h.handle(&mut a, Key::Char('g'));
    assert_eq!(a.selected_index, 0);
}
#[test]
fn vim_empty_enter_noop() {
    let mut h = MedicationListHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("vi"));
    h.handle(&mut a, Key::Enter);
    assert!(matches!(a.current_screen, Screen::HomeScreen));
}
#[test]
fn vim_empty_v_noop() {
    let mut h = MedicationListHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("vi"));
    h.handle(&mut a, Key::Char('v'));
    assert!(matches!(a.current_screen, Screen::HomeScreen));
}
