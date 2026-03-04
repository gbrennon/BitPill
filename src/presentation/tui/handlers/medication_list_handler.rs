use crate::application::ports::inbound::settings_port::SettingsPort;
use crate::application::ports::inbound::mark_medication_taken_port::MarkMedicationTakenPort;
use chrono::Local;
use crate::presentation::tui::app::App;
use crate::presentation::tui::handlers::port::{Handler, HandlerResult};
use crate::presentation::tui::screen::Screen;
use crossterm::event::KeyEvent;
use serde_json::Value;

pub struct MedicationListHandler;

impl Default for MedicationListHandler {
    fn default() -> Self {
        MedicationListHandler
    }
}

impl Handler for MedicationListHandler {
    fn handle(&mut self, app: &mut App, key: KeyEvent) -> HandlerResult {
        // Query application Settings service (inbound port) instead of reading repository directly
        let _vim_enabled = match app.container.get_settings_service().execute(
            crate::application::ports::inbound::settings_port::SettingsRequest {
                op: crate::application::ports::inbound::settings_port::SettingsOperation::Get,
            },
        ) {
            Ok(resp) => resp
                .settings
                .get("vim_navigation")
                .and_then(|v: &Value| v.as_bool())
                .unwrap_or(false),
            Err(_) => false,
        };
        match key.code {

            crossterm::event::KeyCode::Char('j') | crossterm::event::KeyCode::Char('l') => {
                if !app.medications.is_empty() {
                    app.selected_index =
                        (app.selected_index + 1).min(app.medications.len().saturating_sub(1));
                }
            }
            crossterm::event::KeyCode::Down => {
                if !app.medications.is_empty() {
                    app.selected_index =
                        (app.selected_index + 1).min(app.medications.len().saturating_sub(1));
                }
            }
            crossterm::event::KeyCode::Char('k') | crossterm::event::KeyCode::Char('h') => {
                app.selected_index = app.selected_index.saturating_sub(1);
            }
            crossterm::event::KeyCode::Up => {
                app.selected_index = app.selected_index.saturating_sub(1);
            }
            crossterm::event::KeyCode::Char('c') => {
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
            crossterm::event::KeyCode::Char('s') => {
                // Mark the medication as taken now by creating a dose record for current time
                if !app.medications.is_empty() {
                    let med = &app.medications[app.selected_index];
                    let req = crate::application::ports::inbound::mark_medication_taken_port::MarkMedicationTakenRequest::new(
                        med.id.clone(),
                        Local::now().naive_local(),
                    );
                    match app.container.mark_medication_taken_service.execute(req) {
                        Ok(_) => {
                            app.set_status("Marked dose as taken", 3000);
                            app.load_medications();
                        }
                        Err(e) => app.status_message = Some(format!("Error marking dose: {e}")),
                    }
                }
            }
            crossterm::event::KeyCode::Char('v') => {
                if !app.medications.is_empty() {
                    let med = &app.medications[app.selected_index];
                    app.current_screen = Screen::MedicationDetails { id: med.id.clone() };
                }
            }
            crossterm::event::KeyCode::Char('g') => {
                // open settings
                let vim_enabled = match app.container.get_settings_service().execute(
                    crate::application::ports::inbound::settings_port::SettingsRequest { op: crate::application::ports::inbound::settings_port::SettingsOperation::Get },
                ) {
                    Ok(resp) => resp.settings.get("vim_navigation").and_then(|v| v.as_bool()).unwrap_or(false),
                    Err(_) => false,
                };
                app.current_screen = Screen::Settings { vim_enabled };
            }
            crossterm::event::KeyCode::Char('t') => {
                if !app.medications.is_empty() {
                    let med = &app.medications[app.selected_index];
                    match crate::application::ports::inbound::list_dose_records_port::ListDoseRecordsPort::execute(
                        &app.container.list_dose_records_service,
                        crate::application::ports::inbound::list_dose_records_port::ListDoseRecordsRequest {
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
            }
            crossterm::event::KeyCode::Char('d') => {
                if !app.medications.is_empty() {
                    let med = &app.medications[app.selected_index];
                    app.current_screen = Screen::ConfirmDelete { id: med.id.clone(), name: med.name.clone() };
                }
            }
            crossterm::event::KeyCode::Char('e') => {
                if !app.medications.is_empty() {
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
            }
            crossterm::event::KeyCode::Esc => {
                app.load_medications();
            }
            crossterm::event::KeyCode::Char('q') => {
                app.current_screen = Screen::ConfirmQuit {
                    previous: Box::new(app.current_screen.clone()),
                };
            }
            crossterm::event::KeyCode::Enter => {
                if !app.medications.is_empty() {
                    let med = &app.medications[app.selected_index];
                    app.current_screen = Screen::MedicationDetails { id: med.id.clone() };
                }
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
    use std::sync::Arc;

    #[test]
    fn handle_quit_opens_confirm_quit_modal() {
        let container = Arc::new(crate::infrastructure::container::Container::new());
        let mut app = App::new(container);
        let mut handler = MedicationListHandler;
        let key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
        handler.handle(&mut app, key);
        assert!(matches!(app.current_screen, Screen::ConfirmQuit { .. }));
    }

    /// Verifies `MedicationListHandler` is callable via a `Handler` trait object.
    /// If the trait ever becomes non-object-safe this test will fail to compile.
    #[test]
    fn handle_dispatches_correctly_through_trait_object() {
        use crate::presentation::tui::handlers::port::Handler;

        let container = Arc::new(crate::infrastructure::container::Container::new());
        let mut app = App::new(container);
        let mut handler: Box<dyn Handler> = Box::new(MedicationListHandler);
        // 'c' opens the create-medication form — a simple, stable transition
        let key = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE);
        handler.handle(&mut app, key);

        assert!(matches!(app.current_screen, Screen::CreateMedication { .. }));
    }

    #[test]
    fn pressing_s_creates_dose_record_without_freeze() {
        use crate::application::ports::fakes::FakeDoseRecordRepository;
        use crate::application::ports::outbound::dose_record_repository_port::DoseRecordRepository;
        use crate::application::ports::inbound::list_all_medications_port::MedicationDto;
        use crate::application::services::mark_medication_taken_service::MarkMedicationTakenService;

        // Build a container with a fake dose repository so disk IO isn't involved
        let fake_repo = Arc::new(FakeDoseRecordRepository::new());
        let repo_trait: Arc<dyn DoseRecordRepository> = fake_repo.clone();
        let mut container = crate::infrastructure::container::Container::new();
        container.mark_medication_taken_service = MarkMedicationTakenService::new(repo_trait);
        let arc = Arc::new(container);

        let mut app = App::new(arc.clone());
        // insert a medication into app state
        let med = MedicationDto { id: "00000000-0000-0000-0000-000000000001".to_string(), name: "Test".to_string(), amount_mg: 10, scheduled_time: vec![(8,0)], dose_frequency: "OnceDaily".to_string() };
        app.medications.push(med);
        app.selected_index = 0;

        let mut handler = MedicationListHandler::default();
        let key = KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE);
        handler.handle(&mut app, key);

        // ensure the fake repo recorded the save
        assert_eq!(fake_repo.saved_count(), 1);
    }
}
