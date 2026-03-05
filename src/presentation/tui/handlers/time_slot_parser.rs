//! Time-slot parsing and validation for the medication form.
//!
//! All functions are pure: no `App` or I/O access.
//! A successfully parsed set of time slots.

#[cfg(test)]
mod tests {
    use super::*;

    fn strs(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    // --- parse_slots ---

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

        assert_eq!(err, SlotParseError::InvalidFormat { slot_index: 0 });
    }

    #[test]
    fn parse_slots_returns_invalid_format_when_minute_missing() {
        let err = parse_slots(&strs(&["08"])).unwrap_err();

        assert_eq!(err, SlotParseError::InvalidFormat { slot_index: 0 });
    }

    #[test]
    fn parse_slots_returns_out_of_range_for_hour_above_23() {
        let err = parse_slots(&strs(&["24:00"])).unwrap_err();

        assert_eq!(err, SlotParseError::OutOfRange { slot_index: 0 });
    }

    #[test]
    fn parse_slots_returns_out_of_range_for_minute_above_59() {
        let err = parse_slots(&strs(&["08:60"])).unwrap_err();

        assert_eq!(err, SlotParseError::OutOfRange { slot_index: 0 });
    }

    #[test]
    fn parse_slots_error_reports_correct_slot_index() {
        let err = parse_slots(&strs(&["08:00", "bad"])).unwrap_err();

        assert_eq!(err, SlotParseError::InvalidFormat { slot_index: 1 });
    }

    // --- validate_slot_count ---

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
    fn validate_slot_count_custom_accepts_any_nonzero_count() {
        assert!(validate_slot_count(3, 1).is_ok());
        assert!(validate_slot_count(3, 5).is_ok());
    }

    #[test]
    fn validate_slot_count_custom_rejects_zero() {
        assert!(validate_slot_count(3, 0).is_err());
    }

    #[test]
    fn validate_slot_count_error_message_includes_expected_count_for_fixed_frequency() {
        let err = validate_slot_count(1, 1).unwrap_err();

        assert!(err.contains('2'));
    }

    // --- frequency_str ---

    #[test]
    fn frequency_str_maps_indices_to_expected_strings() {
        assert_eq!(frequency_str(0), "OnceDaily");
        assert_eq!(frequency_str(1), "TwiceDaily");
        assert_eq!(frequency_str(2), "ThriceDaily");
        assert_eq!(frequency_str(3), "Custom");
    }
}

#[derive(Debug, PartialEq)]
pub struct ParsedSlots {
    /// Slot strings normalised to `HH:MM` format.
    pub normalized: Vec<String>,
    /// Parsed `(hour, minute)` pairs (empty slots are skipped).
    pub times: Vec<(u32, u32)>,
}

/// Error produced while parsing a single time slot.
#[derive(Debug, PartialEq)]
pub enum SlotParseError {
    InvalidFormat { slot_index: usize },
    OutOfRange { slot_index: usize },
}

impl std::fmt::Display for SlotParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFormat { slot_index } => {
                write!(
                    f,
                    "Invalid time in slot {}: expected HH:MM or HH:MM:SS",
                    slot_index + 1
                )
            }
            Self::OutOfRange { slot_index } => {
                write!(f, "Time out of range in slot {}", slot_index + 1)
            }
        }
    }
}

/// Parses and normalises each non-empty entry of `raw`.
///
/// Empty strings are silently skipped. Returns [`ParsedSlots`] on success or
/// the first [`SlotParseError`] encountered.
pub fn parse_slots(raw: &[String]) -> Result<ParsedSlots, SlotParseError> {
    let mut normalized = raw.to_vec();
    let mut times = Vec::new();

    for (i, slot) in normalized.iter_mut().enumerate() {
        let part = slot.trim();
        if part.is_empty() {
            continue;
        }
        let mut iter = part.split(':');
        let h = iter
            .next()
            .and_then(|s| s.parse::<u32>().ok())
            .ok_or(SlotParseError::InvalidFormat { slot_index: i })?;
        let m = iter
            .next()
            .and_then(|s| s.parse::<u32>().ok())
            .ok_or(SlotParseError::InvalidFormat { slot_index: i })?;
        if h > 23 || m > 59 {
            return Err(SlotParseError::OutOfRange { slot_index: i });
        }
        *slot = format!("{:02}:{:02}", h, m);
        times.push((h, m));
    }

    Ok(ParsedSlots { normalized, times })
}

/// Returns `Ok(())` when the number of parsed times is valid for `selected_frequency`.
///
/// Fixed frequencies (0–2) require an exact count. Custom (3) requires at least 1.
pub fn validate_slot_count(selected_frequency: usize, count: usize) -> Result<(), String> {
    let valid = match selected_frequency {
        0 => count == 1,
        1 => count == 2,
        2 => count == 3,
        _ => count >= 1,
    };
    if valid {
        Ok(())
    } else {
        let msg = match selected_frequency {
            3 => "Please provide at least 1 scheduled time".into(),
            n => format!("Please provide {} scheduled time(s)", n + 1),
        };
        Err(msg)
    }
}

/// Maps a `selected_frequency` index to the string expected by the application service.
pub fn frequency_str(selected_frequency: usize) -> &'static str {
    match selected_frequency {
        0 => "OnceDaily",
        1 => "TwiceDaily",
        2 => "ThriceDaily",
        _ => "Custom",
    }
}
