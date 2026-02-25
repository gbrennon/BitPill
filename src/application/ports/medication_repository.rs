use thiserror::Error;

use crate::domain::{entities::medication::Medication, value_objects::medication_id::MedicationId};

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("medication not found")]
    NotFound,
    #[error("storage error: {0}")]
    StorageError(String),
}

pub trait MedicationRepository: Send + Sync {
    fn save(&self, medication: &Medication) -> Result<(), RepositoryError>;
    fn find_by_id(&self, id: &MedicationId) -> Result<Option<Medication>, RepositoryError>;
    fn find_all(&self) -> Result<Vec<Medication>, RepositoryError>;
    fn delete(&self, id: &MedicationId) -> Result<(), RepositoryError>;
}
