use crate::presentation::tui::app::App;
use crate::presentation::tui::handlers::port::{Handler, HandlerResult};
use crate::presentation::tui::screen::Screen;

pub struct ScheduleResultHandler;

impl Default for ScheduleResultHandler {
    fn default() -> Self {
        ScheduleResultHandler
    }
}

impl Handler for ScheduleResultHandler {
    fn handle(&mut self, app: &mut App, _key: crossterm::event::KeyEvent) -> HandlerResult {
        app.current_screen = Screen::HomeScreen;
        app.load_medications();
        HandlerResult::Continue
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    #[test]
    fn handle_sets_screen_to_medication_list() {
        let container = std::sync::Arc::new(crate::infrastructure::container::Container::new());
        let mut app = App::new(container);
        app.current_screen = Screen::HomeScreen;
        let mut handler = ScheduleResultHandler;
        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
        handler.handle(&mut app, key);
        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }
}
