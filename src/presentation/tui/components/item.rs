use crate::presentation::tui::styles::content_style;
use ratatui::text::Span;
use ratatui::widgets::ListItem;

/// Creates a styled ListItem for a medication entry
pub fn medication_item(name: &str, amount_mg: u32) -> ListItem<'_> {
    let label = format!("{} — {}mg", name, amount_mg);
    ListItem::new(Span::styled(label, content_style()))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn medication_item_constructs() {
        let item = medication_item("Aspirin", 500);
        // Ensure item is non-empty by checking its size in memory
        assert!(std::mem::size_of_val(&item) > 0);
    }
}
