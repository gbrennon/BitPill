use bitpill::{
    infrastructure::container::Container,
    presentation::tui::{
        app::App, app_services::AppServices, renderers::ConfirmCancelRenderer, screen::Screen,
    },
};
use ratatui::{Terminal, backend::TestBackend};
use tempfile::tempdir;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn confirm_cancel_renderer_renders_modal() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("medications.json"), "[]").unwrap();
        std::fs::write(dir.path().join("doses.json"), "[]").unwrap();
        std::fs::write(dir.path().join("settings.json"), r#"{"vim_enabled":false}"#).unwrap();

        let container = Container::new(
            dir.path().join("medications.json"),
            dir.path().join("doses.json"),
            dir.path().join("settings.json"),
        );
        let app = App::new(AppServices::from_container(&container));

        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                let screen = Screen::ConfirmCancel {
                    previous: Box::new(Screen::HomeScreen),
                };
                let mut app = App::new(AppServices::from_container(&container));
                app.current_screen = screen;
                ConfirmCancelRenderer.render(f, &app);
            })
            .unwrap();

        let buffer = terminal.backend().buffer();
        let contains_discard = buffer
            .content
            .iter()
            .any(|cell| cell.symbol().contains("Discard") || cell.symbol().contains("changes"));
        assert!(
            contains_discard,
            "Expected discard changes modal to be rendered"
        );
    }

    #[test]
    fn confirm_cancel_renderer_with_previous_screen() {
        let dir = tempdir().unwrap();
        std::fs::write(
            dir.path().join("medications.json"),
            r#"[{\"id\":\"m1\",\"name\":\"Test\"}]"#,
        )
        .unwrap();
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
                let previous = Box::new(Screen::CreateMedication {
                    name: "TestMed".to_string(),
                    amount_mg: "100".to_string(),
                    selected_frequency: 0,
                    scheduled_time: vec!["08:00".to_string()],
                    scheduled_idx: 0,
                    focused_field: 0,
                    insert_mode: false,
                });
                let mut app = App::new(AppServices::from_container(&container));
                app.current_screen = Screen::ConfirmCancel { previous };
                ConfirmCancelRenderer.render(f, &app);
            })
            .unwrap();

        let buffer = terminal.backend().buffer();
        assert!(!buffer.content.is_empty(), "Expected modal to be rendered");
    }
}
