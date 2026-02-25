use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("invalid dosage: amount must be greater than zero")]
    InvalidDosage,
    #[error("medication name must not be empty")]
    EmptyMedicationName,
    #[error("invalid scheduled time")]
    InvalidScheduledTime,
    #[error("dose has already been taken")]
    DoseAlreadyTaken,
}
