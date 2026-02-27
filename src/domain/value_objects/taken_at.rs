use crate::domain::errors::DomainError;

/// The exact minute at which a medication dose was ingested.
///
/// `TakenAt` is a value object storing a calendar date and clock time as
/// primitives (year, month, day, hour, minute), with no dependency on any
/// date/time library.
///
/// # Invariants
///
/// - `hour` must be in `0..=23` and `minute` in `0..=59`.
/// - The recorded time must not be strictly after `now` (provided at
///   construction as a `(year, month, day, hour, minute)` tuple).
///
/// # Examples
///
/// ```rust
/// use bitpill::domain::{value_objects::taken_at::TakenAt, errors::DomainError};
///
/// let now = (2025_i32, 6_u8, 1_u8, 8_u8, 5_u8);
/// let taken = TakenAt::new(2025, 6, 1, 8, 0, now).unwrap();
/// assert_eq!(taken.to_string(), "2025-06-01 08:00");
///
/// assert!(matches!(
///     TakenAt::new(2025, 6, 1, 9, 0, now),
///     Err(DomainError::TakenAtInFuture)
/// ));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TakenAt {
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

impl TakenAt {
    /// Creates a new `TakenAt` from the given date and time components,
    /// validated against `now = (year, month, day, hour, minute)`.
    ///
    /// # Errors
    ///
    /// - [`DomainError::InvalidTakenAt`] — `hour` ≥ 24 or `minute` ≥ 60.
    /// - [`DomainError::TakenAtInFuture`] — the provided time is strictly
    ///   after `now`.
    pub fn new(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        now: (i32, u8, u8, u8, u8),
    ) -> Result<Self, DomainError> {
        if hour > 23 || minute > 59 {
            return Err(DomainError::InvalidTakenAt);
        }
        if (year, month, day, hour, minute) > now {
            return Err(DomainError::TakenAtInFuture);
        }
        Ok(Self { year, month, day, hour, minute })
    }

    /// Returns the year component.
    pub fn year(&self) -> i32 {
        self.year
    }

    /// Returns the month component (1–12).
    pub fn month(&self) -> u8 {
        self.month
    }

    /// Returns the day component (1–31).
    pub fn day(&self) -> u8 {
        self.day
    }

    /// Returns the hour component (0–23).
    pub fn hour(&self) -> u8 {
        self.hour
    }

    /// Returns the minute component (0–59).
    pub fn minute(&self) -> u8 {
        self.minute
    }
}

impl std::fmt::Display for TakenAt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn now() -> (i32, u8, u8, u8, u8) {
        (2025, 6, 1, 8, 30)
    }

    #[test]
    fn new_with_past_datetime_returns_taken_at() {
        let result = TakenAt::new(2025, 6, 1, 8, 0, now());

        assert!(result.is_ok());
    }

    #[test]
    fn new_with_same_minute_as_now_returns_taken_at() {
        let result = TakenAt::new(2025, 6, 1, 8, 30, now());

        assert!(result.is_ok());
    }

    #[test]
    fn new_with_future_datetime_returns_error() {
        let result = TakenAt::new(2025, 6, 1, 9, 0, now());

        assert!(matches!(result, Err(DomainError::TakenAtInFuture)));
    }

    #[test]
    fn new_with_invalid_hour_returns_error() {
        let result = TakenAt::new(2025, 6, 1, 24, 0, now());

        assert!(matches!(result, Err(DomainError::InvalidTakenAt)));
    }

    #[test]
    fn new_with_invalid_minute_returns_error() {
        let result = TakenAt::new(2025, 6, 1, 8, 60, now());

        assert!(matches!(result, Err(DomainError::InvalidTakenAt)));
    }

    #[test]
    fn accessors_return_stored_components() {
        let taken = TakenAt::new(2025, 6, 1, 8, 0, now()).unwrap();

        assert_eq!(taken.year(), 2025);
        assert_eq!(taken.month(), 6);
        assert_eq!(taken.day(), 1);
        assert_eq!(taken.hour(), 8);
        assert_eq!(taken.minute(), 0);
    }

    #[test]
    fn display_formats_as_date_and_hh_mm() {
        let taken = TakenAt::new(2025, 6, 1, 20, 30, (2025, 6, 1, 20, 35)).unwrap();

        assert_eq!(taken.to_string(), "2025-06-01 20:30");
    }

    #[test]
    fn earlier_datetime_is_less_than_later_datetime() {
        let morning = TakenAt::new(2025, 6, 1, 8, 0, now()).unwrap();
        let evening = TakenAt::new(2025, 6, 1, 8, 30, now()).unwrap();

        assert!(morning < evening);
    }
}
