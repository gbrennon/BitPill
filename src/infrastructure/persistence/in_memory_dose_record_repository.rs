use std::collections::HashMap;
use std::sync::RwLock;

use crate::application::ports::dose_record_repository::{
    DoseRecordRepository, DoseRecordRepositoryError,
};
use crate::domain::{
    entities::dose_record::DoseRecord,
    value_objects::{dose_record_id::DoseRecordId, medication_id::MedicationId},
};

pub struct InMemoryDoseRecordRepository {
    store: RwLock<HashMap<String, DoseRecord>>,
}

impl InMemoryDoseRecordRepository {
    pub fn new() -> Self {
        Self {
            store: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryDoseRecordRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl DoseRecordRepository for InMemoryDoseRecordRepository {
    fn save(&self, record: &DoseRecord) -> Result<(), DoseRecordRepositoryError> {
        self.store
            .write()
            .map_err(|e| DoseRecordRepositoryError::StorageError(e.to_string()))?
            .insert(record.id().to_string(), record.clone());
        Ok(())
    }

    fn find_by_id(
        &self,
        id: &DoseRecordId,
    ) -> Result<Option<DoseRecord>, DoseRecordRepositoryError> {
        Ok(self
            .store
            .read()
            .map_err(|e| DoseRecordRepositoryError::StorageError(e.to_string()))?
            .get(&id.to_string())
            .cloned())
    }

    fn find_all_by_medication(
        &self,
        medication_id: &MedicationId,
    ) -> Result<Vec<DoseRecord>, DoseRecordRepositoryError> {
        Ok(self
            .store
            .read()
            .map_err(|e| DoseRecordRepositoryError::StorageError(e.to_string()))?
            .values()
            .filter(|r| r.medication_id() == medication_id)
            .cloned()
            .collect())
    }

    fn delete(&self, id: &DoseRecordId) -> Result<(), DoseRecordRepositoryError> {
        self.store
            .write()
            .map_err(|e| DoseRecordRepositoryError::StorageError(e.to_string()))?
            .remove(&id.to_string());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn make_datetime(h: u32) -> chrono::NaiveDateTime {
        NaiveDate::from_ymd_opt(2025, 1, 1)
            .unwrap()
            .and_hms_opt(h, 0, 0)
            .unwrap()
    }

    fn make_record(medication_id: MedicationId) -> DoseRecord {
        DoseRecord::new(medication_id, make_datetime(8))
    }

    #[test]
    fn save_stores_record_retrievable_by_id() {
        let repo = InMemoryDoseRecordRepository::new();
        let record = make_record(MedicationId::new());

        repo.save(&record).unwrap();

        let found = repo.find_by_id(record.id()).unwrap();
        assert!(found.is_some());
    }

    #[test]
    fn find_by_id_returns_none_when_not_found() {
        let repo = InMemoryDoseRecordRepository::new();
        let id = DoseRecordId::new();

        assert!(repo.find_by_id(&id).unwrap().is_none());
    }

    #[test]
    fn find_all_by_medication_returns_only_matching_records() {
        let repo = InMemoryDoseRecordRepository::new();
        let med_id = MedicationId::new();
        let other_id = MedicationId::new();
        repo.save(&make_record(med_id.clone())).unwrap();
        repo.save(&make_record(med_id.clone())).unwrap();
        repo.save(&make_record(other_id)).unwrap();

        let results = repo.find_all_by_medication(&med_id).unwrap();

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn save_overwrites_existing_record_with_same_id() {
        let repo = InMemoryDoseRecordRepository::new();
        let mut record = make_record(MedicationId::new());
        repo.save(&record).unwrap();

        record.mark_taken(make_datetime(8)).unwrap();
        repo.save(&record).unwrap();

        let found = repo.find_by_id(record.id()).unwrap().unwrap();
        assert!(found.is_taken());
    }

    #[test]
    fn delete_removes_record_from_store() {
        let repo = InMemoryDoseRecordRepository::new();
        let record = make_record(MedicationId::new());
        repo.save(&record).unwrap();

        repo.delete(record.id()).unwrap();

        assert!(repo.find_by_id(record.id()).unwrap().is_none());
    }

    #[test]
    fn find_all_by_medication_returns_empty_when_none_match() {
        let repo = InMemoryDoseRecordRepository::new();
        let id = MedicationId::new();

        assert!(repo.find_all_by_medication(&id).unwrap().is_empty());
    }
}
