use bitpill::presentation::tui::components::bottom_bar::bottom_bar;

#[test]
fn bottom_bar_constructs() {
    let p = bottom_bar(" [c] Create  [s] Settings  [q] Quit");
    assert!(std::mem::size_of_val(&p) > 0);
}
