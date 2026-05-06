use bitpill::presentation::tui::{
    renderers::{ScreenRenderer, confirm_cancel_renderer::ConfirmCancelRenderer},
    screen::Screen,
};
use ratatui::{Terminal, backend::TestBackend};

use crate::helpers::make_app;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_does_not_panic() {
        let mut t = Terminal::new(TestBackend::new(80, 24)).unwrap();
        let prev = Box::new(Screen::HomeScreen);
        let app = make_app(Screen::ConfirmCancel { previous: prev });
        t.draw(|f| ConfirmCancelRenderer.render(f, &app)).unwrap();
    }
}
