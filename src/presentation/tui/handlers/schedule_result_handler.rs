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
    use crossterm::event::KeyCode;

    use super::*;
    use crate::presentation::tui::{app::App, app_services::AppServices};

    #[test]
    fn handle_enter_sets_screen_to_home() {
        let mut app = App::new(AppServices::fake());
        app.current_screen = Screen::HomeScreen;
        let mut handler = ScheduleResultHandler;
        let key = crate::presentation::tui::input::from_code(KeyCode::Enter);
        handler.handle(&mut app, key);
        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn handle_esc_also_sets_screen_to_home() {
        let mut app = App::new(AppServices::fake());
        app.current_screen = Screen::HomeScreen;
        let mut handler = ScheduleResultHandler;
        let key = crate::presentation::tui::input::from_code(KeyCode::Esc);
        handler.handle(&mut app, key);
        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }

    #[test]
    fn handle_via_trait_object_works() {
        let mut app = App::new(AppServices::fake());
        let mut handler: Box<dyn Handler> = Box::new(ScheduleResultHandler);
        let key = crate::presentation::tui::input::from_code(KeyCode::Enter);
        handler.handle(&mut app, key);
        assert!(matches!(app.current_screen, Screen::HomeScreen));
    }
}
