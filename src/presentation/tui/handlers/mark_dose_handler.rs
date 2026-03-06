use crossterm::event::KeyEvent;

use crate::application::dtos::requests::MarkDoseTakenRequest;
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

        match key.code {
            crossterm::event::KeyCode::Esc => {
                app.current_screen = Screen::HomeScreen;
            }
            crossterm::event::KeyCode::Char('j') | crossterm::event::KeyCode::Down => {
                let idx = (sel_idx + 1).min(recs.len().saturating_sub(1));
                app.current_screen = Screen::MarkDose {
                    medication_id: med_id.clone(),
                    records: recs.to_vec(),
                    selected_index: idx,
                };
            }
            crossterm::event::KeyCode::Char('k') | crossterm::event::KeyCode::Up => {
                let idx = sel_idx.saturating_sub(1);
                app.current_screen = Screen::MarkDose {
                    medication_id: med_id.clone(),
                    records: recs.to_vec(),
                    selected_index: idx,
                };
            }
            crossterm::event::KeyCode::Enter => {
                if recs.is_empty() {
                    app.set_status("No records to mark", 3000);
                    app.current_screen = Screen::HomeScreen;
                } else {
                    let rec = &recs[sel_idx];
                    if rec.id.starts_with("slot:") {
                        match crate::application::ports::inbound::mark_dose_taken_port::MarkDoseTakenPort::execute(
                            &*app.services.mark_dose_taken,
                            crate::application::dtos::requests::MarkDoseTakenRequest::new(rec.medication_id.clone(), rec.scheduled_at),
                        ) {
                            Ok(_) => app.set_status("Marked scheduled slot as taken", 3000),
                            Err(e) => app.status_message = Some(format!("Error: {e}")),
                        }
                        app.load_medications();
                        app.current_screen = Screen::MedicationDetails {
                            id: rec.medication_id.clone(),
                        };
                    } else {
                        let req = MarkDoseTakenRequest::new(
                            rec.id.clone(),
                            chrono::Local::now().naive_local(),
                        );
                        match crate::application::ports::inbound::mark_dose_taken_port::MarkDoseTakenPort::execute(&*app.services.mark_dose_taken, req) {
                            Ok(_) => app.set_status("Marked as taken", 3000),
                            Err(e) => app.status_message = Some(format!("Error: {e}")),
                        }
                        app.current_screen = Screen::HomeScreen;
                        app.load_medications();
                    }
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
    use crate::application::dtos::responses::DoseRecordDto;
    use crate::presentation::tui::app::App;
    use crate::presentation::tui::app_services::AppServices;
    use crate::presentation::tui::screen::Screen;
    use chrono::NaiveDate;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    fn app_with_mark_dose(records: Vec<DoseRecordDto>) -> App {
        let mut app = App::new(AppServices::fake());
        app.current_screen = Screen::MarkDose {
            medication_id: "med-1".to_string(),
            records,
            selected_index: 0,
        };
        app
    }

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    fn dto(id: &str) -> DoseRecordDto {
        DoseRecordDto {
            id: id.to_string(),
            medication_id: "med-1".to_string(),
            scheduled_at: NaiveDate::from_ymd_opt(2025, 1, 1)
                .unwrap()
                .and_hms_opt(8, 0, 0)
                .unwrap(),
            taken_at: None,
        }
    }

    /// Verifies `MarkDoseHandler` is callable via a `Handler` trait object.
    #[test]
    fn handle_dispatches_correctly_through_trait_object() {
        let mut app = app_with_mark_dose(vec![]);
        let mut handler: Box<dyn Handler> = Box::new(MarkDoseHandler);
        handler.handle(&mut app, key(KeyCode::Esc));
        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn esc_goes_to_home() {
        let mut app = app_with_mark_dose(vec![]);
        let mut h = MarkDoseHandler;
        h.handle(&mut app, key(KeyCode::Esc));
        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn down_arrow_increments_selected_index() {
        let mut app = app_with_mark_dose(vec![dto("r1"), dto("r2")]);
        let mut h = MarkDoseHandler;

        h.handle(&mut app, key(KeyCode::Down));

        let Screen::MarkDose { selected_index, .. } = &app.current_screen else {
            panic!("expected MarkDose screen")
        };
        assert_eq!(*selected_index, 1);
    }

    #[test]
    fn j_key_increments_selected_index() {
        let mut app = app_with_mark_dose(vec![dto("r1"), dto("r2")]);
        let mut h = MarkDoseHandler;

        h.handle(&mut app, key(KeyCode::Char('j')));

        let Screen::MarkDose { selected_index, .. } = &app.current_screen else {
            panic!("expected MarkDose screen")
        };
        assert_eq!(*selected_index, 1);
    }

    #[test]
    fn down_does_not_exceed_last_index() {
        let mut app = app_with_mark_dose(vec![dto("r1")]);
        let mut h = MarkDoseHandler;

        h.handle(&mut app, key(KeyCode::Down));

        let Screen::MarkDose { selected_index, .. } = &app.current_screen else {
            panic!("expected MarkDose screen")
        };
        assert_eq!(*selected_index, 0);
    }

    #[test]
    fn up_arrow_decrements_selected_index_clamps_at_zero() {
        let mut app = app_with_mark_dose(vec![dto("r1"), dto("r2")]);
        let mut h = MarkDoseHandler;

        h.handle(&mut app, key(KeyCode::Up));

        let Screen::MarkDose { selected_index, .. } = &app.current_screen else {
            panic!("expected MarkDose screen")
        };
        assert_eq!(*selected_index, 0);
    }

    #[test]
    fn k_key_decrements_selected_index() {
        let mut app = app_with_mark_dose(vec![dto("r1"), dto("r2")]);
        if let Screen::MarkDose {
            ref mut selected_index,
            ..
        } = app.current_screen
        {
            *selected_index = 1;
        }
        let mut h = MarkDoseHandler;

        h.handle(&mut app, key(KeyCode::Char('k')));

        let Screen::MarkDose { selected_index, .. } = &app.current_screen else {
            panic!("expected MarkDose screen")
        };
        assert_eq!(*selected_index, 0);
    }

    #[test]
    fn enter_with_empty_records_sets_status_and_goes_home() {
        let mut app = app_with_mark_dose(vec![]);
        let mut h = MarkDoseHandler;

        h.handle(&mut app, key(KeyCode::Enter));

        assert!(matches!(app.current_screen, Screen::HomeScreen));
        assert!(app.status_message.is_some());
    }

    #[test]
    fn enter_with_real_record_calls_mark_dose_taken_and_goes_home() {
        let mut app = app_with_mark_dose(vec![dto("real-id")]);
        let mut h = MarkDoseHandler;

        h.handle(&mut app, key(KeyCode::Enter));

        // FakeMarkDoseTakenPort returns Ok → status message set, navigates home
        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn enter_with_slot_record_calls_mark_medication_taken() {
        let slot = DoseRecordDto {
            id: "slot:0".to_string(),
            medication_id: "med-1".to_string(),
            scheduled_at: NaiveDate::from_ymd_opt(2025, 1, 1)
                .unwrap()
                .and_hms_opt(8, 0, 0)
                .unwrap(),
            taken_at: None,
        };
        let mut app = app_with_mark_dose(vec![slot]);
        let mut h = MarkDoseHandler;

        h.handle(&mut app, key(KeyCode::Enter));

        // FakeMarkDoseTakenPort returns Ok → navigates to MedicationDetails
        assert!(matches!(
            app.current_screen,
            Screen::MedicationDetails { .. }
        ));
    }

    #[test]
    fn handler_does_nothing_when_not_on_mark_dose_screen() {
        let mut app = App::new(AppServices::fake());
        app.current_screen = Screen::HomeScreen;
        let mut h = MarkDoseHandler;

        h.handle(&mut app, key(KeyCode::Enter));

        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }
}
