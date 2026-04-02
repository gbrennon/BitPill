use chrono::Timelike;
use ratatui::{
    Frame,
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::{
    application::dtos::responses::DoseRecordDto,
    presentation::tui::{
        styles::{content_style, highlight_style},
        templates::screen_template::ScreenTemplate,
    },
};

pub struct MarkDoseInput<'a> {
    pub medication_id: &'a str,
    pub records: &'a [DoseRecordDto],
    pub selected_index: usize,
}

pub struct MarkDosePresenter;

impl MarkDosePresenter {
    pub fn present(&self, f: &mut Frame, input: &MarkDoseInput) {
        let help = "[j/k] Navigate  [Enter] Mark as taken  [Esc] Back";
        ScreenTemplate {
            subtitle: "Mark dose as taken",
            help,
            mode: "NORMAL",
        }
        .render(f, |f, area| {
            let lines = build_mark_dose_lines(input.records, input.selected_index);
            let paragraph = Paragraph::new(lines).style(content_style());
            f.render_widget(paragraph, area);
        });
    }
}

// Extracted helper so it can be unit-tested without a terminal backend.
pub fn build_mark_dose_lines(records: &[DoseRecordDto], selected_index: usize) -> Vec<Line<'_>> {
    // Build grouped lines with checkboxes: registered (ids that don't start with "slot:") and scheduled slots (ids that start with "slot:")
    let mut reg_lines: Vec<Line> = Vec::new();
    let mut slot_lines: Vec<Line> = Vec::new();
    for (i, r) in records.iter().enumerate() {
        // extract hour/minute from scheduled_at
        let h = r.scheduled_at.hour();
        let m = r.scheduled_at.minute();
        let selected = i == selected_index;
        let line = crate::presentation::tui::components::mark_taken_line::mark_taken_line(
            selected, h, m, r.taken_at,
        );
        if r.id.starts_with("slot:") {
            slot_lines.push(line);
        } else {
            reg_lines.push(line);
        }
    }
    let mut lines: Vec<Line> = Vec::new();
    if !reg_lines.is_empty() {
        lines.push(Line::from(Span::styled(
            "Registered records:",
            highlight_style(),
        )));
        lines.extend(reg_lines);
        lines.push(Line::from(Span::raw("")));
    }
    if !slot_lines.is_empty() {
        lines.push(Line::from(Span::styled(
            "Scheduled slots:",
            highlight_style(),
        )));
        lines.extend(slot_lines);
    }
    if lines.is_empty() {
        lines.push(Line::from(Span::raw(
            "No dose records or scheduled slots for this medication today",
        )));
    }
    lines
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime};
    use ratatui::text::{Line, Span};

    use super::*;
    use crate::{
        application::dtos::responses::DoseRecordDto, presentation::tui::styles::highlight_style,
    };

    fn make_dt(h: u32, m: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2025, 1, 1)
            .unwrap()
            .and_hms_opt(h, m, 0)
            .unwrap()
    }

    #[test]
    fn headers_are_styled_and_grouped() {
        let rec1 = DoseRecordDto {
            id: "r1".to_string(),
            medication_id: "med".to_string(),
            scheduled_at: make_dt(8, 0),
            taken_at: None,
        };
        let rec2 = DoseRecordDto {
            id: "slot:0".to_string(),
            medication_id: "med".to_string(),
            scheduled_at: make_dt(9, 0),
            taken_at: None,
        };
        let arr = [rec1, rec2];
        let lines = build_mark_dose_lines(&arr, 0);
        let expected = Line::from(Span::styled("Registered records:", highlight_style()));
        assert_eq!(lines[0], expected);
        let expected2 = Line::from(Span::styled("Scheduled slots:", highlight_style()));
        assert!(lines.iter().any(|l| l == &expected2));
    }

    #[test]
    fn no_records_message() {
        let lines = build_mark_dose_lines(&[], 0);
        assert_eq!(lines.len(), 1);
        assert!(lines[0].to_string().contains("No dose records"));
    }
}
