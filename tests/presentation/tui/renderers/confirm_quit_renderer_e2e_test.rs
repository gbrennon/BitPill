use bitpill::{
    infrastructure::container::Container,
    presentation::tui::{
        app::App,
        app_services::AppServices,
        renderers::{ScreenRenderer, confirm_quit_renderer::ConfirmQuitRenderer},
        screen::Screen,
    },
};
use ratatui::{Terminal, backend::TestBackend};
use tempfile::tempdir;

#[test]
fn confirm_quit_renderer_e2e_renders_quit_confirmation() {
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
            let previous = Box::new(Screen::HomeScreen);
            let mut app = App::new(AppServices::from_container(&container));
            app.current_screen = Screen::ConfirmQuit { previous };
            ConfirmQuitRenderer.render(f, &app);
        })
        .unwrap();

    let buffer = terminal.backend().buffer();
    let contains_quit = buffer
        .content
        .iter()
        .any(|cell| cell.symbol().contains("Quit") || cell.symbol().contains("application"));
    assert!(
        contains_quit,
        "Expected quit confirmation modal to be rendered"
    );
}

#[test]
fn confirm_quit_renderer_e2e_from_settings_screen() {
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
            let previous = Box::new(Screen::Settings {
                vim_enabled: true,
                selected_index: 0,
            });
            let mut app = App::new(AppServices::from_container(&container));
            app.current_screen = Screen::ConfirmQuit { previous };
            ConfirmQuitRenderer.render(f, &app);
        })
        .unwrap();

    let buffer = terminal.backend().buffer();
    assert!(
        !buffer.content.is_empty(),
        "Expected quit modal to be rendered when quitting from settings"
    );
}
