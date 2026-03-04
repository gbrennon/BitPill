use crate::application::ports::inbound::list_all_medications_port::MedicationDto;
use crate::presentation::tui::styles::content_style;
use chrono::NaiveDateTime;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

pub fn medication_detail<'a>(m: &'a MedicationDto, taken_at: &[NaiveDateTime]) -> Paragraph<'a> {
    let mut lines = Vec::new();
    lines.push(Line::from(Span::styled(
        format!("Name: {}", m.name),
        content_style(),
    )));
    lines.push(Line::from(Span::styled(
        format!("Dose: {}mg", m.amount_mg),
        content_style(),
    )));
    let times = if m.scheduled_time.is_empty() {
        "(no schedule)".to_string()
    } else {
        m.scheduled_time
            .iter()
            .map(|(h, m)| format!("{:02}:{:02}", h, m))
            .collect::<Vec<_>>()
            .join(", ")
    };
    lines.push(Line::from(Span::styled(
        format!("Scheduled: {}", times),
        content_style(),
    )));
    lines.push(Line::from(Span::styled(
        format!("Taken: {} time(s)", taken_at.len()),
        content_style(),
    )));
    for ts in taken_at {
        lines.push(Line::from(Span::styled(
            format!("  • {}", ts.format("%Y-%m-%d %H:%M")),
            content_style(),
        )));
    }
    Paragraph::new(lines).style(content_style())
}
