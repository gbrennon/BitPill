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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::presentation::tui::{app::App, screen::Screen};

    #[test]
    fn test_present_sets_validation_error_screen() {
        let presenter = ValidationErrorPresenter;
        let mut app = App::default();
        let messages = vec!["Error 1".to_string(), "Error 2".to_string()];
        presenter.present(&mut app, messages.clone());
        if let Screen::ValidationError { messages: m, .. } = &app.current_screen {
            assert_eq!(m, &messages);
        } else {
            panic!("Screen was not set to ValidationError");
        }
    }
}
