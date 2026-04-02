use ratatui::Frame;

use crate::presentation::tui::{
    app::App, components::modal::render_modal, renderers::ScreenRenderer,
};

pub struct ConfirmCancelRenderer;

impl ScreenRenderer for ConfirmCancelRenderer {
    fn render(&self, f: &mut Frame, app: &App) {
        render_modal(
            f,
            f.area(),
            "Discard Changes",
            "Discard changes and return?  (y/N)",
        );
        let _ = app;
    }
}

#[cfg(test)]
mod tests {
    use ratatui::{Terminal, backend::TestBackend};

    use super::*;
    use crate::presentation::tui::{app::App, app_services::AppServices};

    #[test]
    fn render_does_not_panic() {
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        let app = App::new(AppServices::fake());
        terminal
            .draw(|f| ConfirmCancelRenderer.render(f, &app))
            .unwrap();
        let buffer = terminal.backend().buffer();
        assert!(buffer.content.iter().any(|c| c.symbol() != " "));
    }
}
