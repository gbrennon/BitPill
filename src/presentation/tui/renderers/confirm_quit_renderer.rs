use crate::presentation::tui::app::App;
use crate::presentation::tui::components::modal::render_modal;
use crate::presentation::tui::renderers::ScreenRenderer;
use ratatui::Frame;

pub struct ConfirmQuitRenderer;

impl ScreenRenderer for ConfirmQuitRenderer {
    fn render(&self, f: &mut Frame, app: &App) {
        render_modal(f, f.area(), "Confirm Quit", "Quit application?  (y/N)");
        let _ = app;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::presentation::tui::app::App;
    use crate::presentation::tui::app_services::AppServices;
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    #[test]
    fn render_does_not_panic() {
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        let app = App::new(AppServices::fake());
        terminal
            .draw(|f| ConfirmQuitRenderer.render(f, &app))
            .unwrap();
        let buffer = terminal.backend().buffer();
        assert!(buffer.content.iter().any(|c| c.symbol() != " "));
    }
}
