use ratatui::{
    prelude::Alignment,
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::presentation::tui::styles::bar_style;

/// Renders a help/status bar that applies its own styles so presenters only provide content.
pub fn bottom_bar(content: &str) -> Paragraph<'_> {
    let line = Line::from(vec![Span::styled(content.to_string(), bar_style())]);
    Paragraph::new(line)
        .style(bar_style())
        .alignment(Alignment::Center)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bottom_bar_constructs() {
        let p = bottom_bar(" [c] Create  [s] Schedule  [q] Quit");
        assert!(std::mem::size_of_val(&p) > 0);
    }
}
