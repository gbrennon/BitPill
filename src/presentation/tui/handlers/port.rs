use crate::presentation::tui::{app::App, input::Key};

pub enum HandlerResult {
    Continue,
}

pub trait Handler {
    fn handle(&mut self, app: &mut App, key: Key) -> HandlerResult;
}
