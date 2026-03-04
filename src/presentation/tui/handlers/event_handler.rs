use crossterm::event::{KeyCode, KeyEvent};

use crate::presentation::tui::app::App;
use crate::presentation::tui::handlers::create_medication_handler::CreateMedicationHandler;
use crate::presentation::tui::handlers::medication_list_handler::MedicationListHandler;
use crate::presentation::tui::handlers::port::{Handler, HandlerResult};
use crate::presentation::tui::handlers::mark_dose_handler::MarkDoseHandler;
use crate::presentation::tui::screen::Screen;
use crate::application::ports::inbound::settings_port::SettingsPort;

#[derive(Default)]
pub struct EventHandler {
    medication_list_handler: MedicationListHandler,
    create_medication_handler: CreateMedicationHandler,
    edit_medication_handler: crate::presentation::tui::handlers::edit_medication_handler::EditMedicationHandler,
    mark_dose_handler: MarkDoseHandler,
}

impl Handler for EventHandler {
    fn handle(&mut self, app: &mut App, key: KeyEvent) -> HandlerResult {
        // Global quit: pressing 'q' anywhere opens a quit confirmation modal.
        if let KeyCode::Char('q') = key.code {
            if !matches!(app.current_screen, Screen::ConfirmQuit { .. }) {
                app.current_screen = Screen::ConfirmQuit { previous: Box::new(app.current_screen.clone()) };
                return HandlerResult::Continue;
            }
        }

        match &app.current_screen {
            Screen::HomeScreen => self.medication_list_handler.handle(app, key),
            Screen::CreateMedication { .. } => self.create_medication_handler.handle(app, key),
            Screen::EditMedication { .. } => self.edit_medication_handler.handle(app, key),
            Screen::MedicationDetails { .. } => {
                // handle simple navigation and edit shortcut inside details screen
                match key.code {
                    KeyCode::Esc => {
                        app.current_screen = Screen::HomeScreen;
                    }
                    KeyCode::Char('e') => {
                        if let Screen::MedicationDetails { id } = &app.current_screen
                            && let Some(m) = app.medications.iter().find(|m| m.id == *id)
                        {
                            let times = m
                                .scheduled_time
                                .iter()
                                .map(|(h, m)| format!("{:02}:{:02}", h, m))
                                .collect::<Vec<_>>()
                                .join(",");
                            let selected_frequency = match m.dose_frequency.as_str() {
                                "OnceDaily" => 0,
                                "TwiceDaily" => 1,
                                "ThriceDaily" => 2,
                                _ => 0,
                            };
                            app.current_screen = Screen::EditMedication {
                                id: id.clone(),
                                name: m.name.clone(),
                                amount_mg: m.amount_mg.to_string(),
                                selected_frequency,
                                scheduled_time: times.split(',').map(|s| s.to_string()).collect(),
                                scheduled_idx: 0,
                                focused_field: 0,
                                insert_mode: false,
                            };
                        }
                    }
                    _ => {}
                }
                HandlerResult::Continue
            }
            Screen::Settings { vim_enabled } => {
                match key.code {
                    KeyCode::Char(' ') => {
                        app.current_screen = Screen::Settings { vim_enabled: !*vim_enabled };
                    }
                    KeyCode::Char('s') => {
                        // persist settings: read current state value and update
                        let value = if let Screen::Settings { vim_enabled } = &app.current_screen { *vim_enabled } else { *vim_enabled };
                        let new = serde_json::json!({ "vim_navigation": value });
                        match app.container.get_settings_service().execute(crate::application::ports::inbound::settings_port::SettingsRequest { op: crate::application::ports::inbound::settings_port::SettingsOperation::Update { settings: new.clone() } }) {
                            Ok(_) => app.set_status("Settings saved", 2000),
                            Err(e) => app.status_message = Some(format!("Settings save error: {e}")),
                        }
                        app.current_screen = Screen::HomeScreen;
                    }
                    KeyCode::Esc => {
                        app.current_screen = Screen::HomeScreen;
                    }
                    _ => {}
                }
                HandlerResult::Continue
            }
            Screen::ConfirmDelete { .. } => {
                match key.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => {
                        if let Screen::ConfirmDelete { id, .. } = &app.current_screen {
                            // call delete service
                            match crate::application::ports::inbound::delete_medication_port::DeleteMedicationPort::execute(
                                &app.container.delete_medication_service,
                                crate::application::ports::inbound::delete_medication_port::DeleteMedicationRequest { id: id.clone() },
                            ) {
                                Ok(_) => {
                                    app.set_status("Medication deleted", 3000);
                                    app.load_medications();
                                }
                                Err(e) => {
                                    app.status_message = Some(format!("Delete error: {e}"));
                                }
                            }
                        }
                        app.current_screen = Screen::HomeScreen;
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                        app.current_screen = Screen::HomeScreen;
                    }
                    _ => {}
                }
                HandlerResult::Continue
            }
            Screen::ConfirmCancel { previous } => {
                match key.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => {
                        app.current_screen = Screen::HomeScreen;
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                        // return to previous view
                        app.current_screen = *previous.clone();
                    }
                    _ => {}
                }
                HandlerResult::Continue
            }
            Screen::ConfirmQuit { previous } => {
                match key.code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => {
                        app.should_quit = true;
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                        // return to previous view
                        app.current_screen = *previous.clone();
                    }
                    _ => {}
                }
                HandlerResult::Continue
            }
            Screen::MarkDose { .. } => self.mark_dose_handler.handle(app, key),
        }
    }
}
