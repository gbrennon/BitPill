use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::sync::Mutex;

use crate::application::errors::StorageError;
use crate::application::ports::medication_repository_port::MedicationRepository;
use crate::domain::{entities::medication::Medication, value_objects::medication_id::MedicationId};

const MEDICATIONS_FILE: &str = "medications.json";

pub struct JsonMedicationRepository {
    medications: Mutex<Vec<Medication>>,
}

impl JsonMedicationRepository {
    pub fn new() -> Self {
        let medications = Self::load_from_file().unwrap_or_default();
        Self {
            medications: Mutex::new(medications),
        }
    }

    fn load_from_file() -> Result<Vec<Medication>, StorageError> {
        let mut file = match File::open(MEDICATIONS_FILE) {
            Ok(f) => f,
            Err(_) => return Ok(Vec::new()),
        };
        let mut data = String::new();
        file.read_to_string(&mut data).map_err(|e| StorageError(format!("IO error: {e}")))?;
        serde_json::from_str(&data).map_err(|e| StorageError(format!("Deserialization error: {e}")))
    }

    fn save_to_file(&self) -> Result<(), StorageError> {
        let medications = self.medications.lock().unwrap();
        let data = serde_json::to_string(&*medications).map_err(|e| StorageError(format!("Serialization error: {e}")))?;
        let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(MEDICATIONS_FILE).map_err(|e| StorageError(format!("IO error: {e}")))?;
        file.write_all(data.as_bytes()).map_err(|e| StorageError(format!("IO error: {e}")))
    }
}

impl MedicationRepository for JsonMedicationRepository {
    fn save(&self, medication: &Medication) -> Result<(), StorageError> {
        let mut medications = self.medications.lock().unwrap();
        if let Some(existing) = medications.iter_mut().find(|m| m.id() == medication.id()) {
            *existing = medication.clone();
        } else {
            medications.push(medication.clone());
        }
        self.save_to_file()
    }

    fn find_by_id(&self, id: &MedicationId) -> Result<Option<Medication>, StorageError> {
        let medications = self.medications.lock().unwrap();
        Ok(medications.iter().find(|m| m.id() == id).cloned())
    }

    fn find_all(&self) -> Result<Vec<Medication>, StorageError> {
        let medications = self.medications.lock().unwrap();
        Ok(medications.clone())
    }

    fn delete(&self, id: &MedicationId) -> Result<(), StorageError> {
        let mut medications = self.medications.lock().unwrap();
        medications.retain(|m| m.id() != id);
        self.save_to_file()
    }
}
