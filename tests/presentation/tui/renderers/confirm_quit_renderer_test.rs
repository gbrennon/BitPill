use bitpill::presentation::tui::{
    renderers::{ScreenRenderer, confirm_quit_renderer::ConfirmQuitRenderer},
    screen::Screen,
};
use ratatui::{Terminal, backend::TestBackend};

use crate::helpers::make_app;

#[test]
fn render_does_not_panic() {
    let mut t = Terminal::new(TestBackend::new(80, 24)).unwrap();
    let prev = Box::new(Screen::HomeScreen);
    let app = make_app(Screen::ConfirmQuit { previous: prev });
    t.draw(|f| ConfirmQuitRenderer.render(f, &app)).unwrap();
}
