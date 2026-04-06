use bitpill::presentation::tui::handlers::time_slot_parser::{
    frequency_str, parse_slots, validate_slot_count,
};

fn strs(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

#[test]
fn parse_slots_valid_hh_mm_normalises_and_returns_times() {
    let result = parse_slots(&strs(&["8:5"])).unwrap();

    assert_eq!(result.normalized, vec!["08:05"]);
    assert_eq!(result.times, vec![(8, 5)]);
}

#[test]
fn parse_slots_skips_empty_strings() {
    let result = parse_slots(&strs(&["", "20:00"])).unwrap();

    assert_eq!(result.times, vec![(20, 0)]);
}

#[test]
fn parse_slots_returns_invalid_format_for_non_numeric_input() {
    let err = parse_slots(&strs(&["ab:cd"])).unwrap_err();
    let display = err.to_string();

    assert!(display.contains("'ab:cd'"));
    assert!(display.contains("Invalid time"));
    assert!(display.contains("HH:MM"));
}

#[test]
fn parse_slots_returns_invalid_format_when_minute_missing() {
    let err = parse_slots(&strs(&["08"])).unwrap_err();
    let display = err.to_string();

    assert!(display.contains("'08'"));
    assert!(display.contains("Invalid time"));
    assert!(display.contains("HH:MM"));
}

#[test]
fn parse_slots_returns_out_of_range_for_hour_above_23() {
    let err = parse_slots(&strs(&["24:00"])).unwrap_err();
    let display = err.to_string();

    assert!(display.contains("'24:00'"));
    assert!(display.contains("Invalid time"));
    assert!(display.contains("hour"));
}

#[test]
fn parse_slots_returns_out_of_range_for_minute_above_59() {
    let err = parse_slots(&strs(&["08:60"])).unwrap_err();
    let display = err.to_string();

    assert!(display.contains("'08:60'"));
    assert!(display.contains("Invalid time"));
    assert!(display.contains("minutes"));
}

#[test]
fn parse_slots_error_reports_correct_slot_index() {
    let err = parse_slots(&strs(&["08:00", "bad"])).unwrap_err();
    let display = err.to_string();

    assert!(display.contains("'bad'"));
    assert!(display.contains("Invalid time"));
}

#[test]
fn validate_slot_count_once_daily_accepts_one() {
    assert!(validate_slot_count(0, 1).is_ok());
}

#[test]
fn validate_slot_count_once_daily_rejects_two() {
    assert!(validate_slot_count(0, 2).is_err());
}

#[test]
fn validate_slot_count_twice_daily_accepts_two() {
    assert!(validate_slot_count(1, 2).is_ok());
}

#[test]
fn validate_slot_count_thrice_daily_accepts_three() {
    assert!(validate_slot_count(2, 3).is_ok());
}

#[test]
fn validate_slot_count_custom_accepts_four_or_more() {
    assert!(validate_slot_count(3, 4).is_ok());
    assert!(validate_slot_count(3, 5).is_ok());
}

#[test]
fn validate_slot_count_custom_rejects_fewer_than_four() {
    assert!(validate_slot_count(3, 0).is_err());
    assert!(validate_slot_count(3, 1).is_err());
    assert!(validate_slot_count(3, 2).is_err());
    assert!(validate_slot_count(3, 3).is_err());
}

#[test]
fn validate_slot_count_error_message_includes_expected_count_for_fixed_frequency() {
    let err = validate_slot_count(1, 1).unwrap_err();

    assert!(err.contains('2'));
}

#[test]
fn frequency_str_maps_indices_to_expected_strings() {
    assert_eq!(frequency_str(0), "OnceDaily");
    assert_eq!(frequency_str(1), "TwiceDaily");
    assert_eq!(frequency_str(2), "ThriceDaily");
    assert_eq!(frequency_str(3), "Custom");
}
