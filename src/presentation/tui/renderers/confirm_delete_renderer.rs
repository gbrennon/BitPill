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
