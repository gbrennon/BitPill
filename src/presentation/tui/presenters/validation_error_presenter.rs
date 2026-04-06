use crate::presentation::tui::{app::App, screen::Screen};

pub struct ValidationErrorPresenter;

impl ValidationErrorPresenter {
    pub fn present(&self, app: &mut App, messages: Vec<String>) {
        app.current_screen = Screen::ValidationError {
            messages,
            previous: Box::new(app.current_screen.clone()),
        };
    }
}
