use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
    sync::Mutex,
};

use crate::{
    application::{errors::StorageError, ports::dose_record_repository_port::DoseRecordRepository},
    domain::{
        entities::dose_record::DoseRecord,
        value_objects::{dose_record_id::DoseRecordId, medication_id::MedicationId},
    },
};

pub struct JsonDoseRecordRepository {
    path: PathBuf,
    records: Mutex<Vec<DoseRecord>>,
}

impl Default for JsonDoseRecordRepository {
    fn default() -> Self {
        Self::with_default_path()
    }
}

impl JsonDoseRecordRepository {
    pub fn new(path: PathBuf) -> Self {
        let records = Self::load_from_path(&path).unwrap_or_default();
        Self {
            path,
            records: Mutex::new(records),
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn with_default_path() -> Self {
        let path = std::env::var("BITPILL_DOSE_RECORDS_FILE")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("dose_records.json"));
        Self::new(path)
    }

    fn load_from_path(path: &PathBuf) -> Result<Vec<DoseRecord>, StorageError> {
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(_) => return Ok(Vec::new()),
        };
        let mut data = String::new();
        file.read_to_string(&mut data)
            .map_err(|e| StorageError(format!("IO error: {e}")))?;
        serde_json::from_str(&data).map_err(|e| StorageError(format!("Deserialization error: {e}")))
    }

    fn write_records_to_path(&self, records: &[DoseRecord]) -> Result<(), StorageError> {
        let data = serde_json::to_string(records)
            .map_err(|e| StorageError(format!("Serialization error: {e}")))?;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)
            .map_err(|e| StorageError(format!("IO error: {e}")))?;
        file.write_all(data.as_bytes())
            .map_err(|e| StorageError(format!("IO error: {e}")))
    }
}

impl DoseRecordRepository for JsonDoseRecordRepository {
    fn save(&self, record: &DoseRecord) -> Result<(), StorageError> {
        let mut records_guard = self.records.lock().unwrap();
        if let Some(existing) = records_guard.iter_mut().find(|r| r.id() == record.id()) {
            *existing = record.clone();
        } else {
            records_guard.push(record.clone());
        }
        let to_write = records_guard.clone();
        drop(records_guard);
        self.write_records_to_path(&to_write)
    }

    fn find_by_id(&self, id: &DoseRecordId) -> Result<Option<DoseRecord>, StorageError> {
        let records = self.records.lock().unwrap();
        Ok(records.iter().find(|r| r.id() == id).cloned())
    }

    fn find_all_by_medication(
        &self,
        medication_id: &MedicationId,
    ) -> Result<Vec<DoseRecord>, StorageError> {
        // This will list DoseReord in the inverse order
        let records = self.records.lock().unwrap();
        Ok(records
            .iter()
            .filter(|r| r.medication_id() == medication_id)
            .cloned()
            .rev()
            .collect())
    }

    fn delete(&self, id: &DoseRecordId) -> Result<(), StorageError> {
        let mut records = self.records.lock().unwrap();
        records.retain(|r| r.id() != id);
        let to_write = records.clone();
        drop(records);
        self.write_records_to_path(&to_write)
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use tempfile::tempdir;

    use super::*;
    use crate::{
        application::ports::dose_record_repository_port::DoseRecordRepository,
        domain::{entities::dose_record::DoseRecord, value_objects::medication_id::MedicationId},
    };

    fn make_med_id() -> MedicationId {
        MedicationId::from(uuid::Uuid::nil())
    }

    fn make_record() -> DoseRecord {
        let scheduled_at = NaiveDate::from_ymd_opt(2025, 1, 1)
            .unwrap()
            .and_hms_opt(9, 0, 0)
            .unwrap();
        DoseRecord::new(make_med_id(), scheduled_at)
    }

    #[test]
    fn save_and_find_by_id_roundtrip() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("doses.json");
        let repo = JsonDoseRecordRepository::new(path);
        let record = make_record();

        repo.save(&record).expect("save should succeed");
        let found = repo.find_by_id(record.id()).expect("find should succeed");

        assert!(found.is_some());
        assert_eq!(found.unwrap().id(), record.id());
    }

    #[test]
    fn find_all_by_medication_returns_matching_records() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("doses.json");
        let repo = JsonDoseRecordRepository::new(path);
        let record = make_record();

        repo.save(&record).expect("save should succeed");
        let all = repo
            .find_all_by_medication(&make_med_id())
            .expect("find_all should succeed");

        assert_eq!(all.len(), 1);
    }

    #[test]
    fn delete_removes_record_from_store() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("doses.json");
        let repo = JsonDoseRecordRepository::new(path);
        let record = make_record();

        repo.save(&record).expect("save should succeed");
        repo.delete(record.id()).expect("delete should succeed");
        let found = repo.find_by_id(record.id()).expect("find should succeed");

        assert!(found.is_none());
    }

    #[test]
    fn save_updates_existing_record_when_saved_twice() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("doses.json");
        let repo = JsonDoseRecordRepository::new(path);
        let record = make_record();

        repo.save(&record).expect("first save should succeed");
        repo.save(&record)
            .expect("second save (update path) should succeed");
        let all = repo
            .find_all_by_medication(&make_med_id())
            .expect("find_all should succeed");

        assert_eq!(all.len(), 1); // update, not insert
    }

    #[test]
    fn with_default_path_uses_env_var_when_set() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("doses_default_test.json");
        unsafe {
            std::env::set_var("BITPILL_DOSE_RECORDS_FILE", path.to_str().unwrap());
        }

        let repo1 = JsonDoseRecordRepository::with_default_path();
        let _repo2 = JsonDoseRecordRepository::default();
        let record = make_record();
        repo1.save(&record).expect("save should succeed");

        unsafe {
            std::env::remove_var("BITPILL_DOSE_RECORDS_FILE");
        }
        assert!(path.exists());
    }

    #[test]
    fn data_persists_across_separate_repository_instances() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("doses.json");
        let record = make_record();

        {
            let repo = JsonDoseRecordRepository::new(path.clone());
            repo.save(&record).expect("save should succeed");
        }

        let repo2 = JsonDoseRecordRepository::new(path);
        let found = repo2.find_by_id(record.id()).expect("find should succeed");

        assert!(found.is_some());
    }
}
