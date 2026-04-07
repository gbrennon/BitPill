/// Result of parsing a list of scheduled time strings.
///
/// # Fields
///
/// - `times` — Parsed (hour, minute) tuples.
/// - `normalized` — Zero-padded time strings in "HH:MM" format.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedScheduledTimes {
    pub times: Vec<(u32, u32)>,
    pub normalized: Vec<String>,
}

/// Error that occurs when parsing a scheduled time string.
///
/// # Fields
///
/// - `slot_index` — Position in the input array where the error occurred.
/// - `found` — The problematic string that failed to parse.
/// - `kind` — The specific kind of parse error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduledTimeParseError {
    pub slot_index: usize,
    pub found: String,
    pub kind: ScheduledTimeParseErrorKind,
}

/// Specific kind of scheduled time parse error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScheduledTimeParseErrorKind {
    InvalidFormat,
    OutOfRange,
}

impl std::fmt::Display for ScheduledTimeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ScheduledTimeParseErrorKind::InvalidFormat => {
                write!(
                    f,
                    "Invalid time '{}': expected format HH:MM (e.g., 08:00 or 20:30)",
                    self.found
                )
            }
            ScheduledTimeParseErrorKind::OutOfRange => {
                write!(
                    f,
                    "Invalid time '{}': hour must be 0-23, minutes must be 0-59",
                    self.found
                )
            }
        }
    }
}

impl std::error::Error for ScheduledTimeParseError {}

/// Parses a list of time strings into scheduled times.
///
/// Takes raw string inputs (e.g., ["8:00", "20:30"]) and parses them into
/// (hour, minute) tuples with validation.
///
/// # Errors
///
/// Returns `ScheduledTimeParseError` if any string is malformed or contains
/// out-of-range values.
pub fn parse_scheduled_times(
    raw: &[String],
) -> Result<ParsedScheduledTimes, ScheduledTimeParseError> {
    let mut times = Vec::new();
    let mut normalized = Vec::new();

    for (i, slot) in raw.iter().enumerate() {
        let part = slot.trim();
        if part.is_empty() {
            continue;
        }

        let mut iter = part.split(':');
        let hour: u32 = match iter.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => {
                return Err(ScheduledTimeParseError {
                    slot_index: i,
                    found: part.to_string(),
                    kind: ScheduledTimeParseErrorKind::InvalidFormat,
                });
            }
        };
        let minute: u32 = match iter.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => {
                return Err(ScheduledTimeParseError {
                    slot_index: i,
                    found: part.to_string(),
                    kind: ScheduledTimeParseErrorKind::InvalidFormat,
                });
            }
        };

        if hour > 23 || minute > 59 {
            return Err(ScheduledTimeParseError {
                slot_index: i,
                found: part.to_string(),
                kind: ScheduledTimeParseErrorKind::OutOfRange,
            });
        }

        let normalized_str = format!("{:02}:{:02}", hour, minute);
        normalized.push(normalized_str);
        times.push((hour, minute));
    }

    Ok(ParsedScheduledTimes { times, normalized })
}

impl From<ScheduledTimeParseError> for crate::domain::errors::DomainError {
    fn from(err: ScheduledTimeParseError) -> Self {
        crate::domain::errors::DomainError::InvalidScheduledTimeCustom(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strs(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn parse_scheduled_times_valid_hh_mm_returns_times() {
        let result = parse_scheduled_times(&strs(&["8:5"])).unwrap();

        assert_eq!(result.normalized, vec!["08:05"]);
        assert_eq!(result.times, vec![(8, 5)]);
    }

    #[test]
    fn parse_scheduled_times_skips_empty_strings() {
        let result = parse_scheduled_times(&strs(&["", "20:00"])).unwrap();

        assert_eq!(result.times, vec![(20, 0)]);
    }

    #[test]
    fn parse_scheduled_times_invalid_format_returns_error() {
        let err = parse_scheduled_times(&strs(&["ab:cd"])).unwrap_err();

        assert_eq!(err.slot_index, 0);
        assert_eq!(err.found, "ab:cd");
        assert!(matches!(
            err.kind,
            ScheduledTimeParseErrorKind::InvalidFormat
        ));
    }

    #[test]
    fn parse_scheduled_times_missing_minute_returns_error() {
        let err = parse_scheduled_times(&strs(&["08"])).unwrap_err();

        assert_eq!(err.slot_index, 0);
        assert_eq!(err.found, "08");
        assert!(matches!(
            err.kind,
            ScheduledTimeParseErrorKind::InvalidFormat
        ));
    }

    #[test]
    fn parse_scheduled_times_hour_out_of_range_returns_error() {
        let err = parse_scheduled_times(&strs(&["24:00"])).unwrap_err();

        assert_eq!(err.slot_index, 0);
        assert_eq!(err.found, "24:00");
        assert!(matches!(err.kind, ScheduledTimeParseErrorKind::OutOfRange));
    }

    #[test]
    fn parse_scheduled_times_minute_out_of_range_returns_error() {
        let err = parse_scheduled_times(&strs(&["08:60"])).unwrap_err();

        assert_eq!(err.slot_index, 0);
        assert_eq!(err.found, "08:60");
        assert!(matches!(err.kind, ScheduledTimeParseErrorKind::OutOfRange));
    }

    #[test]
    fn parse_scheduled_times_second_slot_error_reports_correct_index() {
        let err = parse_scheduled_times(&strs(&["08:00", "bad"])).unwrap_err();

        assert_eq!(err.slot_index, 1);
        assert_eq!(err.found, "bad");
    }

    #[test]
    fn parse_scheduled_times_error_display_is_user_friendly() {
        let err = parse_scheduled_times(&strs(&["abc:12"])).unwrap_err();

        let display = err.to_string();
        assert!(display.contains("'abc:12'"));
        assert!(display.contains("HH:MM"));
        assert!(display.contains("Invalid time"));
    }
}
