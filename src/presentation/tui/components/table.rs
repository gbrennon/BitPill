use crate::application::ports::inbound::list_all_medications_port::MedicationDto;
use crate::presentation::tui::styles::{BORDER_COLOR, content_style, highlight_style, title_style};
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
            // Build cells to match requested columns: Name, mg, optionally Actions
            let mut cells = vec![
                Cell::from(m.name.clone()),
                Cell::from(m.amount_mg.to_string()),
            ];
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
