use bitpill::{
    infrastructure::container::Container,
    presentation::tui::{
        app::App, app_services::AppServices, draw,
        presenters::validation_error_presenter::ValidationErrorPresenter, screen::Screen,
    },
};
use ratatui::{Terminal, backend::TestBackend};
use tempfile::tempdir;

#[test]
fn validation_error_presenter_e2e_presents_error_screen() {
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

    let previous = app.current_screen.clone();
    ValidationErrorPresenter.present(&mut app, vec!["Error 1".to_string(), "Error 2".to_string()]);

    assert!(
        matches!(app.current_screen, Screen::ValidationError { messages, previous: _ }
        if messages.len() == 2 && messages[0] == "Error 1" && messages[1] == "Error 2")
    );
}

#[test]
fn validation_error_presenter_e2e_renders_error_modal() {
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
        name: "Test".to_string(),
        amount_mg: "100".to_string(),
        selected_frequency: 0,
        scheduled_time: vec!["08:00".to_string()],
        scheduled_idx: 0,
        focused_field: 0,
        insert_mode: false,
    };

    ValidationErrorPresenter.present(&mut app, vec!["Test error".to_string()]);

    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|f| draw::draw(f, &app)).unwrap();

    let buffer = terminal.backend().buffer();
    let contains_validation = buffer.content.iter().any(|cell| {
        cell.symbol().contains("Validation")
            || cell.symbol().contains("error")
            || cell.symbol().contains("Test error")
    });
    assert!(
        contains_validation,
        "Expected validation error modal to be rendered"
    );
}
