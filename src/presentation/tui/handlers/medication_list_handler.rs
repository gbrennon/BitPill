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
        let _vim_enabled = match app.services.settings.execute(
            crate::application::dtos::requests::SettingsRequest {
                op: crate::application::dtos::requests::SettingsOperation::Get,
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
                // Mark-as-taken is only available from the Medication Details screen.
                app.set_status("Open medication details (v) to mark doses as taken", 3000);
            }
            crossterm::event::KeyCode::Char('v') => {
                if !app.medications.is_empty() {
                    let med = &app.medications[app.selected_index];
                    app.current_screen = Screen::MedicationDetails { id: med.id.clone() };
                }
            }
            crossterm::event::KeyCode::Char('g') => {
                // open settings
                let vim_enabled = match app.services.settings.execute(
                    crate::application::dtos::requests::SettingsRequest { op: crate::application::dtos::requests::SettingsOperation::Get },
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
            }
            crossterm::event::KeyCode::Char('d') => {
                if !app.medications.is_empty() {
                    let med = &app.medications[app.selected_index];
                    app.current_screen = Screen::ConfirmDelete {
                        id: med.id.clone(),
                        name: med.name.clone(),
                    };
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
    use crate::application::dtos::responses::MedicationDto;
    use crate::presentation::tui::app::App;
    use crate::presentation::tui::app_services::AppServices;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    fn new_app() -> App {
        App::new(AppServices::fake())
    }

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    fn med(id: &str) -> MedicationDto {
        MedicationDto {
            id: id.to_string(),
            name: "Med".to_string(),
            amount_mg: 100,
            dose_frequency: "OnceDaily".to_string(),
            scheduled_time: vec![(8, 0)],
        }
    }

    #[test]
    fn handle_quit_opens_confirm_quit_modal() {
        let mut app = new_app();
        let mut handler = MedicationListHandler;
        handler.handle(&mut app, key(KeyCode::Char('q')));
        assert!(matches!(app.current_screen, Screen::ConfirmQuit { .. }));
    }

    #[test]
    fn handle_dispatches_correctly_through_trait_object() {
        let mut app = new_app();
        let mut handler: Box<dyn Handler> = Box::new(MedicationListHandler);
        handler.handle(&mut app, key(KeyCode::Char('c')));
        assert!(matches!(app.current_screen, Screen::CreateMedication { .. }));
    }

    #[test]
    fn pressing_s_shows_instruction_to_open_details() {
        let mut app = new_app();
        app.medications = vec![med("m1")];
        let mut h = MedicationListHandler::default();
        h.handle(&mut app, key(KeyCode::Char('s')));
        assert!(app.status_message.as_deref().unwrap_or("").contains("Open medication details"));
    }

    #[test]
    fn j_increments_selected_index() {
        let mut app = new_app();
        app.medications = vec![med("m1"), med("m2")];
        let mut h = MedicationListHandler;
        h.handle(&mut app, key(KeyCode::Char('j')));
        assert_eq!(app.selected_index, 1);
    }

    #[test]
    fn down_arrow_increments_selected_index() {
        let mut app = new_app();
        app.medications = vec![med("m1"), med("m2")];
        let mut h = MedicationListHandler;
        h.handle(&mut app, key(KeyCode::Down));
        assert_eq!(app.selected_index, 1);
    }

    #[test]
    fn j_does_not_exceed_last_index() {
        let mut app = new_app();
        app.medications = vec![med("m1")];
        let mut h = MedicationListHandler;
        h.handle(&mut app, key(KeyCode::Char('j')));
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn k_decrements_selected_index() {
        let mut app = new_app();
        app.medications = vec![med("m1"), med("m2")];
        app.selected_index = 1;
        let mut h = MedicationListHandler;
        h.handle(&mut app, key(KeyCode::Char('k')));
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn k_clamps_at_zero() {
        let mut app = new_app();
        app.medications = vec![med("m1")];
        let mut h = MedicationListHandler;
        h.handle(&mut app, key(KeyCode::Char('k')));
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn up_arrow_decrements_selected_index() {
        let mut app = new_app();
        app.medications = vec![med("m1"), med("m2")];
        app.selected_index = 1;
        let mut h = MedicationListHandler;
        h.handle(&mut app, key(KeyCode::Up));
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn l_key_increments_index() {
        let mut app = new_app();
        app.medications = vec![med("m1"), med("m2")];
        let mut h = MedicationListHandler;
        h.handle(&mut app, key(KeyCode::Char('l')));
        assert_eq!(app.selected_index, 1);
    }

    #[test]
    fn h_key_decrements_index() {
        let mut app = new_app();
        app.medications = vec![med("m1"), med("m2")];
        app.selected_index = 1;
        let mut h = MedicationListHandler;
        h.handle(&mut app, key(KeyCode::Char('h')));
        assert_eq!(app.selected_index, 0);
    }

    #[test]
    fn v_opens_medication_details() {
        let mut app = new_app();
        app.medications = vec![med("med-v")];
        let mut h = MedicationListHandler;
        h.handle(&mut app, key(KeyCode::Char('v')));
        assert!(matches!(app.current_screen, Screen::MedicationDetails { id } if id == "med-v"));
    }

    #[test]
    fn enter_opens_medication_details() {
        let mut app = new_app();
        app.medications = vec![med("med-enter")];
        let mut h = MedicationListHandler;
        h.handle(&mut app, key(KeyCode::Enter));
        assert!(matches!(app.current_screen, Screen::MedicationDetails { id } if id == "med-enter"));
    }

    #[test]
    fn g_opens_settings() {
        let mut app = new_app();
        let mut h = MedicationListHandler;
        h.handle(&mut app, key(KeyCode::Char('g')));
        assert!(matches!(app.current_screen, Screen::Settings { .. }));
    }

    #[test]
    fn d_opens_confirm_delete() {
        let mut app = new_app();
        app.medications = vec![med("del-id")];
        let mut h = MedicationListHandler;
        h.handle(&mut app, key(KeyCode::Char('d')));
        assert!(matches!(app.current_screen, Screen::ConfirmDelete { id, .. } if id == "del-id"));
    }

    #[test]
    fn e_opens_edit_medication() {
        let mut app = new_app();
        app.medications = vec![med("edit-id")];
        let mut h = MedicationListHandler;
        h.handle(&mut app, key(KeyCode::Char('e')));
        assert!(matches!(app.current_screen, Screen::EditMedication { id, .. } if id == "edit-id"));
    }

    #[test]
    fn t_opens_mark_dose_screen() {
        let mut app = new_app();
        app.medications = vec![med("t-id")];
        let mut h = MedicationListHandler;
        h.handle(&mut app, key(KeyCode::Char('t')));
        assert!(matches!(app.current_screen, Screen::MarkDose { medication_id, .. } if medication_id == "t-id"));
    }

    #[test]
    fn esc_reloads_and_stays_on_home() {
        let mut app = new_app();
        let mut h = MedicationListHandler;
        h.handle(&mut app, key(KeyCode::Esc));
        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn no_op_when_no_medications_on_v() {
        let mut app = new_app();
        app.medications.clear();
        let mut h = MedicationListHandler;
        h.handle(&mut app, key(KeyCode::Char('v')));
        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }
}
