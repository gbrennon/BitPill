use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("invalid dosage: amount must be greater than zero")]
    InvalidDosage,
    #[error("pill name must not be empty")]
    EmptyPillName,
}
