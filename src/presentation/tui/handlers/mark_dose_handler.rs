use crossterm::event::KeyEvent;

use crate::application::ports::inbound::mark_dose_taken_port::MarkDoseTakenRequest;
use crate::presentation::tui::app::App;
use crate::presentation::tui::handlers::port::{Handler, HandlerResult};
use crate::presentation::tui::screen::Screen;

pub struct MarkDoseHandler;

impl Default for MarkDoseHandler {
    fn default() -> Self {
        MarkDoseHandler
    }
}

impl Handler for MarkDoseHandler {
    fn handle(&mut self, app: &mut App, key: KeyEvent) -> HandlerResult {
        match &app.current_screen {
            Screen::MarkDose { medication_id, records, selected_index } => {
                match key.code {
                    crossterm::event::KeyCode::Esc => {
                        app.current_screen = Screen::HomeScreen;
                    }
                    crossterm::event::KeyCode::Char('j') | crossterm::event::KeyCode::Down => {
                        let idx = (*selected_index + 1).min(records.len().saturating_sub(1));
                        app.current_screen = Screen::MarkDose {
                            medication_id: medication_id.clone(),
                            records: records.to_vec(),
                            selected_index: idx,
                        };
                    }
                    crossterm::event::KeyCode::Char('k') | crossterm::event::KeyCode::Up => {
                        let idx = selected_index.saturating_sub(1);
                        app.current_screen = Screen::MarkDose {
                            medication_id: medication_id.clone(),
                            records: records.to_vec(),
                            selected_index: idx,
                        };
                    }
                    crossterm::event::KeyCode::Enter => {
                        if records.is_empty() {
                            app.set_status("No records to mark", 3000);
                            app.current_screen = Screen::HomeScreen;
                        } else {
                            let rec = &records[*selected_index];
                            let req = MarkDoseTakenRequest::new(rec.id.clone(), chrono::Local::now().naive_local());
                            match crate::application::ports::inbound::mark_dose_taken_port::MarkDoseTakenPort::execute(&app.container.mark_dose_taken_service, req) {
                                Ok(_) => app.set_status("Marked as taken", 3000),
                                Err(e) => app.status_message = Some(format!("Error: {e}")),
                            }
                            app.current_screen = Screen::HomeScreen;
                            app.load_medications();
                        }
                    }
                    _ => {}
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

    /// Verifies `MarkDoseHandler` is callable via a `Handler` trait object.
    #[test]
    fn handle_dispatches_correctly_through_trait_object() {
        use crate::presentation::tui::handlers::port::Handler;

        let container = Arc::new(crate::infrastructure::container::Container::new());
        let mut app = App::new(container);
        app.current_screen = Screen::MarkDose {
            medication_id: "med-1".to_string(),
            records: vec![],
            selected_index: 0,
        };
        let mut handler: Box<dyn Handler> = Box::new(MarkDoseHandler);
        let key = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
        handler.handle(&mut app, key);

        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }
}
