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
