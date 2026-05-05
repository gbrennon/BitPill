use bitpill::{
    application::dtos::responses::MedicationDto,
    presentation::tui::components::detail::medication_detail,
};
use chrono::NaiveDate;
use ratatui::{Terminal, backend::TestBackend, layout::Rect};

fn med(with_schedule: bool) -> MedicationDto {
    MedicationDto {
        id: "m1".to_string(),
        name: "Aspirin".to_string(),
        amount_mg: 100,
        dose_frequency: "OnceDaily".to_string(),
        scheduled_time: if with_schedule { vec![(8, 0)] } else { vec![] },
        taken_today: 0,
        scheduled_today: 0,
    }
}

fn ts() -> chrono::NaiveDateTime {
    NaiveDate::from_ymd_opt(2025, 1, 1)
        .unwrap()
        .and_hms_opt(8, 0, 0)
        .unwrap()
}

#[test]
fn medication_detail_with_no_taken_renders() {
    let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
    let m = med(true);
    terminal
        .draw(|f| {
            let widget = medication_detail(&m, &[]);
            f.render_widget(widget, Rect::new(0, 0, 80, 24));
        })
        .unwrap();
    let content: String = terminal
        .backend()
        .buffer()
        .content
        .iter()
        .map(|c| c.symbol())
        .collect();
    assert!(content.contains("Aspirin"));
}

#[test]
fn medication_detail_with_taken_renders_timestamp() {
    let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
    let m = med(false);
    let taken = [ts()];
    terminal
        .draw(|f| {
            let widget = medication_detail(&m, &taken);
            f.render_widget(widget, Rect::new(0, 0, 80, 24));
        })
        .unwrap();
    let content: String = terminal
        .backend()
        .buffer()
        .content
        .iter()
        .map(|c| c.symbol())
        .collect();
    assert!(content.contains("Taken: 1 time(s)") || content.contains("2025"));
}

#[test]
fn medication_detail_no_schedule_shows_no_schedule() {
    let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
    let m = med(false);
    terminal
        .draw(|f| {
            let widget = medication_detail(&m, &[]);
            f.render_widget(widget, Rect::new(0, 0, 80, 24));
        })
        .unwrap();
    let content: String = terminal
        .backend()
        .buffer()
        .content
        .iter()
        .map(|c| c.symbol())
        .collect();
    assert!(content.contains("no schedule"));
}
