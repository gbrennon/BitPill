// Renders a line for a scheduled slot or dose record with a checkbox indicating taken status.
use crate::presentation::tui::styles::highlight_style;
use chrono::NaiveDateTime;
use ratatui::text::{Line, Span};

pub fn mark_taken_line(
    selected: bool,
    h: u32,
    mm: u32,
    taken_at: Option<NaiveDateTime>,
) -> Line<'static> {
    // selection marker
    let marker = if selected { ">" } else { " " };
    // Checkbox
    let checkbox = if taken_at.is_some() { "[x]" } else { "[ ]" };
    let taken_str = match taken_at {
        Some(t) => format!(" (taken at {})", t.format("%H:%M")),
        None => "".to_string(),
    };
    Line::from(vec![
        Span::raw(format!("{} ", marker)),
        Span::styled(checkbox, highlight_style()),
        Span::raw(format!(" {:02}:{:02}{}", h, mm, taken_str)),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    #[test]
    fn untaken_slot_shows_empty_checkbox() {
        let line = mark_taken_line(false, 8, 30, None);
        let text = line
            .spans
            .iter()
            .map(|s| s.content.as_ref())
            .collect::<String>();
        assert!(text.contains("[ ]"));
        assert!(text.contains("08:30"));
    }

    #[test]
    fn taken_slot_shows_checked_checkbox_and_time() {
        let taken_at =
            NaiveDateTime::parse_from_str("2025-01-01 09:15:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let line = mark_taken_line(true, 9, 0, Some(taken_at));
        let text = line
            .spans
            .iter()
            .map(|s| s.content.as_ref())
            .collect::<String>();
        assert!(text.contains("[x]"));
        assert!(text.contains("taken at 09:15"));
        assert!(text.contains(">"));
    }
}
