use chrono::NaiveTime;

use crate::domain::errors::DomainError;

/// A validated clock time at which a dose is scheduled (hour and minute only).
///
/// `ScheduledTime` is a value object wrapping [`NaiveTime`] with seconds fixed
/// at zero. Instances are comparable and sortable so a medication's schedule
/// can be ordered chronologically.
///
/// # Invariants
///
/// - `hour` must be in `0..=23`.
/// - `minute` must be in `0..=59`.
///
/// # Examples
///
/// ```rust
/// use bitpill::domain::{value_objects::scheduled_time::ScheduledTime, errors::DomainError};
///
/// let morning = ScheduledTime::new(8, 0).unwrap();
/// let evening = ScheduledTime::new(20, 30).unwrap();
///
/// assert_eq!(morning.to_string(), "08:00");
/// assert_eq!(evening.to_string(), "20:30");
/// assert!(morning < evening);
///
/// assert!(matches!(ScheduledTime::new(24, 0), Err(DomainError::InvalidScheduledTime)));
/// assert!(matches!(ScheduledTime::new(8, 60), Err(DomainError::InvalidScheduledTime)));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScheduledTime(NaiveTime);

impl ScheduledTime {
    /// Creates a new `ScheduledTime` from an `hour` (0–23) and `minute` (0–59).
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::InvalidScheduledTime`] when either value is out of range.
    pub fn new(hour: u32, minute: u32) -> Result<Self, DomainError> {
        NaiveTime::from_hms_opt(hour, minute, 0)
            .map(Self)
            .ok_or(DomainError::InvalidScheduledTime)
    }

    /// Returns the underlying [`NaiveTime`].
    pub fn value(&self) -> NaiveTime {
        self.0
    }
}

impl std::fmt::Display for ScheduledTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format("%H:%M"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_with_valid_hour_and_minute_returns_scheduled_time() {
        let result = ScheduledTime::new(8, 0);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string(), "08:00");
    }

    #[test]
    fn new_with_invalid_hour_returns_error() {
        let result = ScheduledTime::new(24, 0);

        assert!(matches!(result, Err(DomainError::InvalidScheduledTime)));
    }

    #[test]
    fn new_with_invalid_minute_returns_error() {
        let result = ScheduledTime::new(8, 60);

        assert!(matches!(result, Err(DomainError::InvalidScheduledTime)));
    }

    #[test]
    fn display_formats_as_hh_mm() {
        let time = ScheduledTime::new(20, 30).unwrap();

        assert_eq!(time.to_string(), "20:30");
    }

    #[test]
    fn value_returns_the_inner_naive_time() {
        let time = ScheduledTime::new(8, 0).unwrap();

        assert_eq!(time.value(), NaiveTime::from_hms_opt(8, 0, 0).unwrap());
    }

    #[test]
    fn earlier_time_is_less_than_later_time() {
        let morning = ScheduledTime::new(8, 0).unwrap();
        let evening = ScheduledTime::new(20, 0).unwrap();

        assert!(morning < evening);
    }
}
