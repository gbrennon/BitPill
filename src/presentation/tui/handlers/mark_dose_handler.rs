use crate::{
    application::dtos::requests::MarkDoseTakenRequest,
    presentation::tui::{
        app::App,
        handlers::port::{Handler, HandlerResult},
        input::Key,
        screen::Screen,
    },
};

pub struct MarkDoseHandler;

impl Default for MarkDoseHandler {
    fn default() -> Self {
        MarkDoseHandler
    }
}

impl Handler for MarkDoseHandler {
    fn handle(&mut self, app: &mut App, key: Key) -> HandlerResult {
        // Extract current mark-dose state up-front to avoid borrowing `app` mutably while it's still borrowed immutably.
        let (med_id, recs, sel_idx) = if let Screen::MarkDose {
            medication_id,
            records,
            selected_index,
        } = &app.current_screen
        {
            (medication_id.clone(), records.clone(), *selected_index)
        } else {
            return HandlerResult::Continue;
        };

        let vim_enabled = app.is_vim_mode();

        // Emacs mode: n/p for navigation
        if !vim_enabled {
            if let Key::Char('n') = key {
                let idx = (sel_idx + 1).min(recs.len().saturating_sub(1));
                app.current_screen = Screen::MarkDose {
                    medication_id: med_id.clone(),
                    records: recs.to_vec(),
                    selected_index: idx,
                };
                return HandlerResult::Continue;
            }
            if let Key::Char('p') = key {
                let idx = sel_idx.saturating_sub(1);
                app.current_screen = Screen::MarkDose {
                    medication_id: med_id.clone(),
                    records: recs.to_vec(),
                    selected_index: idx,
                };
                return HandlerResult::Continue;
            }
            if let Key::Char('f') = key {
                let idx = (sel_idx + 1).min(recs.len().saturating_sub(1));
                app.current_screen = Screen::MarkDose {
                    medication_id: med_id.clone(),
                    records: recs.to_vec(),
                    selected_index: idx,
                };
                return HandlerResult::Continue;
            }
            if let Key::Char('b') = key {
                let idx = sel_idx.saturating_sub(1);
                app.current_screen = Screen::MarkDose {
                    medication_id: med_id.clone(),
                    records: recs.to_vec(),
                    selected_index: idx,
                };
                return HandlerResult::Continue;
            }
            // Skip vim keys but allow other keys to pass through
            if matches!(key, Key::Char('j') | Key::Char('k')) {
                return HandlerResult::Continue;
            }
        }

        // Vim mode: j/k for navigation
        match key {
            Key::Esc => {
                app.current_screen = Screen::HomeScreen;
            }
            Key::Char('j') | Key::Down => {
                let idx = (sel_idx + 1).min(recs.len().saturating_sub(1));
                app.current_screen = Screen::MarkDose {
                    medication_id: med_id.clone(),
                    records: recs.to_vec(),
                    selected_index: idx,
                };
            }
            Key::Char('k') | Key::Up => {
                let idx = sel_idx.saturating_sub(1);
                app.current_screen = Screen::MarkDose {
                    medication_id: med_id.clone(),
                    records: recs.to_vec(),
                    selected_index: idx,
                };
            }
            Key::Enter => {
                if recs.is_empty() {
                    app.set_status("No records to mark", 3000);
                    app.current_screen = Screen::HomeScreen;
                } else {
                    let rec = &recs[sel_idx];
                    if rec.id.starts_with("slot:") {
                        match crate::application::ports::inbound::mark_dose_taken_port::MarkDoseTakenPort::execute(
                            &*app.services.mark_dose_taken,
                            crate::application::dtos::requests::MarkDoseTakenRequest::new_with_schedule(
                                rec.medication_id.clone(),
                                rec.scheduled_at,
                            ),
                        ) {
                            Ok(_) => app.set_status("Marked scheduled slot as taken", 3000),
                            Err(e) => app.status_message = Some(format!("Error: {e}")),
                        }
                        app.load_medications();
                        app.current_screen = Screen::MedicationDetails {
                            id: rec.medication_id.clone(),
                        };
                    } else {
                        let req = MarkDoseTakenRequest::new(rec.id.clone());
                        match crate::application::ports::inbound::mark_dose_taken_port::MarkDoseTakenPort::execute(&*app.services.mark_dose_taken, req) {
                            Ok(_) => {
                                app.set_status("Marked as taken", 3000);
                                app.load_medications();
                                let new_records: Vec<crate::application::dtos::responses::DoseRecordDto> = match crate::application::ports::inbound::list_dose_records_port::ListDoseRecordsPort::execute(
                                    &*app.services.list_dose_records,
                                    crate::application::dtos::requests::ListDoseRecordsRequest {
                                        medication_id: med_id.clone(),
                                    },
                                ) {
                                    Ok(resp) => {
                                        let today = chrono::Local::now().date_naive();
                                        resp.records.into_iter()
                                            .filter(|r| r.scheduled_at.date() == today && r.taken_at.is_none())
                                            .collect()
                                    }
                                    Err(_) => vec![],
                                };
                                let new_len = new_records.len();
                                app.current_screen = Screen::MarkDose {
                                    medication_id: med_id,
                                    records: new_records,
                                    selected_index: sel_idx.min(new_len.saturating_sub(1)),
                                };
                            }
                            Err(e) => app.status_message = Some(format!("Error: {e}")),
                        }
                    }
                }
            }
            _ => {}
        }
        HandlerResult::Continue
    }
}
