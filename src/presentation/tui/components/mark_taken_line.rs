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
