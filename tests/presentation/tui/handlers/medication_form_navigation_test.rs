use bitpill::presentation::tui::handlers::medication_form_navigation::{
    NavigationState, navigate_down, navigate_left, navigate_right, navigate_up, remove_custom_slot,
    resize_slots_for_frequency, slots_for,
};

fn state(focused_field: u8, slots: Vec<&str>, idx: usize) -> NavigationState {
    NavigationState {
        focused_field,
        scheduled_time: slots.iter().map(|s| s.to_string()).collect(),
        scheduled_idx: idx,
    }
}

// --- slots_for ---

#[test]
fn slots_for_once_daily_returns_one() {
    assert_eq!(slots_for(0, &[]), 1);
}

#[test]
fn slots_for_twice_daily_returns_two() {
    assert_eq!(slots_for(1, &[]), 2);
}

#[test]
fn slots_for_thrice_daily_returns_three() {
    assert_eq!(slots_for(2, &[]), 3);
}

#[test]
fn slots_for_custom_returns_vec_length() {
    let slots = vec!["08:00".to_string(), "12:00".to_string()];
    assert_eq!(slots_for(3, &slots), 2);
}

#[test]
fn slots_for_custom_empty_vec_returns_one() {
    assert_eq!(slots_for(3, &[]), 1);
}

// --- resize_slots_for_frequency ---

#[test]
fn resize_slots_grows_vec_for_twice_daily() {
    let mut slots = vec!["08:00".to_string()];
    let mut idx = 0;
    resize_slots_for_frequency(1, &mut slots, &mut idx);
    assert_eq!(slots.len(), 2);
}

#[test]
fn resize_slots_truncates_vec_for_once_daily() {
    let mut slots = vec![
        "08:00".to_string(),
        "12:00".to_string(),
        "20:00".to_string(),
    ];
    let mut idx = 2;
    resize_slots_for_frequency(0, &mut slots, &mut idx);
    assert_eq!(slots.len(), 1);
    assert_eq!(idx, 0);
}

#[test]
fn resize_slots_custom_preserves_existing_slots() {
    let mut slots = vec!["08:00".to_string(), "12:00".to_string()];
    let mut idx = 1;
    resize_slots_for_frequency(3, &mut slots, &mut idx);
    assert_eq!(slots.len(), 2);
    assert_eq!(idx, 1);
}

#[test]
fn resize_slots_custom_ensures_at_least_one_slot() {
    let mut slots: Vec<String> = vec![];
    let mut idx = 0;
    resize_slots_for_frequency(3, &mut slots, &mut idx);
    assert_eq!(slots.len(), 1);
}

// --- navigate_down ---

#[test]
fn navigate_down_on_field_zero_moves_to_field_one() {
    let result = navigate_down(state(0, vec!["08:00"], 0), 0);
    assert_eq!(result.focused_field, 1);
}

#[test]
fn navigate_down_on_field_three_advances_slot_index() {
    let result = navigate_down(state(3, vec!["08:00", "12:00"], 0), 1);
    assert_eq!(result.scheduled_idx, 1);
    assert_eq!(result.focused_field, 3);
}

#[test]
fn navigate_down_on_field_three_custom_at_last_slot_appends_new_slot() {
    let result = navigate_down(state(3, vec!["08:00"], 0), 3);
    assert_eq!(result.scheduled_time.len(), 2);
    assert_eq!(result.scheduled_idx, 1);
}

#[test]
fn navigate_down_on_field_three_fixed_at_last_slot_stays() {
    let result = navigate_down(state(3, vec!["08:00"], 0), 0);
    assert_eq!(result.scheduled_idx, 0);
    assert_eq!(result.scheduled_time.len(), 1);
}

#[test]
fn navigate_down_on_field_three_caps_at_field_three() {
    let result = navigate_down(state(3, vec!["08:00"], 0), 2);
    assert_eq!(result.focused_field, 3);
}

// --- navigate_up ---

#[test]
fn navigate_up_on_field_one_moves_to_field_zero() {
    let result = navigate_up(state(1, vec!["08:00"], 0));
    assert_eq!(result.focused_field, 0);
}

#[test]
fn navigate_up_on_field_zero_stays() {
    let result = navigate_up(state(0, vec!["08:00"], 0));
    assert_eq!(result.focused_field, 0);
}

#[test]
fn navigate_up_on_field_three_with_slot_gt_zero_decrements_slot() {
    let result = navigate_up(state(3, vec!["08:00", "12:00"], 1));
    assert_eq!(result.scheduled_idx, 0);
    assert_eq!(result.focused_field, 3);
}

#[test]
fn navigate_up_on_field_three_at_slot_zero_moves_to_field_two() {
    let result = navigate_up(state(3, vec!["08:00"], 0));
    assert_eq!(result.focused_field, 2);
}

// --- navigate_right ---

#[test]
fn navigate_right_on_field_two_advances_frequency() {
    let (freq, nav) = navigate_right(state(2, vec!["08:00"], 0), 0);
    assert_eq!(freq, 1);
    assert_eq!(nav.scheduled_time.len(), 2);
}

#[test]
fn navigate_right_on_field_two_caps_at_custom() {
    let (freq, _) = navigate_right(state(2, vec!["08:00", "12:00", "20:00"], 0), 3);
    assert_eq!(freq, 3);
}

#[test]
fn navigate_right_on_field_three_advances_slot() {
    let (_, nav) = navigate_right(state(3, vec!["08:00", "12:00"], 0), 1);
    assert_eq!(nav.scheduled_idx, 1);
}

// --- navigate_left ---

#[test]
fn navigate_left_on_field_two_retreats_frequency() {
    let (freq, nav) = navigate_left(state(2, vec!["08:00", "12:00"], 0), 1);
    assert_eq!(freq, 0);
    assert_eq!(nav.scheduled_time.len(), 1);
}

#[test]
fn navigate_left_on_field_two_stays_at_zero() {
    let (freq, _) = navigate_left(state(2, vec!["08:00"], 0), 0);
    assert_eq!(freq, 0);
}

#[test]
fn navigate_left_on_field_three_retreats_slot() {
    let (_, nav) = navigate_left(state(3, vec!["08:00", "12:00"], 1), 1);
    assert_eq!(nav.scheduled_idx, 0);
}

// --- remove_custom_slot ---

#[test]
fn remove_custom_slot_removes_selected_slot() {
    let slots = vec![
        "08:00".to_string(),
        "12:00".to_string(),
        "20:00".to_string(),
    ];
    let (result, idx) = remove_custom_slot(slots, 1);
    assert_eq!(result, vec!["08:00", "20:00"]);
    assert_eq!(idx, 1);
}

#[test]
fn remove_custom_slot_clamps_index_when_last_slot_removed() {
    let slots = vec!["08:00".to_string(), "12:00".to_string()];
    let (result, idx) = remove_custom_slot(slots, 1);
    assert_eq!(result, vec!["08:00"]);
    assert_eq!(idx, 0);
}

#[test]
fn remove_custom_slot_does_not_remove_when_only_one_slot() {
    let slots = vec!["08:00".to_string()];
    let (result, idx) = remove_custom_slot(slots, 0);
    assert_eq!(result.len(), 1);
    assert_eq!(idx, 0);
}
