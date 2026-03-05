use crate::presentation::tui::app::App;
use crate::presentation::tui::renderers;
use ratatui::Frame;

pub fn draw(f: &mut Frame, app: &App) {
    renderers::render(f, app);
}
