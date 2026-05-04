use crate::presentation::tui::{
    app::App,
    handlers::port::{Handler, HandlerResult},
    screen::Screen,
};

pub struct ScheduleResultHandler;

impl Default for ScheduleResultHandler {
    fn default() -> Self {
        ScheduleResultHandler
    }
}

impl Handler for ScheduleResultHandler {
    fn handle(
        &mut self,
        app: &mut App,
        _key: crate::presentation::tui::input::Key,
    ) -> HandlerResult {
        app.current_screen = Screen::HomeScreen;
        app.load_medications();
        HandlerResult::Continue
    }
}
