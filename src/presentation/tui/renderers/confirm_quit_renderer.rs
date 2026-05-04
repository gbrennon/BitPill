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
