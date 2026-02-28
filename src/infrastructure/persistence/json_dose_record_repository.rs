use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::sync::Mutex;

use crate::application::errors::StorageError;
use crate::application::ports::dose_record_repository_port::DoseRecordRepository;
use crate::domain::{
    entities::dose_record::DoseRecord,
    value_objects::{dose_record_id::DoseRecordId, medication_id::MedicationId},
};

const DOSE_RECORDS_FILE: &str = "dose_records.json";

pub struct JsonDoseRecordRepository {
    records: Mutex<Vec<DoseRecord>>,
}

impl JsonDoseRecordRepository {
    pub fn new() -> Self {
        let records = Self::load_from_file().unwrap_or_default();
        Self {
            records: Mutex::new(records),
        }
    }

    fn load_from_file() -> Result<Vec<DoseRecord>, StorageError> {
        let mut file = match File::open(DOSE_RECORDS_FILE) {
            Ok(f) => f,
            Err(_) => return Ok(Vec::new()),
        };
        let mut data = String::new();
        file.read_to_string(&mut data).map_err(|e| StorageError(format!("IO error: {e}")))?;
        serde_json::from_str(&data).map_err(|e| StorageError(format!("Deserialization error: {e}")))
    }

    fn save_to_file(&self) -> Result<(), StorageError> {
        let records = self.records.lock().unwrap();
        let data = serde_json::to_string(&*records).map_err(|e| StorageError(format!("Serialization error: {e}")))?;
        let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(DOSE_RECORDS_FILE).map_err(|e| StorageError(format!("IO error: {e}")))?;
        file.write_all(data.as_bytes()).map_err(|e| StorageError(format!("IO error: {e}")))
    }
}

impl DoseRecordRepository for JsonDoseRecordRepository {
    fn save(&self, record: &DoseRecord) -> Result<(), StorageError> {
        let mut records = self.records.lock().unwrap();
        if let Some(existing) = records.iter_mut().find(|r| r.id() == record.id()) {
            *existing = record.clone();
        } else {
            records.push(record.clone());
        }
        self.save_to_file()
    }

    fn find_by_id(&self, id: &DoseRecordId) -> Result<Option<DoseRecord>, StorageError> {
        let records = self.records.lock().unwrap();
        Ok(records.iter().find(|r| r.id() == id).cloned())
    }

    fn find_all_by_medication(&self, medication_id: &MedicationId) -> Result<Vec<DoseRecord>, StorageError> {
        let records = self.records.lock().unwrap();
        Ok(records.iter().filter(|r| r.medication_id() == medication_id).cloned().collect())
    }

    fn delete(&self, id: &DoseRecordId) -> Result<(), StorageError> {
        let mut records = self.records.lock().unwrap();
        records.retain(|r| r.id() != id);
        self.save_to_file()
    }
}
