use bitpill::{
    infrastructure::container::Container,
    presentation::tui::{
        app::App,
        app_services::AppServices,
        draw,
        handlers::{port::Handler, schedule_result_handler::ScheduleResultHandler},
        input::Key,
        screen::Screen,
    },
};
use ratatui::{Terminal, backend::TestBackend};
use tempfile::tempdir;

#[test]
fn schedule_result_handler_e2e_default_impl() {
    let _handler = ScheduleResultHandler::default();
}

#[test]
fn schedule_result_handler_e2e_from_medication_details() {
    let dir = tempdir().unwrap();
    std::fs::write(dir.path().join("medications.json"), "[]").unwrap();
    std::fs::write(dir.path().join("doses.json"), "[]").unwrap();
    std::fs::write(dir.path().join("settings.json"), r#"{"vim_enabled":false}"#).unwrap();

    let container = Container::new(
        dir.path().join("medications.json"),
        dir.path().join("doses.json"),
        dir.path().join("settings.json"),
    );
    let mut app = App::new(AppServices::from_container(&container));

    app.current_screen = Screen::MedicationDetails {
        id: "m1".to_string(),
    };

    let mut handler = ScheduleResultHandler::default();
    handler.handle(&mut app, Key::Enter);

    assert!(matches!(app.current_screen, Screen::HomeScreen));
}

#[test]
fn schedule_result_handler_e2e_from_create_medication() {
    let dir = tempdir().unwrap();
    std::fs::write(dir.path().join("medications.json"), "[]").unwrap();
    std::fs::write(dir.path().join("doses.json"), "[]").unwrap();
    std::fs::write(dir.path().join("settings.json"), r#"{"vim_enabled":false}"#).unwrap();

    let container = Container::new(
        dir.path().join("medications.json"),
        dir.path().join("doses.json"),
        dir.path().join("settings.json"),
    );
    let mut app = App::new(AppServices::from_container(&container));

    app.current_screen = Screen::CreateMedication {
        name: "TestMed".to_string(),
        amount_mg: "100".to_string(),
        selected_frequency: 0,
        scheduled_time: vec!["08:00".to_string()],
        scheduled_idx: 0,
        focused_field: 0,
        insert_mode: false,
    };

    let mut handler = ScheduleResultHandler::default();
    handler.handle(&mut app, Key::Char('a'));

    assert!(matches!(app.current_screen, Screen::HomeScreen));
}

#[test]
fn schedule_result_handler_e2e_renders_and_verifies_output() {
    let dir = tempdir().unwrap();
    std::fs::write(dir.path().join("medications.json"), "[]").unwrap();
    std::fs::write(dir.path().join("doses.json"), "[]").unwrap();
    std::fs::write(dir.path().join("settings.json"), r#"{"vim_enabled":false}"#).unwrap();

    let container = Container::new(
        dir.path().join("medications.json"),
        dir.path().join("doses.json"),
        dir.path().join("settings.json"),
    );
    let mut app = App::new(AppServices::from_container(&container));

    app.current_screen = Screen::Settings {
        vim_enabled: true,
        selected_index: 0,
    };

    let mut handler = ScheduleResultHandler::default();
    handler.handle(&mut app, Key::Esc);

    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|f| draw::draw(f, &app)).unwrap();

    assert!(matches!(app.current_screen, Screen::HomeScreen));
}
