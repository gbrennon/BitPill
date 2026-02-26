use crate::application::errors::StorageError;
use crate::domain::{
    entities::dose_record::DoseRecord,
    value_objects::{dose_record_id::DoseRecordId, medication_id::MedicationId},
};

pub trait DoseRecordRepository: Send + Sync {
    fn save(&self, record: &DoseRecord) -> Result<(), StorageError>;
    fn find_by_id(&self, id: &DoseRecordId) -> Result<Option<DoseRecord>, StorageError>;
    fn find_all_by_medication(
        &self,
        medication_id: &MedicationId,
    ) -> Result<Vec<DoseRecord>, StorageError>;
    fn delete(&self, id: &DoseRecordId) -> Result<(), StorageError>;
}
