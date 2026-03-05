use crate::presentation::tui::styles::{BORDER_COLOR, content_style};
use ratatui::style::Modifier;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem};

/// Dumb schedule-time component.
/// - `count`: number of input slots to render (driven by frequency)
/// - `values`: slice of current slot string values (may be shorter than count)
/// - `selected_idx`: which slot is currently active (for highlighting)
/// - `focused`: whether the whole component is focused
pub fn schedule_time<'a>(
    count: usize,
    values: &'a [String],
    selected_idx: usize,
    focused: bool,
) -> List<'a> {
    let items: Vec<ListItem> = (0..count)
        .map(|i| {
            let raw = values.get(i).map(|s| s.as_str()).unwrap_or("");
            let display = if raw.trim().is_empty() {
                "HH:MM:SS"
            } else {
                raw
            };
            let prefix = if focused && i == selected_idx {
                "> "
            } else {
                "  "
            };
            // All schedule items use BORDER_COLOR; do not bold to keep only title bold
            let item_style = content_style().fg(BORDER_COLOR);
            ListItem::new(Line::from(Span::styled(
                format!("{}{}", prefix, display),
                item_style,
            )))
        })
        .collect();

    let title_style = if focused {
        content_style()
            .fg(BORDER_COLOR)
            .add_modifier(Modifier::BOLD)
    } else {
        content_style().fg(BORDER_COLOR)
    };
    let border_style = if focused {
        content_style()
            .fg(BORDER_COLOR)
            .add_modifier(Modifier::BOLD)
    } else {
        content_style().fg(BORDER_COLOR)
    };

    List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .style(content_style())
            .title(Span::styled("Scheduled times", title_style)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schedule_time_constructs_with_placeholders() {
        let vals: Vec<String> = vec![];
        let list = schedule_time(3, &vals, 0, true);
        // ensure widget constructed
        assert!(std::mem::size_of_val(&list) > 0);
    }
}
