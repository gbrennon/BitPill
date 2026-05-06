use bitpill::presentation::tui::templates::screen_template::ScreenTemplate;
use ratatui::{Terminal, backend::TestBackend};

#[test]
fn render_normal_height_no_panic() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|f| {
            ScreenTemplate {
                subtitle: "Test",
                help: "[q] Quit",
                mode: "NORMAL",
            }
            .render(f, |_, _| {});
        })
        .unwrap();
}

#[test]
fn render_very_short_terminal_uses_fallback_help_bar() {
    // Height of 1 forces chunks[2].height == 0 (TOP_BAR_HEIGHT=5 consumes all space),
    // exercising the fallback render branch (lines 67-72).
    let backend = TestBackend::new(80, 1);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|f| {
            ScreenTemplate {
                subtitle: "Test",
                help: "[q] Quit",
                mode: "NORMAL",
            }
            .render(f, |_, _| {});
        })
        .unwrap();
}
