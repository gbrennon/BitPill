use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::layout::Alignment;

const BORDER_COLOR: Color = Color::Rgb(214, 93, 14);
const BAR_FOREGROUND_COLOR: Color = Color::Rgb(49, 49, 49);

fn bar_style() -> Style {
    Style::default().bg(BORDER_COLOR).fg(BAR_FOREGROUND_COLOR)
}

/// Renders a vertically-centered, horizontally-centered title bar where
/// "BitPill" is bold and `subtitle` follows in normal weight.
pub fn title_bar(subtitle: &str) -> Paragraph<'_> {
    let line = Line::from(vec![
        Span::styled("BitPill", bar_style().add_modifier(Modifier::BOLD)),
        Span::styled(
            if subtitle.is_empty() {
                String::new()
            } else {
                format!("  —  {subtitle}")
            },
            bar_style(),
        ),
    ]);
    Paragraph::new(vec![Line::raw(""), Line::raw(""), line, Line::raw(""), Line::raw("")])
        .style(bar_style())
        .alignment(Alignment::Center)
}
