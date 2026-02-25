use thiserror::Error;

use crate::domain::{
    entities::dose_record::DoseRecord, value_objects::dose_record_id::DoseRecordId,
    value_objects::medication_id::MedicationId,
};

#[derive(Debug, Error)]
pub enum DoseRecordRepositoryError {
    #[error("dose record not found")]
    NotFound,
    #[error("storage error: {0}")]
    StorageError(String),
}

pub trait DoseRecordRepository: Send + Sync {
    fn save(&self, record: &DoseRecord) -> Result<(), DoseRecordRepositoryError>;
    fn find_by_id(
        &self,
        id: &DoseRecordId,
    ) -> Result<Option<DoseRecord>, DoseRecordRepositoryError>;
    fn find_all_by_medication(
        &self,
        medication_id: &MedicationId,
    ) -> Result<Vec<DoseRecord>, DoseRecordRepositoryError>;
    fn delete(&self, id: &DoseRecordId) -> Result<(), DoseRecordRepositoryError>;
}
