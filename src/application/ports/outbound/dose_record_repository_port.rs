use crate::{
    application::errors::StorageError,
    domain::{
        entities::dose_record::DoseRecord,
        value_objects::{dose_record_id::DoseRecordId, medication_id::MedicationId},
    },
};

pub trait DoseRecordRepository: Send + Sync {
    /// Persists a dose record. Acts as an **upsert**: inserts the record if no
    /// entry with the same `id` exists, or replaces the existing one if it does.
    fn save(&self, record: &DoseRecord) -> Result<(), StorageError>;
    fn find_by_id(&self, id: &DoseRecordId) -> Result<Option<DoseRecord>, StorageError>;
    fn find_all_by_medication(
        &self,
        medication_id: &MedicationId,
    ) -> Result<Vec<DoseRecord>, StorageError>;
    fn delete(&self, id: &DoseRecordId) -> Result<(), StorageError>;
}
