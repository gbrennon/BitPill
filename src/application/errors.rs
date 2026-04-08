use thiserror::Error;

use crate::domain::errors::DomainError;

/// Top-level error type for application use cases.
#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error("multiple validation errors")]
    MultipleDomainErrors { errors: Vec<DomainError> },
    #[error(transparent)]
    Storage(#[from] StorageError),
    #[error(transparent)]
    NotFound(#[from] NotFoundError),
    #[error(transparent)]
    Delivery(#[from] DeliveryError),
    #[error("invalid input: {0}")]
    InvalidInput(String),
}

/// Storage operation failure (read, write, or delete).
///
/// Use when a repository operation fails.
#[derive(Debug, Error, Clone)]
#[error("storage error: {0}")]
pub struct StorageError(pub String);

/// A required resource does not exist.
///
/// Use when attempting to access a non-existent resource.
#[derive(Debug, Error, PartialEq, Eq)]
#[error("not found")]
pub struct NotFoundError;

/// An operation conflicts with existing state (e.g. duplicate entity).
///
/// Use when an operation would create duplicate state.
#[derive(Debug, Error, PartialEq, Eq)]
#[error("conflict")]
pub struct ConflictError;

/// Notification delivery failure.
///
/// Use when notification delivery fails (e.g. unable to send reminder).
#[derive(Debug, Error)]
#[error("delivery failed: {0}")]
pub struct DeliveryError(pub String);
