use bitpill::infrastructure::container::Container;
use bitpill::presentation::tui::app::App;
use bitpill::presentation::tui::app_services::AppServices;
use bitpill::presentation::tui::draw;
use ratatui::Terminal;
use ratatui::backend::TestBackend;
use tempfile::tempdir;

#[test]
fn medication_list_screen_renders_items_e2e() {
    let dir = tempdir().unwrap();
    let container = Container::new(
        dir.path().join("medications.json"),
        dir.path().join("doses.json"),
        dir.path().join("settings.json"),
    );
    let mut app = App::new(AppServices::from_container(&container));
    app.medications
        .push(bitpill::application::dtos::responses::MedicationDto {
            id: "id-1".to_string(),
            name: "Aspirin".to_string(),
            amount_mg: 500,
            scheduled_time: vec![],
            dose_frequency: "OnceDaily".to_string(),
            taken_today: 0,
            scheduled_today: 0,
        });
    app.medications
        .push(bitpill::application::dtos::responses::MedicationDto {
            id: "id-2".to_string(),
            name: "Ibuprofen".to_string(),
            amount_mg: 200,
            scheduled_time: vec![],
            dose_frequency: "OnceDaily".to_string(),
            taken_today: 0,
            scheduled_today: 0,
        });
    app.selected_index = 0;
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    // Act
    terminal.draw(|f| draw::draw(f, &app)).unwrap();

    // Assert
    let buffer = terminal.backend().buffer();
    assert!(!buffer.content.is_empty());
    let contains_chars = buffer.content.iter().any(|cell| !cell.symbol().is_empty());
    assert!(contains_chars);
}
