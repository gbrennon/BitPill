use std::sync::Mutex;

use crate::application::errors::StorageError;
use crate::application::ports::medication_repository_port::MedicationRepository;
use crate::domain::{entities::medication::Medication, value_objects::medication_id::MedicationId};

pub struct FakeMedicationRepository {
    medications: Mutex<Vec<Medication>>,
    fail_on_save: bool,
}

impl FakeMedicationRepository {
    pub fn new() -> Self {
        Self {
            medications: Mutex::new(Vec::new()),
            fail_on_save: false,
        }
    }

    pub fn with(medications: Vec<Medication>) -> Self {
        Self {
            medications: Mutex::new(medications),
            fail_on_save: false,
        }
    }

    pub fn failing() -> Self {
        Self {
            medications: Mutex::new(Vec::new()),
            fail_on_save: true,
        }
    }

    pub fn saved_count(&self) -> usize {
        self.medications.lock().unwrap().len()
    }
}

impl MedicationRepository for FakeMedicationRepository {
    fn save(&self, medication: &Medication) -> Result<(), StorageError> {
        if self.fail_on_save {
            return Err(StorageError("forced failure".into()));
        }
        self.medications.lock().unwrap().push(medication.clone());
        Ok(())
    }

    fn find_by_id(&self, id: &MedicationId) -> Result<Option<Medication>, StorageError> {
        Ok(self
            .medications
            .lock()
            .unwrap()
            .iter()
            .find(|m| m.id() == id)
            .cloned())
    }

    fn find_all(&self) -> Result<Vec<Medication>, StorageError> {
        Ok(self.medications.lock().unwrap().clone())
    }

    fn delete(&self, _id: &MedicationId) -> Result<(), StorageError> {
        Ok(())
    }
}
