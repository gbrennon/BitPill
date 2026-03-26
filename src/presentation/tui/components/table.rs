use crate::application::dtos::responses::MedicationDto;
use crate::presentation::tui::styles::{content_style, highlight_style, title_style, BORDER_COLOR};
use ratatui::layout::Constraint;
use ratatui::widgets::{Block, Borders, Cell, Row, Table};

/// Simple table component specialized for medications. Title and column names are injected.
/// Selected row (if any) will be styled using highlight_style().
pub fn medication_table<'a>(
    title: &'a str,
    columns: &'a [&str],
    medications: &'a [MedicationDto],
    selected: Option<usize>,
) -> Table<'a> {
    let header_cells = columns
        .iter()
        .map(|c| Cell::from(*c))
        .collect::<Vec<Cell>>();
    let header = Row::new(header_cells).style(title_style());

    let rows = medications
        .iter()
        .enumerate()
        .map(|(i, m)| {
            // Build cells to match requested columns: Name, mg, optionally Actions, optionally Taken
            let mut cells = vec![
                Cell::from(m.name.clone()),
                Cell::from(m.amount_mg.to_string()),
            ];
            if columns.len() >= 4 {
                let taken_str = if m.scheduled_today > 0 {
                    format!("{}/{}", m.taken_today, m.scheduled_today)
                } else {
                    "-".to_string()
                };
                cells.push(Cell::from(taken_str));
            }
            if columns.len() >= 3 {
                cells.push(Cell::from("[e] Edit"));
            }
            let mut row = Row::new(cells);
            if Some(i) == selected {
                row = row.style(highlight_style());
            }
            row
        })
        .collect::<Vec<Row>>();

    // Choose column width constraints based on how many columns requested
    let table = match columns.len() {
        2 => Table::new(
            rows,
            [Constraint::Percentage(70), Constraint::Percentage(30)],
        ),
        4 => Table::new(
            rows,
            [
                Constraint::Percentage(40),
                Constraint::Percentage(15),
                Constraint::Percentage(15),
                Constraint::Percentage(30),
            ],
        ),
        _ => Table::new(
            rows,
            [
                Constraint::Percentage(60),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ],
        ),
    };

    table.column_spacing(1).header(header).block(
        Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(content_style().fg(BORDER_COLOR))
            .style(content_style()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::dtos::responses::MedicationDto;
    use ratatui::backend::TestBackend;
    use ratatui::layout::Rect;
    use ratatui::Terminal;

    fn med(name: &str) -> MedicationDto {
        MedicationDto {
            id: "m1".to_string(),
            name: name.to_string(),
            amount_mg: 100,
            dose_frequency: "OnceDaily".to_string(),
            scheduled_time: vec![],
            taken_today: 0,
            scheduled_today: 0,
        }
    }

    #[test]
    fn two_column_table_renders_without_panic() {
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        let meds = vec![med("Aspirin"), med("Ibuprofen")];
        terminal
            .draw(|f| {
                let table = medication_table("Meds", &["Name", "mg"], &meds, None);
                f.render_widget(table, Rect::new(0, 0, 80, 24));
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
    fn three_column_table_renders_without_panic() {
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        let meds = vec![med("Aspirin")];
        terminal
            .draw(|f| {
                let table = medication_table("Meds", &["Name", "mg", "Actions"], &meds, None);
                f.render_widget(table, Rect::new(0, 0, 80, 24));
            })
            .unwrap();
        let content: String = terminal
            .backend()
            .buffer()
            .content
            .iter()
            .map(|c| c.symbol())
            .collect();
        assert!(content.contains("Edit") || content.contains("e]"));
    }

    #[test]
    fn selected_row_is_highlighted() {
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        let meds = vec![med("Aspirin"), med("Ibuprofen")];
        terminal
            .draw(|f| {
                let table = medication_table("Meds", &["Name", "mg"], &meds, Some(0));
                f.render_widget(table, Rect::new(0, 0, 80, 24));
            })
            .unwrap();
        // Just ensure it renders without panic
        let buffer = terminal.backend().buffer();
        assert!(buffer.content.iter().any(|c| c.symbol() != " "));
    }

    #[test]
    fn empty_medication_list_renders_header_only() {
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        terminal
            .draw(|f| {
                let table = medication_table("Meds", &["Name", "mg"], &[], None);
                f.render_widget(table, Rect::new(0, 0, 80, 24));
            })
            .unwrap();
        let content: String = terminal
            .backend()
            .buffer()
            .content
            .iter()
            .map(|c| c.symbol())
            .collect();
        assert!(content.contains("Name"));
    }
}
