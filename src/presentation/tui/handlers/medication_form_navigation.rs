//! Shared navigation logic for the Create and Edit medication form screens.
//!
//! All functions are pure: they take form state by value or reference and return
//! updated state, with no access to `App` or any I/O.
//! Carries the mutable navigation fields that both form handlers share.

#![allow(clippy::non_ascii_literal)]

//! Shared navigation logic for the Create and Edit medication form screens.
//!
//! All functions are pure: they take form state by value or reference and return
//! updated state, with no access to `App` or any I/O.
//! Carries the mutable navigation fields that both form handlers share.

#[cfg(test)]
mod tests {
    use super::*;

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
}

pub struct NavigationState {
    pub focused_field: u8,
    pub scheduled_time: Vec<String>,
    pub scheduled_idx: usize,
}

/// Returns the number of active time slots for `selected_frequency`.
///
/// Indices 0–2 map to fixed counts (1, 2, 3). Index 3 (Custom) is dynamic:
/// the slot count equals the current `scheduled_time` length, minimum 1.
pub fn slots_for(selected_frequency: usize, scheduled_time: &[String]) -> usize {
    match selected_frequency {
        0 => 1,
        1 => 2,
        2 => 3,
        _ => scheduled_time.len().max(1),
    }
}

/// Resizes `scheduled_time` and clamps `scheduled_idx` to fit `selected_frequency`.
///
/// For Custom (sel == 3) the existing slots are preserved; only the index is
/// clamped. For all other values the vec is grown/truncated to the fixed count.
pub fn resize_slots_for_frequency(
    sel: usize,
    scheduled_time: &mut Vec<String>,
    scheduled_idx: &mut usize,
) {
    if sel == 3 {
        if scheduled_time.is_empty() {
            scheduled_time.push(String::new());
        }
        if *scheduled_idx >= scheduled_time.len() {
            *scheduled_idx = scheduled_time.len() - 1;
        }
    } else {
        let slots = match sel {
            0 => 1,
            1 => 2,
            2 => 3,
            _ => 3,
        };
        while scheduled_time.len() < slots {
            scheduled_time.push(String::new());
        }
        scheduled_time.truncate(slots);
        if *scheduled_idx >= slots {
            *scheduled_idx = slots - 1;
        }
    }
}

/// Moves focus down: advances the slot index within field 3, or moves to the
/// next field (capped at 3).
///
/// For Custom frequency (`selected_frequency == 3`), pressing down past the
/// last slot appends a new empty slot.
pub fn navigate_down(state: NavigationState, selected_frequency: usize) -> NavigationState {
    let NavigationState {
        focused_field,
        mut scheduled_time,
        mut scheduled_idx,
    } = state;
    if focused_field == 3 {
        let slots = slots_for(selected_frequency, &scheduled_time);
        if selected_frequency == 3 && scheduled_idx + 1 >= slots {
            scheduled_time.push(String::new());
            scheduled_idx = scheduled_time.len() - 1;
        } else if scheduled_idx + 1 < slots {
            scheduled_idx += 1;
        }
        NavigationState {
            focused_field,
            scheduled_time,
            scheduled_idx,
        }
    } else {
        let next = (focused_field + 1).min(3);
        NavigationState {
            focused_field: next,
            scheduled_time,
            scheduled_idx,
        }
    }
}

/// Moves focus up: decrements the slot index within field 3, or moves to the
/// previous field (capped at 0).
pub fn navigate_up(state: NavigationState) -> NavigationState {
    let NavigationState {
        focused_field,
        scheduled_time,
        mut scheduled_idx,
    } = state;
    if focused_field == 3 {
        if scheduled_idx > 0 {
            scheduled_idx -= 1;
            NavigationState {
                focused_field,
                scheduled_time,
                scheduled_idx,
            }
        } else {
            NavigationState {
                focused_field: focused_field.saturating_sub(1),
                scheduled_time,
                scheduled_idx,
            }
        }
    } else {
        NavigationState {
            focused_field: focused_field.saturating_sub(1),
            scheduled_time,
            scheduled_idx,
        }
    }
}

/// Advances the frequency selector right (field 2) or moves to the next slot
/// within field 3. `selected_frequency` must be the current value; the updated
/// value is returned alongside the mutated navigation state.
///
/// Returns `(new_selected_frequency, NavigationState)`. If not on field 2 or 3
/// the frequency and state are returned unchanged.
pub fn navigate_right(
    state: NavigationState,
    selected_frequency: usize,
) -> (usize, NavigationState) {
    let NavigationState {
        focused_field,
        mut scheduled_time,
        mut scheduled_idx,
    } = state;
    if focused_field == 2 {
        let sel = (selected_frequency + 1).min(3);
        resize_slots_for_frequency(sel, &mut scheduled_time, &mut scheduled_idx);
        (
            sel,
            NavigationState {
                focused_field,
                scheduled_time,
                scheduled_idx,
            },
        )
    } else if focused_field == 3 {
        let slots = slots_for(selected_frequency, &scheduled_time);
        if scheduled_idx + 1 < slots {
            scheduled_idx += 1;
        }
        (
            selected_frequency,
            NavigationState {
                focused_field,
                scheduled_time,
                scheduled_idx,
            },
        )
    } else {
        (
            selected_frequency,
            NavigationState {
                focused_field,
                scheduled_time,
                scheduled_idx,
            },
        )
    }
}

/// Retreats the frequency selector left (field 2) or moves to the previous slot
/// within field 3.
///
/// Returns `(new_selected_frequency, NavigationState)`.
pub fn navigate_left(
    state: NavigationState,
    selected_frequency: usize,
) -> (usize, NavigationState) {
    let NavigationState {
        focused_field,
        mut scheduled_time,
        mut scheduled_idx,
    } = state;
    if focused_field == 2 {
        let sel = selected_frequency.saturating_sub(1);
        resize_slots_for_frequency(sel, &mut scheduled_time, &mut scheduled_idx);
        (
            sel,
            NavigationState {
                focused_field,
                scheduled_time,
                scheduled_idx,
            },
        )
    } else if focused_field == 3 && scheduled_idx > 0 {
        scheduled_idx -= 1;
        (
            selected_frequency,
            NavigationState {
                focused_field,
                scheduled_time,
                scheduled_idx,
            },
        )
    } else {
        (
            selected_frequency,
            NavigationState {
                focused_field,
                scheduled_time,
                scheduled_idx,
            },
        )
    }
}

/// Removes the slot at `scheduled_idx` when there is more than one slot.
/// Clamps the index after removal. Panics if `scheduled_time` is empty.
pub fn remove_custom_slot(
    mut scheduled_time: Vec<String>,
    mut scheduled_idx: usize,
) -> (Vec<String>, usize) {
    if scheduled_time.len() > 1 {
        scheduled_time.remove(scheduled_idx);
        if scheduled_idx >= scheduled_time.len() {
            scheduled_idx = scheduled_time.len() - 1;
        }
    }
    (scheduled_time, scheduled_idx)
}
