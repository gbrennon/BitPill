use std::sync::Mutex;

use crate::{
    application::{errors::StorageError, ports::dose_record_repository_port::DoseRecordRepository},
    domain::{
        entities::dose_record::DoseRecord,
        value_objects::{dose_record_id::DoseRecordId, medication_id::MedicationId},
    },
};

pub struct FakeDoseRecordRepository {
    records: Mutex<Vec<DoseRecord>>,
    find_by_id_error: Mutex<Option<StorageError>>,
}

impl Default for FakeDoseRecordRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl FakeDoseRecordRepository {
    pub fn new() -> Self {
        Self {
            records: Mutex::new(Vec::new()),
            find_by_id_error: Mutex::new(None),
        }
    }

    pub fn with(record: DoseRecord) -> Self {
        Self {
            records: Mutex::new(vec![record]),
            find_by_id_error: Mutex::new(None),
        }
    }

    pub fn with_find_by_id_error(self, err: StorageError) -> Self {
        *self.find_by_id_error.lock().unwrap() = Some(err);
        self
    }

    pub fn saved_count(&self) -> usize {
        self.records.lock().unwrap().len()
    }
}

impl DoseRecordRepository for FakeDoseRecordRepository {
    fn save(&self, record: &DoseRecord) -> Result<(), StorageError> {
        let mut records = self.records.lock().unwrap();
        if let Some(existing) = records.iter_mut().find(|r| r.id() == record.id()) {
            *existing = record.clone();
        } else {
            records.push(record.clone());
        }
        Ok(())
    }

    fn find_by_id(&self, id: &DoseRecordId) -> Result<Option<DoseRecord>, StorageError> {
        if let Some(err) = self.find_by_id_error.lock().unwrap().as_ref() {
            return Err(err.clone());
        }
        Ok(self
            .records
            .lock()
            .unwrap()
            .iter()
            .find(|r| r.id() == id)
            .cloned())
    }

    fn find_all_by_medication(
        &self,
        medication_id: &MedicationId,
    ) -> Result<Vec<DoseRecord>, StorageError> {
        Ok(self
            .records
            .lock()
            .unwrap()
            .iter()
            .filter(|r| r.medication_id() == medication_id)
            .cloned()
            .collect())
    }

    fn delete(&self, _id: &DoseRecordId) -> Result<(), StorageError> {
        Ok(())
    }
}
