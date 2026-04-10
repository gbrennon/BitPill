use crate::{
    application::errors::StorageError, domain::entities::medication_refill::MedicationRefill,
};

pub trait MedicationRefillRepositoryPort: Send + Sync {
    fn save(&self, refill: &MedicationRefill) -> Result<(), StorageError>;
    fn find_by_id(&self, id: &MedicationRefill) -> Result<Option<MedicationRefill>, StorageError>;
}
