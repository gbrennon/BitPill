use crate::{
    application::errors::StorageError, domain::entities::medication_stock::MedicationStock,
};

pub trait MedicationStockRepositoryPort: Send + Sync {
    fn save(&self, medication_stock: &MedicationStock) -> Result<(), StorageError>;
    fn find_by_id(&self, id: &MedicationStock) -> Result<Option<MedicationStock>, StorageError>;
}
