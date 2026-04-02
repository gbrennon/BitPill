use ratatui::Frame;

use crate::presentation::tui::{
    app::App, components::modal::render_modal, renderers::ScreenRenderer, screen::Screen,
};

pub struct ConfirmDeleteRenderer;

impl ScreenRenderer for ConfirmDeleteRenderer {
    fn render(&self, f: &mut Frame, app: &App) {
        let Screen::ConfirmDelete { name, .. } = &app.current_screen else {
            return;
        };

        let content = format!("Delete medication '{}'?  (y/N)", name);
        render_modal(f, f.area(), "Confirm Delete", &content);
    }
}

#[cfg(test)]
mod tests {
    use ratatui::{Terminal, backend::TestBackend};

    use super::*;
    use crate::presentation::tui::{app::App, app_services::AppServices};

    #[test]
    fn render_on_confirm_delete_screen_does_not_panic() {
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        let mut app = App::new(AppServices::fake());
        app.current_screen = Screen::ConfirmDelete {
            id: "id1".to_string(),
            name: "Aspirin".to_string(),
        };
        terminal
            .draw(|f| ConfirmDeleteRenderer.render(f, &app))
            .unwrap();
        let buffer = terminal.backend().buffer();
        assert!(buffer.content.iter().any(|c| c.symbol() != " "));
    }

    #[test]
    fn render_on_wrong_screen_returns_without_panic() {
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        let app = App::new(AppServices::fake());
        // HomeScreen — guard clause should return early
        terminal
            .draw(|f| ConfirmDeleteRenderer.render(f, &app))
            .unwrap();
    }
}
