use bitpill::presentation::tui::components::mark_taken_line::mark_taken_line;
use chrono::NaiveDateTime;

#[test]
fn untaken_slot_shows_empty_checkbox() {
    let line = mark_taken_line(false, 8, 30, None);
    let text = line
        .spans
        .iter()
        .map(|s| s.content.as_ref())
        .collect::<String>();
    assert!(text.contains("[ ]"));
    assert!(text.contains("08:30"));
}

#[test]
fn taken_slot_shows_checked_checkbox_and_time() {
    let taken_at =
        NaiveDateTime::parse_from_str("2025-01-01 09:15:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let line = mark_taken_line(true, 9, 0, Some(taken_at));
    let text = line
        .spans
        .iter()
        .map(|s| s.content.as_ref())
        .collect::<String>();
    assert!(text.contains("[x]"));
    assert!(text.contains("taken at 09:15"));
    assert!(text.contains(">"));
}
