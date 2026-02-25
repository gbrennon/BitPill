use std::collections::HashMap;
use std::sync::RwLock;

use crate::application::ports::medication_repository::{MedicationRepository, RepositoryError};
use crate::domain::{entities::medication::Medication, value_objects::medication_id::MedicationId};

pub struct InMemoryMedicationRepository {
    store: RwLock<HashMap<String, Medication>>,
}

impl InMemoryMedicationRepository {
    pub fn new() -> Self {
        Self {
            store: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryMedicationRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl MedicationRepository for InMemoryMedicationRepository {
    fn save(&self, medication: &Medication) -> Result<(), RepositoryError> {
        self.store
            .write()
            .map_err(|e| RepositoryError::StorageError(e.to_string()))?
            .insert(medication.id().to_string(), medication.clone());
        Ok(())
    }

    fn find_by_id(&self, id: &MedicationId) -> Result<Option<Medication>, RepositoryError> {
        Ok(self
            .store
            .read()
            .map_err(|e| RepositoryError::StorageError(e.to_string()))?
            .get(&id.to_string())
            .cloned())
    }

    fn find_all(&self) -> Result<Vec<Medication>, RepositoryError> {
        Ok(self
            .store
            .read()
            .map_err(|e| RepositoryError::StorageError(e.to_string()))?
            .values()
            .cloned()
            .collect())
    }

    fn delete(&self, id: &MedicationId) -> Result<(), RepositoryError> {
        self.store
            .write()
            .map_err(|e| RepositoryError::StorageError(e.to_string()))?
            .remove(&id.to_string());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::{
        dosage::Dosage, medication_name::MedicationName, scheduled_time::ScheduledTime,
    };

    fn make_medication(name: &str) -> Medication {
        Medication::new(
            MedicationName::new(name).unwrap(),
            Dosage::new(500).unwrap(),
            vec![ScheduledTime::new(8, 0).unwrap()],
        )
    }

    #[test]
    fn save_stores_medication_retrievable_by_id() {
        let repo = InMemoryMedicationRepository::new();
        let med = make_medication("Aspirin");

        repo.save(&med).unwrap();

        let found = repo.find_by_id(med.id()).unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().name().value(), "Aspirin");
    }

    #[test]
    fn find_by_id_returns_none_when_not_found() {
        let repo = InMemoryMedicationRepository::new();
        let id = MedicationId::new();

        let result = repo.find_by_id(&id).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn find_all_returns_all_saved_medications() {
        let repo = InMemoryMedicationRepository::new();
        repo.save(&make_medication("Aspirin")).unwrap();
        repo.save(&make_medication("Ibuprofen")).unwrap();

        let all = repo.find_all().unwrap();

        assert_eq!(all.len(), 2);
    }

    #[test]
    fn delete_removes_medication_from_store() {
        let repo = InMemoryMedicationRepository::new();
        let med = make_medication("Aspirin");
        repo.save(&med).unwrap();

        repo.delete(med.id()).unwrap();

        assert!(repo.find_by_id(med.id()).unwrap().is_none());
    }

    #[test]
    fn delete_nonexistent_id_succeeds_silently() {
        let repo = InMemoryMedicationRepository::new();
        let id = MedicationId::new();

        assert!(repo.delete(&id).is_ok());
    }

    #[test]
    fn find_all_returns_empty_vec_when_store_is_empty() {
        let repo = InMemoryMedicationRepository::new();

        assert!(repo.find_all().unwrap().is_empty());
    }
}
