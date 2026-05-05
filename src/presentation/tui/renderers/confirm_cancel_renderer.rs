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
    use ratatui::prelude::*;

    use super::*;
    use crate::presentation::tui::app::App;

    struct DummyFrame;
    impl DummyFrame {
        fn new() -> Self {
            DummyFrame
        }
    }

    #[test]
    fn test_struct_exists() {
        let _ = ConfirmCancelRenderer;
    }

    #[test]
    fn test_render_does_not_panic() {
        let renderer = ConfirmCancelRenderer;
        let mut app = App::default();

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
