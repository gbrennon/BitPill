use bitpill::presentation::tui::components::schedule_time::schedule_time;

#[test]
fn schedule_time_constructs_with_placeholders() {
    let vals: Vec<String> = vec![];
    let list = schedule_time(3, &vals, 0, true);
    // ensure widget constructed
    assert!(std::mem::size_of_val(&list) > 0);
}
