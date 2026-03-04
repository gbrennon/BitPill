// styles/mod.rs: Centralized TUI style definitions
use ratatui::style::{Color, Modifier, Style};

pub const BACKGROUND_COLOR: Color = Color::Rgb(28, 27, 26);
pub const COPY_COLOR: Color = Color::Rgb(217, 206, 195);
pub const BAR_TEXT_COLOR: Color = Color::Rgb(49, 49, 49);
pub const BORDER_COLOR: Color = Color::Rgb(214, 93, 14);
pub const TOP_BAR_HEIGHT: u16 = 5;

pub fn bar_style() -> Style {
    Style::default().bg(BORDER_COLOR).fg(BAR_TEXT_COLOR)
}

pub fn title_style() -> Style {
    // Stronger emphasis for title (visually larger via bold modifier)
    bar_style().add_modifier(Modifier::BOLD)
}

pub fn content_style() -> Style {
    Style::default().bg(BACKGROUND_COLOR).fg(COPY_COLOR)
}

pub fn highlight_style() -> Style {
    Style::default()
        .fg(BORDER_COLOR)
        .add_modifier(Modifier::BOLD)
}
