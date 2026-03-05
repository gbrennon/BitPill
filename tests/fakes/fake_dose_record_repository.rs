use std::sync::Mutex;

use bitpill::application::errors::StorageError;
use bitpill::application::ports::outbound::dose_record_repository_port::DoseRecordRepository;
use bitpill::domain::{
    entities::dose_record::DoseRecord,
    value_objects::{dose_record_id::DoseRecordId, medication_id::MedicationId},
};

pub struct FakeDoseRecordRepository {
    records: Mutex<Vec<DoseRecord>>,
    fail_on_save: bool,
    fail_on_find_by_id: bool,
    fail_on_find_all_by_medication: bool,
}

impl FakeDoseRecordRepository {
    pub fn new() -> Self {
        Self {
            records: Mutex::new(Vec::new()),
            fail_on_save: false,
            fail_on_find_by_id: false,
            fail_on_find_all_by_medication: false,
        }
    }

    pub fn with(record: DoseRecord) -> Self {
        Self {
            records: Mutex::new(vec![record]),
            fail_on_save: false,
            fail_on_find_by_id: false,
            fail_on_find_all_by_medication: false,
        }
    }

    pub fn failing() -> Self {
        Self {
            records: Mutex::new(Vec::new()),
            fail_on_save: true,
            fail_on_find_by_id: false,
            fail_on_find_all_by_medication: false,
        }
    }

    pub fn with_failing(record: DoseRecord) -> Self {
        Self {
            records: Mutex::new(vec![record]),
            fail_on_save: true,
            fail_on_find_by_id: false,
            fail_on_find_all_by_medication: false,
        }
    }

    pub fn failing_on_find_by_id() -> Self {
        Self {
            records: Mutex::new(Vec::new()),
            fail_on_save: false,
            fail_on_find_by_id: true,
            fail_on_find_all_by_medication: false,
        }
    }

    pub fn failing_on_find_all_by_medication() -> Self {
        Self {
            records: Mutex::new(Vec::new()),
            fail_on_save: false,
            fail_on_find_by_id: false,
            fail_on_find_all_by_medication: true,
        }
    }

    pub fn saved_count(&self) -> usize {
        self.records.lock().unwrap().len()
    }

    pub fn find_by_id(&self, id: &DoseRecordId) -> Result<Option<DoseRecord>, StorageError> {
        Ok(self
            .records
            .lock()
            .unwrap()
            .iter()
            .find(|r| r.id() == id)
            .cloned())
    }
}

impl DoseRecordRepository for FakeDoseRecordRepository {
    fn save(&self, record: &DoseRecord) -> Result<(), StorageError> {
        if self.fail_on_save {
            return Err(StorageError("forced save failure".into()));
        }
        let mut records = self.records.lock().unwrap();
        if let Some(existing) = records.iter_mut().find(|r| r.id() == record.id()) {
            *existing = record.clone();
        } else {
            records.push(record.clone());
        }
        Ok(())
    }

    fn find_by_id(&self, id: &DoseRecordId) -> Result<Option<DoseRecord>, StorageError> {
        if self.fail_on_find_by_id {
            return Err(StorageError("forced find_by_id failure".into()));
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
        if self.fail_on_find_all_by_medication {
            return Err(StorageError("forced find_all_by_medication failure".into()));
        }
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
