use thiserror::Error;

/// All domain-level errors that can arise during invariant validation or
/// state-transition enforcement.
///
/// Every public constructor and mutating method that enforces a business rule
/// returns `Result<_, DomainError>` so callers know exactly which invariant
/// was violated without parsing error strings.
#[derive(Debug, Error, PartialEq)]
pub enum DomainError {
    /// Returned by [`Dosage::new`](crate::domain::value_objects::dosage::Dosage::new)
    /// when the supplied amount is `0`. A dosage must be at least 1 mg.
    #[error("invalid dosage: amount must be greater than zero")]
    InvalidDosage,

    /// Returned by
    /// [`MedicationName::new`](crate::domain::value_objects::medication_name::MedicationName::new)
    /// when the supplied string is empty or contains only whitespace.
    #[error("medication name must not be empty")]
    EmptyMedicationName,

    /// Returned by
    /// [`ScheduledTime::new`](crate::domain::value_objects::scheduled_time::ScheduledTime::new)
    /// when `hour` ≥ 24 or `minute` ≥ 60.
    #[error("invalid scheduled time")]
    InvalidScheduledTime,

    /// Returned by
    /// [`DoseRecord::mark_taken`](crate::domain::entities::dose_record::DoseRecord::mark_taken)
    /// when the record was already marked as taken. A dose can only be taken once.
    #[error("dose has already been taken")]
    DoseAlreadyTaken,

    /// Returned by
    /// [`TakenAt::new`](crate::domain::value_objects::taken_at::TakenAt::new)
    /// when `hour` ≥ 24 or `minute` ≥ 60.
    #[error("invalid taken-at time: hour must be 0–23 and minute 0–59")]
    InvalidTakenAt,

    /// Returned by
    /// [`TakenAt::new`](crate::domain::value_objects::taken_at::TakenAt::new)
    /// when the supplied datetime is strictly after `now`. A dose cannot be
    /// logged as taken in the future.
    #[error("taken-at time cannot be in the future")]
    TakenAtInFuture,

    /// Returned by
    /// [`NavigationMode::try_from`](crate::domain::value_objects::navigation_mode::NavigationMode::try_from)
    /// when the supplied string is not "vi" or "emacs".
    #[error("invalid navigation mode")]
    InvalidNavigationMode,

    /// Returned by
    /// [`Medication::new`](crate::domain::entities::medication::Medication::new)
    /// when the number of scheduled times does not match the dose frequency.
    #[error("scheduled times count does not match dose frequency")]
    InvalidScheduledTimesCount,

    /// Returned by
    /// [`Medication::new`](crate::domain::entities::medication::Medication::new)
    /// when using `Custom` frequency with fewer than 4 scheduled times.
    #[error("custom frequency requires at least 4 scheduled times")]
    CustomFrequencyRequiresMinimumFourTimes,

    /// Returned by
    /// [`Medication::new`](crate::domain::entities::medication::Medication::new)
    /// when the scheduled times list contains duplicate entries.
    #[error("scheduled times must not contain duplicates")]
    DuplicateScheduledTime,

    /// Returned when parsing a custom scheduled time that fails validation.
    #[error("invalid scheduled time: {0}")]
    InvalidScheduledTimeCustom(String),

    /// Returned by
    /// [`StockQuantity::consume`](crate::domain::value_objects::stock_quantity::StockQuantity::consume)
    /// when attempting to consume more pills than available in stock.
    #[error("stock cannot be negative")]
    StockCannotBeNegative,
}
