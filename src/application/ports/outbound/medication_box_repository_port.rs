use crate::{
    application::errors::StorageError,
    domain::{
        entities::medication_box::MedicationBox, value_objects::medication_box_id::MedicationBoxId,
    },
};

pub trait MedicationBoxRepositoryPort: Send + Sync {
    fn save(&self, medication_box: &MedicationBox) -> Result<(), StorageError>;
    fn find_by_id(&self, id: &MedicationBoxId) -> Result<Option<MedicationBox>, StorageError>;
    fn find_all(&self) -> Result<Vec<MedicationBox>, StorageError>;
    fn delete(&self, id: &MedicationBoxId) -> Result<(), StorageError>;
}
