use crate::application::errors::StorageError;
use crate::domain::{entities::medication::Medication, value_objects::medication_id::MedicationId};

pub trait MedicationRepository: Send + Sync {
    fn save(&self, medication: &Medication) -> Result<(), StorageError>;
    fn find_by_id(&self, id: &MedicationId) -> Result<Option<Medication>, StorageError>;
    fn find_all(&self) -> Result<Vec<Medication>, StorageError>;
    fn delete(&self, id: &MedicationId) -> Result<(), StorageError>;
}
