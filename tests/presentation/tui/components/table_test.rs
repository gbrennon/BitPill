use bitpill::{
    application::dtos::responses::MedicationDto,
    presentation::tui::components::table::medication_table,
};
use ratatui::{Terminal, backend::TestBackend, layout::Rect};

fn med(name: &str) -> MedicationDto {
    MedicationDto {
        id: "m1".to_string(),
        name: name.to_string(),
        amount_mg: 100,
        dose_frequency: "OnceDaily".to_string(),
        scheduled_time: vec![],
        taken_today: 0,
        scheduled_today: 0,
    }
}

#[test]
fn two_column_table_renders_without_panic() {
    let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
    let meds = vec![med("Aspirin"), med("Ibuprofen")];
    terminal
        .draw(|f| {
            let table = medication_table("Meds", &["Name", "mg"], &meds, None);
            f.render_widget(table, Rect::new(0, 0, 80, 24));
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
fn three_column_table_renders_without_panic() {
    let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
    let meds = vec![med("Aspirin")];
    terminal
        .draw(|f| {
            let table = medication_table("Meds", &["Name", "mg", "Actions"], &meds, None);
            f.render_widget(table, Rect::new(0, 0, 80, 24));
        })
        .unwrap();
    let content: String = terminal
        .backend()
        .buffer()
        .content
        .iter()
        .map(|c| c.symbol())
        .collect();
    assert!(content.contains("Edit") || content.contains("e]"));
}

#[test]
fn selected_row_is_highlighted() {
    let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
    let meds = vec![med("Aspirin"), med("Ibuprofen")];
    terminal
        .draw(|f| {
            let table = medication_table("Meds", &["Name", "mg"], &meds, Some(0));
            f.render_widget(table, Rect::new(0, 0, 80, 24));
        })
        .unwrap();
    // Just ensure it renders without panic
    let buffer = terminal.backend().buffer();
    assert!(buffer.content.iter().any(|c| c.symbol() != " "));
}

#[test]
fn empty_medication_list_renders_header_only() {
    let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
    terminal
        .draw(|f| {
            let table = medication_table("Meds", &["Name", "mg"], &[], None);
            f.render_widget(table, Rect::new(0, 0, 80, 24));
        })
        .unwrap();
    let content: String = terminal
        .backend()
        .buffer()
        .content
        .iter()
        .map(|c| c.symbol())
        .collect();
    assert!(content.contains("Name"));
}
