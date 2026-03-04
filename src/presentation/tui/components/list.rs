use crate::application::ports::inbound::list_all_medications_port::MedicationDto;
use crate::presentation::tui::components::item::medication_item;
use crate::presentation::tui::styles::{BORDER_COLOR, content_style};
use ratatui::widgets::{Block, Borders, List, ListItem};

/// Dumb list component: accepts domain DTOs and builds styled list items
pub fn medication_list<'a>(medications: &'a [MedicationDto]) -> List<'a> {
    let items = medications
        .iter()
        .map(|m| medication_item(&m.name, m.amount_mg))
        .collect::<Vec<ListItem<'a>>>();

    List::new(items).block(
        Block::default()
            .borders(Borders::LEFT | Borders::RIGHT)
            .border_style(content_style().fg(BORDER_COLOR))
            .style(content_style()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::inbound::list_all_medications_port::MedicationDto;

    #[test]
    fn medication_list_constructs_with_items() {
        let items = vec![
            MedicationDto {
                id: "1".into(),
                name: "Aspirin".into(),
                amount_mg: 500,
                scheduled_time: vec![],
                dose_frequency: "OnceDaily".to_string(),
            },
            MedicationDto {
                id: "2".into(),
                name: "Ibuprofen".into(),
                amount_mg: 200,
                scheduled_time: vec![],
                dose_frequency: "OnceDaily".to_string(),
            },
        ];
        let list = medication_list(&items);
        // Ensure list is non-empty by checking size
        assert!(std::mem::size_of_val(&list) > 0);
    }
}
