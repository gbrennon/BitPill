use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::Alignment;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};

use crate::presentation::tui::styles::bar_style;

/// Renders a vertically-centered, horizontally-centered title bar where
/// "BitPill" is bold and `subtitle` follows in normal weight.
pub fn title_bar(subtitle: &str) -> Paragraph<'_> {
    let line = Line::from(vec![
        Span::styled("BitPill", crate::presentation::tui::styles::title_style()),
        Span::styled(
            if subtitle.is_empty() {
                String::new()
            } else {
                format!("  —  {subtitle}")
            },
            crate::presentation::tui::styles::bar_style(),
        ),
    ]);
    Paragraph::new(vec![line])
        .style(bar_style())
        .alignment(Alignment::Center)
}

pub fn render_title_bar(f: &mut Frame, area: Rect, subtitle: &str) {
    f.render_widget(Block::default().style(bar_style()), area);
    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Min(1),
        ])
        .split(area);
    f.render_widget(title_bar(subtitle), inner[1]);
}
