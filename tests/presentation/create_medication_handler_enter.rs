use bitpill::infrastructure::container::Container;
use bitpill::presentation::tui::app::App;
use bitpill::presentation::tui::app_services::AppServices;
use bitpill::presentation::tui::handlers::create_medication_handler::CreateMedicationHandler;
use bitpill::presentation::tui::handlers::port::Handler;
use bitpill::presentation::tui::screen::Screen;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tempfile::tempdir;

#[test]
fn handle_enter_creates_medication() {
    let dir = tempdir().unwrap();
    let container = Container::new_with_paths(
        dir.path().join("medications.json"),
        dir.path().join("doses.json"),
        dir.path().join("settings.json"),
    );
    let services = AppServices::from_container(&container);
    let mut app = App::new(services);
    app.current_screen = Screen::CreateMedication {
        name: "TestMed".into(),
        amount_mg: "100".into(),
        selected_frequency: 0,
        scheduled_time: vec!["08:00".into()],
        scheduled_idx: 0,
        focused_field: 0,
        insert_mode: false,
    };

    let mut handler = CreateMedicationHandler::default();
    let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
    handler.handle(&mut app, key);

    assert!(matches!(app.current_screen, Screen::HomeScreen));
    assert_eq!(
        app.status_message.as_deref(),
        Some("Medication created successfully")
    );
}
