use crate::presentation::tui::app::App;
use crate::presentation::tui::components::modal::render_modal;
use crate::presentation::tui::renderers::ScreenRenderer;
use crate::presentation::tui::screen::Screen;
use ratatui::Frame;

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
