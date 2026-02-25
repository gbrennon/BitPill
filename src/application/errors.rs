use thiserror::Error;

/// Errors representing application-level rule violations.
///
/// These are distinct from domain validation errors ([`DomainError`]) and
/// infrastructure errors (repository / port errors):
///
/// | Layer          | Error type              | Examples                                  |
/// |----------------|-------------------------|-------------------------------------------|
/// | Domain         | `DomainError`           | zero dosage, empty name, already taken    |
/// | Application    | `ApplicationError`      | resource not found, operation not allowed |
/// | Infrastructure | `RepositoryError`, etc. | storage failure, I/O error                |
///
/// [`DomainError`]: crate::domain::errors::DomainError
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ApplicationError {
    /// A required resource does not exist.
    ///
    /// Raised when an operation requires a pre-existing entity (e.g. a
    /// [`DoseRecord`] or [`Medication`]) that cannot be found.
    ///
    /// [`DoseRecord`]: crate::domain::entities::dose_record::DoseRecord
    /// [`Medication`]: crate::domain::entities::medication::Medication
    #[error("not found")]
    NotFound,
}
