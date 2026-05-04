use ratatui::Frame;

use crate::presentation::tui::{app::App, renderers};

pub fn draw(f: &mut Frame, app: &App) {
    renderers::render(f, app);
}
