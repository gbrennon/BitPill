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

fn med(has_schedule: bool) -> MedicationDto {
    MedicationDto {
        id: "m1".to_string(),
        name: "Aspirin".to_string(),
        amount_mg: 100,
        dose_frequency: "OnceDaily".to_string(),
        scheduled_time: if has_schedule { vec![(8, 0)] } else { vec![] },
        taken_today: 0,
        scheduled_today: 0,
    }
}

fn dose_record(taken: bool) -> DoseRecordDto {
    let base = NaiveDate::from_ymd_opt(2025, 1, 1)
        .unwrap()
        .and_hms_opt(8, 0, 0)
        .unwrap();
    DoseRecordDto {
        id: "r1".to_string(),
        medication_id: "m1".to_string(),
        scheduled_at: base,
        taken_at: if taken { Some(base) } else { None },
    }
}

#[test]
fn present_with_medication_does_not_panic() {
    let mut terminal = make_terminal();
    let m = med(true);
    let input = MedicationDetailsInput {
        medication: Some(&m),
        records: vec![dose_record(false)],
    };
    terminal
        .draw(|f| MedicationDetailsPresenter.present(f, &input))
        .unwrap();
    let buffer = terminal.backend().buffer();
    assert!(buffer.content.iter().any(|c| c.symbol() != " "));
}

#[test]
fn present_with_no_medication_renders_not_found() {
    let mut terminal = make_terminal();
    let input = MedicationDetailsInput {
        medication: None,
        records: vec![],
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
    assert!(content.contains("not found"));
}

#[test]
fn present_with_taken_record_does_not_panic() {
    let mut terminal = make_terminal();
    let m = med(true);
    let input = MedicationDetailsInput {
        medication: Some(&m),
        records: vec![dose_record(true)],
    };
    terminal
        .draw(|f| MedicationDetailsPresenter.present(f, &input))
        .unwrap();
}

#[test]
fn present_medication_no_schedule_and_no_records_does_not_panic() {
    let mut terminal = make_terminal();
    let m = med(false);
    let input = MedicationDetailsInput {
        medication: Some(&m),
        records: vec![],
    };
    terminal
        .draw(|f| MedicationDetailsPresenter.present(f, &input))
        .unwrap();
}

#[test]
fn present_medication_custom_frequency_renders() {
    let mut terminal = make_terminal();
    let m = MedicationDto {
        dose_frequency: "Custom".to_string(),
        ..med(false)
    };
    let input = MedicationDetailsInput {
        medication: Some(&m),
        records: vec![],
    };
    terminal
        .draw(|f| MedicationDetailsPresenter.present(f, &input))
        .unwrap();
}
