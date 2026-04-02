use bitpill::{
    application::dtos::responses::DoseRecordDto,
    presentation::tui::{
        presenters::mark_dose_presenter::build_mark_dose_lines, styles::highlight_style,
    },
};
use chrono::{NaiveDate, NaiveDateTime};
use ratatui::text::{Line, Span};

fn make_dt(h: u32, m: u32) -> NaiveDateTime {
    NaiveDate::from_ymd_opt(2025, 1, 1)
        .unwrap()
        .and_hms_opt(h, m, 0)
        .unwrap()
}

#[test]
fn headers_are_styled_and_grouped() {
    let rec1 = DoseRecordDto {
        id: "r1".to_string(),
        medication_id: "med".to_string(),
        scheduled_at: make_dt(8, 0),
        taken_at: None,
    };
    let rec2 = DoseRecordDto {
        id: "slot:0".to_string(),
        medication_id: "med".to_string(),
        scheduled_at: make_dt(9, 0),
        taken_at: None,
    };
    let input = [rec1, rec2];
    let lines = build_mark_dose_lines(&input, 0);
    // expect Registered records header as styled span first
    let expected = Line::from(Span::styled("Registered records:", highlight_style()));
    assert_eq!(lines[0], expected);
    // scheduled slots header present later
    let expected2 = Line::from(Span::styled("Scheduled slots:", highlight_style()));
    assert!(lines.iter().any(|l| l == &expected2));
}

#[test]
fn no_records_message() {
    let lines = build_mark_dose_lines(&[], 0);
    assert_eq!(lines.len(), 1);
    assert!(lines[0].to_string().contains("No dose records"));
}
