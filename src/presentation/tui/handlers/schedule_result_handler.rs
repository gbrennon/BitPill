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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::presentation::tui::{app::App, input::Key};

    #[test]
    fn schedule_result_brings_home_and_loads() {
        let mut h = ScheduleResultHandler::default();
        let mut app = App::default();
        app.current_screen = Screen::HomeScreen;
        h.handle(&mut app, Key::Char('x'));
        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }
}
