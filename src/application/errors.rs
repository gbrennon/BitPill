use thiserror::Error;

/// Infrastructure storage failure. Shared across all repository ports.
#[derive(Debug, Error)]
#[error("storage error: {0}")]
pub struct StorageError(pub String);

/// A required resource does not exist. Raised in services when a lookup returns `None`.
#[derive(Debug, Error, PartialEq, Eq)]
#[error("not found")]
pub struct NotFoundError;

/// An operation conflicts with existing state (e.g. duplicate entity).
#[derive(Debug, Error, PartialEq, Eq)]
#[error("conflict")]
pub struct ConflictError;

/// Notification delivery failure. Shared across all notification ports.
#[derive(Debug, Error)]
#[error("delivery failed: {0}")]
pub struct DeliveryError(pub String);
