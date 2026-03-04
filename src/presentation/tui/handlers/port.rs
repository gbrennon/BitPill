use crate::presentation::tui::app::App;
use crossterm::event::KeyEvent;

pub enum HandlerResult {
    Continue,
}

pub trait Handler {
    fn handle(&mut self, app: &mut App, key: KeyEvent) -> HandlerResult;
}
