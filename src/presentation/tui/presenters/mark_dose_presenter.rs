use crate::presentation::tui::templates::screen_template::ScreenTemplate;
use ratatui::Frame;
use ratatui::widgets::Paragraph;
use crate::presentation::tui::styles::content_style;
use crate::application::ports::inbound::list_dose_records_port::DoseRecordDto;

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
            // Build lines
            let mut lines: Vec<String> = Vec::new();
            for (i, r) in input.records.iter().enumerate() {
                let marker = if i == input.selected_index { ">" } else { " " };
                lines.push(format!("{} {} - {}", marker, r.id, r.scheduled_at.format("%Y-%m-%d %H:%M")));
            }
            if lines.is_empty() {
                lines.push("No dose records for this medication".to_string());
            }
            let paragraph = Paragraph::new(lines.join("\n")).style(content_style());
            f.render_widget(paragraph, area);
        });
    }
}
