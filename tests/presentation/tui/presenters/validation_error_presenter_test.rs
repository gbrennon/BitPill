use bitpill::presentation::tui::{
    presenters::validation_error_presenter::ValidationErrorPresenter, screen::Screen,
};

use crate::helpers::make_app;

#[test]
fn present_sets_validation_error_screen() {
    let mut app = make_app(Screen::HomeScreen);
    ValidationErrorPresenter.present(&mut app, vec!["error1".into()]);
    assert!(matches!(app.current_screen, Screen::ValidationError { .. }));
}
