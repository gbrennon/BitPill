use crate::presentation::tui::app::App;
use crate::presentation::tui::components::modal::render_modal;
use crate::presentation::tui::renderers::ScreenRenderer;
use ratatui::Frame;

pub struct ConfirmCancelRenderer;

impl ScreenRenderer for ConfirmCancelRenderer {
    fn render(&self, f: &mut Frame, app: &App) {
        render_modal(f, f.area(), "Discard Changes", "Discard changes and return?  (y/N)");
        let _ = app;
    }
}
