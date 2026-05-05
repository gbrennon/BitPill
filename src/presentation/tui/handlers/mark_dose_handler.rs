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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::presentation::tui::{app::App, input::Key};

    #[test]
    fn mark_dose_navigation_and_enter() {
        let mut h = MarkDoseHandler::default();
        let mut app = App::default();
        let rec = crate::application::dtos::responses::DoseRecordDto {
            id: "slot:0".into(),
            medication_id: "m1".into(),
            scheduled_at: chrono::Local::now().naive_local(),
            taken_at: None,
        };
        app.current_screen = Screen::MarkDose {
            medication_id: "m1".into(),
            records: vec![rec.clone()],
            selected_index: 0,
        };

        // Enter on slot should not panic and should transition
        h.handle(&mut app, Key::Enter);
        assert!(matches!(
            app.current_screen,
            Screen::MedicationDetails { .. } | Screen::HomeScreen | Screen::MarkDose { .. }
        ));

        // navigation j/k
        app.current_screen = Screen::MarkDose {
            medication_id: "m1".into(),
            records: vec![rec.clone(), rec.clone()],
            selected_index: 0,
        };
        h.handle(&mut app, Key::Char('j'));
        assert!(matches!(app.current_screen, Screen::MarkDose { .. }));
        h.handle(&mut app, Key::Char('k'));
        assert!(matches!(app.current_screen, Screen::MarkDose { .. }));
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

struct FakeMarkDoseOk;
impl crate::application::ports::inbound::mark_dose_taken_port::MarkDoseTakenPort
    for FakeMarkDoseOk
{
    fn execute(
        &self,
        _: crate::application::dtos::requests::MarkDoseTakenRequest,
    ) -> Result<
        crate::application::dtos::responses::MarkDoseTakenResponse,
        crate::application::errors::ApplicationError,
    > {
        Ok(crate::application::dtos::responses::MarkDoseTakenResponse::new("ok"))
    }
}

fn make_rec(id: &str, med_id: &str) -> crate::application::dtos::responses::DoseRecordDto {
    crate::application::dtos::responses::DoseRecordDto {
        id: id.into(),
        medication_id: med_id.into(),
        scheduled_at: chrono::Local::now().naive_local(),
        taken_at: None,
    }
}

fn make_slot(id: &str, med_id: &str) -> crate::application::dtos::responses::DoseRecordDto {
    crate::application::dtos::responses::DoseRecordDto {
        id: format!("slot:{}", id),
        medication_id: med_id.into(),
        scheduled_at: chrono::Local::now().naive_local(),
        taken_at: None,
    }
}

// --- Non-MarkDose screen ---
#[test]
fn non_mark_dose_screen_returns_continue() {
    let mut h = MarkDoseHandler::default();
    let mut a = App::default();
    a.current_screen = Screen::HomeScreen;
    assert!(matches!(
        h.handle(&mut a, Key::Char('x')),
        HandlerResult::Continue
    ));
}

// --- Emacs mode ---
#[test]
fn emacs_n_moves_down() {
    let mut h = MarkDoseHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("emacs"));
    let recs = vec![make_rec("r1", "m1"), make_rec("r2", "m1")];
    a.current_screen = Screen::MarkDose {
        medication_id: "m1".into(),
        records: recs,
        selected_index: 0,
    };
    h.handle(&mut a, Key::Char('n'));
    if let Screen::MarkDose { selected_index, .. } = &a.current_screen {
        assert_eq!(*selected_index, 1);
    }
}
#[test]
fn emacs_p_moves_up() {
    let mut h = MarkDoseHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("emacs"));
    let recs = vec![make_rec("r1", "m1"), make_rec("r2", "m1")];
    a.current_screen = Screen::MarkDose {
        medication_id: "m1".into(),
        records: recs,
        selected_index: 1,
    };
    h.handle(&mut a, Key::Char('p'));
    if let Screen::MarkDose { selected_index, .. } = &a.current_screen {
        assert_eq!(*selected_index, 0);
    }
}
#[test]
fn emacs_f_moves_down() {
    let mut h = MarkDoseHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("emacs"));
    let recs = vec![make_rec("r1", "m1"), make_rec("r2", "m1")];
    a.current_screen = Screen::MarkDose {
        medication_id: "m1".into(),
        records: recs,
        selected_index: 0,
    };
    h.handle(&mut a, Key::Char('f'));
    if let Screen::MarkDose { selected_index, .. } = &a.current_screen {
        assert_eq!(*selected_index, 1);
    }
}
#[test]
fn emacs_b_moves_up() {
    let mut h = MarkDoseHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("emacs"));
    let recs = vec![make_rec("r1", "m1"), make_rec("r2", "m1")];
    a.current_screen = Screen::MarkDose {
        medication_id: "m1".into(),
        records: recs,
        selected_index: 1,
    };
    h.handle(&mut a, Key::Char('b'));
    if let Screen::MarkDose { selected_index, .. } = &a.current_screen {
        assert_eq!(*selected_index, 0);
    }
}
#[test]
fn emacs_skip_jk() {
    let mut h = MarkDoseHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("emacs"));
    let recs = vec![make_rec("r1", "m1")];
    a.current_screen = Screen::MarkDose {
        medication_id: "m1".into(),
        records: recs,
        selected_index: 0,
    };
    for key in [Key::Char('j'), Key::Char('k')] {
        assert!(matches!(h.handle(&mut a, key), HandlerResult::Continue));
    }
}

// --- Vim mode ---
#[test]
fn vim_esc_returns_home() {
    let mut h = MarkDoseHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("vi"));
    let recs = vec![make_rec("r1", "m1")];
    a.current_screen = Screen::MarkDose {
        medication_id: "m1".into(),
        records: recs,
        selected_index: 0,
    };
    h.handle(&mut a, Key::Esc);
    assert!(matches!(a.current_screen, Screen::HomeScreen));
}
#[test]
fn vim_down_moves_down() {
    let mut h = MarkDoseHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("vi"));
    let recs = vec![make_rec("r1", "m1"), make_rec("r2", "m1")];
    a.current_screen = Screen::MarkDose {
        medication_id: "m1".into(),
        records: recs,
        selected_index: 0,
    };
    h.handle(&mut a, Key::Down);
    if let Screen::MarkDose { selected_index, .. } = &a.current_screen {
        assert_eq!(*selected_index, 1);
    }
}
#[test]
fn vim_up_moves_up() {
    let mut h = MarkDoseHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("vi"));
    let recs = vec![make_rec("r1", "m1"), make_rec("r2", "m1")];
    a.current_screen = Screen::MarkDose {
        medication_id: "m1".into(),
        records: recs,
        selected_index: 1,
    };
    h.handle(&mut a, Key::Up);
    if let Screen::MarkDose { selected_index, .. } = &a.current_screen {
        assert_eq!(*selected_index, 0);
    }
}
#[test]
fn vim_enter_empty_shows_status() {
    let mut h = MarkDoseHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("vi"));
    a.current_screen = Screen::MarkDose {
        medication_id: "m1".into(),
        records: vec![],
        selected_index: 0,
    };
    h.handle(&mut a, Key::Enter);
    assert!(a.status_message.is_some());
    assert!(matches!(a.current_screen, Screen::HomeScreen));
}
#[test]
fn vim_enter_slot_marks() {
    let mut h = MarkDoseHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("vi"));
    a.services.mark_dose_taken = Arc::new(FakeMarkDoseOk);
    let recs = vec![make_slot("0", "m1")];
    a.current_screen = Screen::MarkDose {
        medication_id: "m1".into(),
        records: recs,
        selected_index: 0,
    };
    h.handle(&mut a, Key::Enter);
    assert!(
        matches!(a.current_screen, Screen::MedicationDetails { .. })
            || matches!(a.current_screen, Screen::HomeScreen)
    );
}
#[test]
fn vim_enter_non_slot_marks() {
    let mut h = MarkDoseHandler::default();
    let mut a = App::default();
    a.services.get_settings = Arc::new(FakeSettings("vi"));
    a.services.mark_dose_taken = Arc::new(FakeMarkDoseOk);
    let recs = vec![make_rec("r1", "m1")];
    a.current_screen = Screen::MarkDose {
        medication_id: "m1".into(),
        records: recs,
        selected_index: 0,
    };
    h.handle(&mut a, Key::Enter);
    assert!(a.status_message.is_some());
}
