use bitpill::presentation::tui::{
    renderers::{ScreenRenderer, confirm_delete_renderer::ConfirmDeleteRenderer},
    screen::Screen,
};
use ratatui::{Terminal, backend::TestBackend};

use crate::helpers::make_app;

#[test]
fn render_does_not_panic() {
    let mut t = Terminal::new(TestBackend::new(80, 24)).unwrap();
    let app = make_app(Screen::ConfirmDelete {
        id: "m1".into(),
        name: "Test".into(),
    });
    t.draw(|f| ConfirmDeleteRenderer.render(f, &app)).unwrap();
}

#[test]
fn render_with_wrong_screen_returns_early() {
    let mut t = Terminal::new(TestBackend::new(80, 24)).unwrap();
    let app = make_app(Screen::HomeScreen);
    t.draw(|f| ConfirmDeleteRenderer.render(f, &app)).unwrap();
}
