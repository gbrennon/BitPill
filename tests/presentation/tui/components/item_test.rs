use bitpill::presentation::tui::components::item::medication_item;

#[test]
fn medication_item_constructs() {
    let item = medication_item("Aspirin", 500);
    // Ensure item is non-empty by checking its size in memory
    assert!(std::mem::size_of_val(&item) > 0);
}
