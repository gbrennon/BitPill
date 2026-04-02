use ratatui::Frame;

use crate::presentation::tui::{app::App, renderers};

pub fn draw(f: &mut Frame, app: &App) {
    renderers::render(f, app);
}

#[cfg(test)]
mod tests {
    use ratatui::{Terminal, backend::TestBackend};

    use super::*;
    use crate::presentation::tui::{app::App, app_services::AppServices};

    #[test]
    fn draw_home_screen_does_not_panic() {
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        let app = App::new(AppServices::fake());
        terminal.draw(|f| draw(f, &app)).unwrap();
        let buffer = terminal.backend().buffer();
        assert!(buffer.content.iter().any(|c| c.symbol() != " "));
    }
}
