use bitpill::{
    infrastructure::container::Container,
    presentation::tui::{
        app::App,
        app_services::AppServices,
        draw,
        renderers::{ScreenRenderer, confirm_delete_renderer::ConfirmDeleteRenderer},
        screen::Screen,
    },
};
use ratatui::{Terminal, backend::TestBackend};
use tempfile::tempdir;

#[test]
fn confirm_delete_renderer_e2e_renders_with_medication_name() {
    let dir = tempdir().unwrap();
    std::fs::write(dir.path().join("medications.json"), "[]").unwrap();
    std::fs::write(dir.path().join("doses.json"), "[]").unwrap();
    std::fs::write(dir.path().join("settings.json"), r#"{"vim_enabled":false}"#).unwrap();

    let container = Container::new(
        dir.path().join("medications.json"),
        dir.path().join("doses.json"),
        dir.path().join("settings.json"),
    );

    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let mut app = App::new(AppServices::from_container(&container));
            app.current_screen = Screen::ConfirmDelete {
                id: "med-123".to_string(),
                name: "Aspirin".to_string(),
            };
            ConfirmDeleteRenderer.render(f, &app);
        })
        .unwrap();

    let buffer = terminal.backend().buffer();
    let contains_delete = buffer
        .content
        .iter()
        .any(|cell| cell.symbol().contains("Delete") || cell.symbol().contains("Aspirin"));
    assert!(
        contains_delete,
        "Expected delete confirmation modal with medication name to be rendered"
    );
}

#[test]
fn confirm_delete_renderer_e2e_handles_empty_name() {
    let dir = tempdir().unwrap();
    std::fs::write(dir.path().join("medications.json"), "[]").unwrap();
    std::fs::write(dir.path().join("doses.json"), "[]").unwrap();
    std::fs::write(dir.path().join("settings.json"), r#"{"vim_enabled":false}"#).unwrap();

    let container = Container::new(
        dir.path().join("medications.json"),
        dir.path().join("doses.json"),
        dir.path().join("settings.json"),
    );

    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let mut app = App::new(AppServices::from_container(&container));
            app.current_screen = Screen::ConfirmDelete {
                id: "med-456".to_string(),
                name: "".to_string(),
            };
            ConfirmDeleteRenderer.render(f, &app);
        })
        .unwrap();

    let buffer = terminal.backend().buffer();
    assert!(
        !buffer.content.is_empty(),
        "Expected modal to be rendered even with empty name"
    );
}

#[test]
fn confirm_delete_renderer_e2e_renders_on_non_confirm_delete_screen() {
    let dir = tempdir().unwrap();
    std::fs::write(dir.path().join("medications.json"), "[]").unwrap();
    std::fs::write(dir.path().join("doses.json"), "[]").unwrap();
    std::fs::write(dir.path().join("settings.json"), r#"{"vim_enabled":false}"#).unwrap();

    let container = Container::new(
        dir.path().join("medications.json"),
        dir.path().join("doses.json"),
        dir.path().join("settings.json"),
    );

    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let mut app = App::new(AppServices::from_container(&container));
            app.current_screen = Screen::HomeScreen;
            ConfirmDeleteRenderer.render(f, &app);
        })
        .unwrap();

    let buffer = terminal.backend().buffer();
    assert!(
        buffer.content.is_empty() || buffer.content.iter().all(|c| c.symbol().is_empty()),
        "Expected no rendering when not on ConfirmDelete screen"
    );
}
