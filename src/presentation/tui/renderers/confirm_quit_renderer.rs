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
