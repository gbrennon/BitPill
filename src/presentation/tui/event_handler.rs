use crossterm::event::{KeyCode, KeyEvent};

use crate::application::ports::create_medication_port::{
    CreateMedicationPort, CreateMedicationRequest,
};
use crate::presentation::tui::app::App;
use crate::presentation::tui::screen::Screen;

pub struct EventHandler;

impl EventHandler {
    pub fn handle(app: &mut App, key: KeyEvent) {
        match &app.current_screen {
            Screen::MedicationList => handle_medication_list(app, key),
            Screen::CreateMedication { .. } => handle_create_medication(app, key),
            Screen::ScheduleResult { .. } => handle_schedule_result(app),
        }
    }
}

fn handle_medication_list(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Char('j') | KeyCode::Down => {
            if !app.medications.is_empty() {
                app.selected_index =
                    (app.selected_index + 1).min(app.medications.len().saturating_sub(1));
            }
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app.selected_index = app.selected_index.saturating_sub(1);
        }
        KeyCode::Char('c') => {
            app.current_screen = Screen::CreateMedication {
                name: String::new(),
                amount_mg: String::new(),
                scheduled_times: String::new(),
                focused_field: 0,
            };
        }
        KeyCode::Char('s') => {
            match app.container.schedule_dose_service.execute() {
                Ok(records) => {
                    app.current_screen = Screen::ScheduleResult {
                        created_count: records.len(),
                    };
                }
                Err(e) => {
                    app.status_message = Some(format!("Schedule error: {e}"));
                }
            }
        }
        KeyCode::Esc | KeyCode::Enter => {
            app.load_medications();
        }
        _ => {}
    }
}

fn handle_create_medication(app: &mut App, key: KeyEvent) {
    // Clone the fields out to avoid simultaneous borrow
    let (name, amount_mg, scheduled_times, focused_field) =
        if let Screen::CreateMedication {
            name,
            amount_mg,
            scheduled_times,
            focused_field,
        } = &app.current_screen
        {
            (
                name.clone(),
                amount_mg.clone(),
                scheduled_times.clone(),
                *focused_field,
            )
        } else {
            return;
        };

    match key.code {
        KeyCode::Esc => {
            app.current_screen = Screen::MedicationList;
        }
        KeyCode::Tab => {
            let next_field = (focused_field + 1) % 3;
            app.current_screen = Screen::CreateMedication {
                name,
                amount_mg,
                scheduled_times,
                focused_field: next_field,
            };
        }
        KeyCode::Enter => {
            let parsed_amount: u32 = match amount_mg.trim().parse() {
                Ok(v) => v,
                Err(_) => {
                    app.status_message = Some("Invalid amount_mg value".into());
                    return;
                }
            };

            let parsed_times: Vec<(u32, u32)> = if scheduled_times.trim().is_empty() {
                vec![]
            } else {
                let mut times = Vec::new();
                for part in scheduled_times.split(',') {
                    let part = part.trim();
                    let mut iter = part.splitn(2, ':');
                    let h: u32 = iter.next().and_then(|s| s.parse().ok()).unwrap_or(0);
                    let m: u32 = iter.next().and_then(|s| s.parse().ok()).unwrap_or(0);
                    times.push((h, m));
                }
                times
            };

            let request = CreateMedicationRequest::new(name, parsed_amount, parsed_times);
            match app.container.create_medication_service.execute(request) {
                Ok(_) => {
                    app.current_screen = Screen::MedicationList;
                    app.load_medications();
                }
                Err(e) => {
                    app.status_message = Some(format!("Create error: {e}"));
                    app.current_screen = Screen::MedicationList;
                }
            }
        }
        KeyCode::Backspace => {
            let mut name = name;
            let mut amount_mg = amount_mg;
            let mut scheduled_times = scheduled_times;
            match focused_field {
                0 => {
                    name.pop();
                }
                1 => {
                    amount_mg.pop();
                }
                _ => {
                    scheduled_times.pop();
                }
            }
            app.current_screen = Screen::CreateMedication {
                name,
                amount_mg,
                scheduled_times,
                focused_field,
            };
        }
        KeyCode::Char(c) => {
            let mut name = name;
            let mut amount_mg = amount_mg;
            let mut scheduled_times = scheduled_times;
            match focused_field {
                0 => name.push(c),
                1 => amount_mg.push(c),
                _ => scheduled_times.push(c),
            }
            app.current_screen = Screen::CreateMedication {
                name,
                amount_mg,
                scheduled_times,
                focused_field,
            };
        }
        _ => {}
    }
}

fn handle_schedule_result(app: &mut App) {
    app.current_screen = Screen::MedicationList;
    app.load_medications();
}
