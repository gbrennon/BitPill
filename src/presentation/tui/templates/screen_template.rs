use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};

use crate::presentation::tui::components::bottom_bar::bottom_bar;
use crate::presentation::tui::components::title_bar::render_title_bar;
use crate::presentation::tui::styles::TOP_BAR_HEIGHT;

/// Base screen template — the equivalent of a web base layout.
///
/// Defines the three permanent zones shared by every screen:
///
/// ```text
/// ┌─────────────────────────────────┐
/// │  Title bar  (TOP_BAR_HEIGHT)    │  ← always rendered by the template
/// ├─────────────────────────────────┤
/// │                                 │
/// │  Content  (Min 0)               │  ← caller fills via `render_content`
/// │                                 │
/// ├─────────────────────────────────┤
/// │  Help / status bar  (1 line)    │  ← always rendered by the template
/// └─────────────────────────────────┘
/// ```
///
/// # Example
/// ```ignore
/// ScreenTemplate { subtitle: "Medications", help: "[q] Quit" }
///     .render(f, |f, content_area| {
///         f.render_widget(my_widget, content_area);
///     });
/// ```
pub struct ScreenTemplate<'a> {
    /// Screen-specific subtitle shown after "BitPill  —  " in the title bar.
    pub subtitle: &'a str,
    /// Text shown in the bottom bar (keybinding hints or status message).
    pub help: &'a str,
    /// Current input mode label (eg. "NORMAL" or "INSERT").
    pub mode: &'a str,
}

impl<'a> ScreenTemplate<'a> {
    /// Render the chrome (title + help bar) and delegate content to the caller.
    ///
    /// `render_content` receives the `Rect` of the content zone so it can fill it.
    /// If the terminal is too short for the help bar, the template falls back to
    /// rendering the help text in the last row of the content zone.
    pub fn render<F>(&self, f: &mut Frame, render_content: F)
    where
        F: FnOnce(&mut Frame, Rect),
    {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(TOP_BAR_HEIGHT),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(f.area());

        render_title_bar(f, chunks[0], self.subtitle);
        render_content(f, chunks[1]);

        let combined = format!("{}  |  MODE: {}", self.help, self.mode);
        let help_widget = bottom_bar(&combined);
        if chunks[2].height > 0 {
            f.render_widget(help_widget, chunks[2]);
        } else {
            // Fallback: paint help text in the last row of the content zone
            // so it is always visible regardless of terminal height.
            let y = chunks[1].y + chunks[1].height.saturating_sub(1);
            let alt = Rect::new(chunks[1].x, y, chunks[1].width, 1);
            f.render_widget(help_widget, alt);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

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
}
