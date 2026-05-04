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
