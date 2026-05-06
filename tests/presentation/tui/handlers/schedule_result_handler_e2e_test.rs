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
fn schedule_result_handler_e2e_handles_key_and_returns_home() {
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

    app.current_screen = Screen::MarkDose {
        medication_id: "m1".to_string(),
        records: vec![],
        selected_index: 0,
    };

    let mut handler = ScheduleResultHandler::default();
    let result = handler.handle(&mut app, Key::Enter);

    assert!(matches!(
        result,
        bitpill::presentation::tui::handlers::port::HandlerResult::Continue
    ));
    assert!(matches!(app.current_screen, Screen::HomeScreen));
}

#[test]
fn schedule_result_handler_e2e_renders_terminal_after_handle() {
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

    app.current_screen = Screen::ConfirmQuit {
        previous: Box::new(Screen::HomeScreen),
    };

    let mut handler = ScheduleResultHandler::default();
    handler.handle(&mut app, Key::Enter);

    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|f| draw::draw(f, &app)).unwrap();

    let buffer = terminal.backend().buffer();
    assert!(!buffer.content.is_empty());
}
