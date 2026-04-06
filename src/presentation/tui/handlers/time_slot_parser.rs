//! Time-slot parsing and validation for the medication form.
//!
//! This module re-exports domain-level parsing functionality and adds
//! presentation-specific helpers (frequency mapping, slot count validation).

use crate::domain::value_objects::ParsedScheduledTimes;

#[derive(Debug)]
pub struct ParsedSlots {
    pub normalized: Vec<String>,
    pub times: Vec<(u32, u32)>,
}

impl From<ParsedScheduledTimes> for ParsedSlots {
    fn from(p: ParsedScheduledTimes) -> Self {
        Self {
            normalized: p.normalized,
            times: p.times,
        }
    }
}

#[derive(Debug)]
pub enum SlotParseError {
    InvalidFormat { slot_index: usize, found: String },
    OutOfRange { slot_index: usize, found: String },
}

impl std::fmt::Display for SlotParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFormat { found, .. } => {
                write!(
                    f,
                    "Invalid time '{}': expected format HH:MM (e.g., 08:00 or 20:30)",
                    found
                )
            }
            Self::OutOfRange { found, .. } => {
                write!(
                    f,
                    "Invalid time '{}': hour must be 0-23, minutes must be 0-59",
                    found
                )
            }
        }
    }
}

impl From<crate::domain::value_objects::ScheduledTimeParseError> for SlotParseError {
    fn from(err: crate::domain::value_objects::ScheduledTimeParseError) -> Self {
        match err.kind {
            crate::domain::value_objects::ScheduledTimeParseErrorKind::InvalidFormat => {
                SlotParseError::InvalidFormat {
                    slot_index: err.slot_index,
                    found: err.found,
                }
            }
            crate::domain::value_objects::ScheduledTimeParseErrorKind::OutOfRange => {
                SlotParseError::OutOfRange {
                    slot_index: err.slot_index,
                    found: err.found,
                }
            }
        }
    }
}

pub fn parse_slots(raw: &[String]) -> Result<ParsedSlots, SlotParseError> {
    crate::domain::value_objects::parse_scheduled_times(raw)
        .map(ParsedSlots::from)
        .map_err(SlotParseError::from)
}

pub fn validate_slot_count(selected_frequency: usize, count: usize) -> Result<(), String> {
    let valid = match selected_frequency {
        0 => count == 1,
        1 => count == 2,
        2 => count == 3,
        _ => count >= 4,
    };
    if valid {
        Ok(())
    } else {
        let msg = match selected_frequency {
            3 => "Custom frequency requires at least 4 scheduled times".into(),
            n => format!("Please provide {} scheduled time(s)", n + 1),
        };
        Err(msg)
    }
}

pub fn frequency_str(selected_frequency: usize) -> &'static str {
    match selected_frequency {
        0 => "OnceDaily",
        1 => "TwiceDaily",
        2 => "ThriceDaily",
        _ => "Custom",
    }
}
