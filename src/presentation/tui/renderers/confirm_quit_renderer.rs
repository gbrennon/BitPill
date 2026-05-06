use ratatui::Frame;

use crate::presentation::tui::{
    app::App, components::modal::render_modal, renderers::ScreenRenderer,
};

pub struct ConfirmQuitRenderer;

impl ScreenRenderer for ConfirmQuitRenderer {
    fn render(&self, f: &mut Frame, app: &App) {
        render_modal(f, f.area(), "Confirm Quit", "Quit application?  (y/N)");
        let _ = app;
    }
}

#[cfg(test)]
mod tests {
    use ratatui::prelude::*;

    use super::*;
    use crate::presentation::tui::app::App;

    #[test]
    fn test_render_does_not_panic() {
        let renderer = ConfirmQuitRenderer;
        let app = App::default();

        use ratatui::{Terminal, backend::TestBackend};
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|f| {
                renderer.render(f, &app);
            })
            .unwrap();
    }
}
