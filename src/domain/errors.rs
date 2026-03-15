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

    /// Returned when a domain value object increase is bigger than the maximum allowed value.
    #[error("stock amount cannot exceed 255")]
    StockAmountOverflow,

    /// Returned when stock decrease is bigger than the amount registered.
    #[error("decrease stock amount cannot be bigger than the amount registered.")]
    StockAmountNotEnough
}
