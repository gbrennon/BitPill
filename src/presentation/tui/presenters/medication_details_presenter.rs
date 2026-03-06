// Internal imports first
use crate::application::dtos::responses::{DoseRecordDto, MedicationDto};
use crate::presentation::tui::templates::screen_template::ScreenTemplate;

// External crates
use crate::presentation::tui::styles::{content_style, highlight_style};
use chrono::Datelike;
use ratatui::Frame;
use ratatui::text::{Line, Span};

pub struct MedicationDetailsInput<'a> {
    pub medication: Option<&'a MedicationDto>,
    pub records: Vec<DoseRecordDto>,
}

pub struct MedicationDetailsPresenter;

impl MedicationDetailsPresenter {
    pub fn present(&self, f: &mut Frame, input: &MedicationDetailsInput) {
        ScreenTemplate {
            subtitle: "Medication Details",
            help: " [e] Edit  [s] Mark scheduled slot  [Esc] Back",
            mode: "NORMAL",
        }
        .render(f, |f, area| {
            if let Some(m) = input.medication {
                use chrono::Local;

                // Build styled lines combining labels and values
                let mut lines: Vec<Line> = Vec::new();
                // Basic medication info
                lines.push(Line::from(vec![
                    Span::styled("ID: ", highlight_style()),
                    Span::raw(m.id.to_string()),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("Name: ", highlight_style()),
                    Span::raw(m.name.to_string()),
                ]));
                lines.push(Line::from(vec![
                    Span::styled("Dosage: ", highlight_style()),
                    Span::raw(format!("{} mg", m.amount_mg)),
                ]));
                let freq_readable = match m.dose_frequency.as_str() {
                    "OnceDaily" => "Once Daily",
                    "TwiceDaily" => "Twice Daily",
                    "ThriceDaily" => "Thrice Daily",
                    "Custom" => "Custom",
                    other => other,
                };
                lines.push(Line::from(vec![
                    Span::styled("Frequency: ", highlight_style()),
                    Span::raw(freq_readable),
                ]));
                lines.push(Line::from(Span::raw("")));

                // Scheduled times with taken status for today (match within ±15 minutes)
                lines.push(Line::from(Span::styled(
                    "Scheduled times:",
                    highlight_style(),
                )));
                let today = Local::now().date_naive();
                for (h, mm) in m.scheduled_time.iter() {
                    // scheduled NaiveDateTime for today
                    let scheduled_dt_opt =
                        chrono::NaiveDate::from_ymd_opt(today.year(), today.month(), today.day())
                            .and_then(|d| d.and_hms_opt(*h, *mm, 0));
                    let mut taken_opt: Option<chrono::NaiveDateTime> = None;
                    if let Some(scheduled_dt) = scheduled_dt_opt {
                        for r in input.records.iter() {
                            if let Some(taken) = r.taken_at {
                                // match within ±15 minutes
                                let diff = (taken - scheduled_dt).num_minutes().abs();
                                if taken.date() == today && diff <= 15 {
                                    taken_opt = Some(taken);
                                    break;
                                }
                            }
                        }
                    }
                    lines.push(
                        crate::presentation::tui::components::mark_taken_line::mark_taken_line(
                            false, *h, *mm, taken_opt,
                        ),
                    );
                }
                if m.scheduled_time.is_empty() {
                    lines.push(Line::from(Span::raw("  (none)")));
                }

                lines.push(Line::from(Span::raw("")));
                // Dose records history in a simple list
                lines.push(Line::from(Span::styled("Dose records:", highlight_style())));
                if input.records.is_empty() {
                    lines.push(Line::from(Span::raw("  (no records)")));
                } else {
                    for r in input.records.iter() {
                        let scheduled = r.scheduled_at.format("%Y-%m-%d %H:%M").to_string();
                        let taken = match r.taken_at {
                            Some(t) => t.format("%Y-%m-%d %H:%M").to_string(),
                            None => "(not taken)".to_string(),
                        };
                        lines.push(Line::from(Span::raw(format!(
                            "  - scheduled: {}  taken: {}  id: {}",
                            scheduled, taken, r.id
                        ))));
                    }
                }

                let paragraph = ratatui::widgets::Paragraph::new(lines).style(content_style());
                f.render_widget(paragraph, area);
            } else {
                let paragraph =
                    ratatui::widgets::Paragraph::new("Medication not found").style(content_style());
                f.render_widget(paragraph, area);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::dtos::responses::MedicationDto;
    use chrono::NaiveDate;
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    fn make_terminal() -> Terminal<TestBackend> {
        Terminal::new(TestBackend::new(80, 24)).unwrap()
    }

    fn med(has_schedule: bool) -> MedicationDto {
        MedicationDto {
            id: "m1".to_string(),
            name: "Aspirin".to_string(),
            amount_mg: 100,
            dose_frequency: "OnceDaily".to_string(),
            scheduled_time: if has_schedule { vec![(8, 0)] } else { vec![] },
        }
    }

    fn dose_record(taken: bool) -> DoseRecordDto {
        let base = NaiveDate::from_ymd_opt(2025, 1, 1)
            .unwrap()
            .and_hms_opt(8, 0, 0)
            .unwrap();
        DoseRecordDto {
            id: "r1".to_string(),
            medication_id: "m1".to_string(),
            scheduled_at: base,
            taken_at: if taken { Some(base) } else { None },
        }
    }

    #[test]
    fn present_with_medication_does_not_panic() {
        let mut terminal = make_terminal();
        let m = med(true);
        let input = MedicationDetailsInput {
            medication: Some(&m),
            records: vec![dose_record(false)],
        };
        terminal
            .draw(|f| MedicationDetailsPresenter.present(f, &input))
            .unwrap();
        let buffer = terminal.backend().buffer();
        assert!(buffer.content.iter().any(|c| c.symbol() != " "));
    }

    #[test]
    fn present_with_no_medication_renders_not_found() {
        let mut terminal = make_terminal();
        let input = MedicationDetailsInput {
            medication: None,
            records: vec![],
        };
        terminal
            .draw(|f| MedicationDetailsPresenter.present(f, &input))
            .unwrap();
        let content: String = terminal
            .backend()
            .buffer()
            .content
            .iter()
            .map(|c| c.symbol())
            .collect();
        assert!(content.contains("not found"));
    }

    #[test]
    fn present_with_taken_record_does_not_panic() {
        let mut terminal = make_terminal();
        let m = med(true);
        let input = MedicationDetailsInput {
            medication: Some(&m),
            records: vec![dose_record(true)],
        };
        terminal
            .draw(|f| MedicationDetailsPresenter.present(f, &input))
            .unwrap();
    }

    #[test]
    fn present_medication_no_schedule_and_no_records_does_not_panic() {
        let mut terminal = make_terminal();
        let m = med(false);
        let input = MedicationDetailsInput {
            medication: Some(&m),
            records: vec![],
        };
        terminal
            .draw(|f| MedicationDetailsPresenter.present(f, &input))
            .unwrap();
    }

    #[test]
    fn present_medication_custom_frequency_renders() {
        let mut terminal = make_terminal();
        let m = MedicationDto {
            dose_frequency: "Custom".to_string(),
            ..med(false)
        };
        let input = MedicationDetailsInput {
            medication: Some(&m),
            records: vec![],
        };
        terminal
            .draw(|f| MedicationDetailsPresenter.present(f, &input))
            .unwrap();
    }
}
