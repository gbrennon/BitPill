use std::sync::Mutex;

use crate::{
    application::{errors::StorageError, ports::medication_repository_port::MedicationRepository},
    domain::{entities::medication::Medication, value_objects::medication_id::MedicationId},
};

pub struct FakeMedicationRepository {
    medications: Mutex<Vec<Medication>>,
    fail_on_save: bool,
    deleted_count: Mutex<usize>,
}

impl Default for FakeMedicationRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl FakeMedicationRepository {
    pub fn new() -> Self {
        Self {
            medications: Mutex::new(Vec::new()),
            fail_on_save: false,
            deleted_count: Mutex::new(0),
        }
    }

    pub fn with(medications: Vec<Medication>) -> Self {
        Self {
            medications: Mutex::new(medications),
            fail_on_save: false,
            deleted_count: Mutex::new(0),
        }
    }

    pub fn failing() -> Self {
        Self {
            medications: Mutex::new(Vec::new()),
            fail_on_save: true,
            deleted_count: Mutex::new(0),
        }
    }

    pub fn saved_count(&self) -> usize {
        self.medications.lock().unwrap().len()
    }

    pub fn deleted_count(&self) -> usize {
        *self.deleted_count.lock().unwrap()
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

    fn delete(&self, id: &MedicationId) -> Result<(), StorageError> {
        let mut meds = self.medications.lock().unwrap();
        let before = meds.len();
        meds.retain(|m| m.id() != id);
        let after = meds.len();
        let removed = before.saturating_sub(after);
        if removed > 0 {
            let mut cnt = self.deleted_count.lock().unwrap();
            *cnt += removed;
        }
        Ok(())
    }
}
