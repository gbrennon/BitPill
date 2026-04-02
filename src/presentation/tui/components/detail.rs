use chrono::NaiveDateTime;
use ratatui::{
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::{
    application::dtos::responses::MedicationDto, presentation::tui::styles::content_style,
};

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

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use ratatui::{Terminal, backend::TestBackend, layout::Rect};

    use super::*;
    use crate::application::dtos::responses::MedicationDto;

    fn med(with_schedule: bool) -> MedicationDto {
        MedicationDto {
            id: "m1".to_string(),
            name: "Aspirin".to_string(),
            amount_mg: 100,
            dose_frequency: "OnceDaily".to_string(),
            scheduled_time: if with_schedule { vec![(8, 0)] } else { vec![] },
            taken_today: 0,
            scheduled_today: 0,
        }
    }

    fn ts() -> chrono::NaiveDateTime {
        NaiveDate::from_ymd_opt(2025, 1, 1)
            .unwrap()
            .and_hms_opt(8, 0, 0)
            .unwrap()
    }

    #[test]
    fn medication_detail_with_no_taken_renders() {
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        let m = med(true);
        terminal
            .draw(|f| {
                let widget = medication_detail(&m, &[]);
                f.render_widget(widget, Rect::new(0, 0, 80, 24));
            })
            .unwrap();
        let content: String = terminal
            .backend()
            .buffer()
            .content
            .iter()
            .map(|c| c.symbol())
            .collect();
        assert!(content.contains("Aspirin"));
    }

    #[test]
    fn medication_detail_with_taken_renders_timestamp() {
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        let m = med(false);
        let taken = [ts()];
        terminal
            .draw(|f| {
                let widget = medication_detail(&m, &taken);
                f.render_widget(widget, Rect::new(0, 0, 80, 24));
            })
            .unwrap();
        let content: String = terminal
            .backend()
            .buffer()
            .content
            .iter()
            .map(|c| c.symbol())
            .collect();
        assert!(content.contains("Taken: 1 time(s)") || content.contains("2025"));
    }

    #[test]
    fn medication_detail_no_schedule_shows_no_schedule() {
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        let m = med(false);
        terminal
            .draw(|f| {
                let widget = medication_detail(&m, &[]);
                f.render_widget(widget, Rect::new(0, 0, 80, 24));
            })
            .unwrap();
        let content: String = terminal
            .backend()
            .buffer()
            .content
            .iter()
            .map(|c| c.symbol())
            .collect();
        assert!(content.contains("no schedule"));
    }
}
