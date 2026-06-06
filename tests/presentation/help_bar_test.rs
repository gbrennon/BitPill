use bitpill::{
    application::dtos::responses::{DoseRecordDto, MedicationDto},
    presentation::tui::presenters::medication_details_presenter::{
        MedicationDetailsInput, MedicationDetailsPresenter,
    },
};
use chrono::NaiveDate;
use ratatui::{Terminal, backend::TestBackend};

fn make_terminal() -> Terminal<TestBackend> {
    Terminal::new(TestBackend::new(80, 24)).unwrap()
}

fn med() -> MedicationDto {
    MedicationDto {
        id: "m1".to_string(),
        name: "Aspirin".to_string(),
        amount_mg: 100,
        dose_frequency: "OnceDaily".to_string(),
        scheduled_time: vec![(8, 0)],
        taken_today: 0,
        scheduled_today: 0,
    }
}

fn dose_record() -> DoseRecordDto {
    let base = NaiveDate::from_ymd_opt(2025, 1, 1)
        .unwrap()
        .and_hms_opt(8, 0, 0)
        .unwrap();
    DoseRecordDto {
        id: "r1".to_string(),
        medication_id: "m1".to_string(),
        scheduled_at: base,
        taken_at: None,
    }
}

#[test]
fn medication_details_help_bar_shows_m_binding() {
    let mut terminal = make_terminal();
    let m = med();
    let input = MedicationDetailsInput {
        medication: Some(&m),
        records: vec![dose_record()],
    };
    terminal
        .draw(|f| MedicationDetailsPresenter.present(f, &input))
        .unwrap();
    let content: String = terminal
        .backend()
        .buffer()
        .content
        .iter()
        .map(|c| c.symbol())
        .collect();
    assert!(
        content.contains("[m] Mark scheduled slot"),
        "help bar should show [m] binding in: {content}"
    );
    assert!(
        !content.contains("[s] Mark scheduled slot"),
        "help bar should NOT show [s] binding in: {content}"
    );
    assert!(
        content.contains("[e] Edit"),
        "help bar should show [e] Edit in: {content}"
    );
    assert!(
        content.contains("[Esc] Back"),
        "help bar should show [Esc] Back in: {content}"
    );
}
