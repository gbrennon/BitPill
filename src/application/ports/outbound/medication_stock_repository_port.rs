use crate::{
    application::errors::StorageError,
    domain::{
        entities::medication_stock::MedicationStock, value_objects::medication_id::MedicationId,
    },
};

pub trait MedicationStockRepositoryPort: Send + Sync {
    fn save(&self, medication_stock: &MedicationStock) -> Result<(), StorageError>;
    fn find_by_medication_id(
        &self,
        medication_id: &MedicationId,
    ) -> Result<Option<MedicationStock>, StorageError>;
}
