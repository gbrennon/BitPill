use std::sync::Mutex;

use bitpill::{
    application::{
        errors::StorageError, ports::outbound::medication_repository_port::MedicationRepository,
    },
    domain::{entities::medication::Medication, value_objects::medication_id::MedicationId},
};

pub struct FakeMedicationRepository {
    medications: Mutex<Vec<Medication>>,
    fail_on_save: bool,
    fail_on_find_all: bool,
    fail_on_find_by_id: bool,
    fail_on_delete: bool,
    custom_find_by_id: Mutex<Option<Medication>>,
}

impl FakeMedicationRepository {
    pub fn new() -> Self {
        Self {
            medications: Mutex::new(Vec::new()),
            fail_on_save: false,
            fail_on_find_all: false,
            fail_on_find_by_id: false,
            fail_on_delete: false,
        }
    }

    pub fn with(medications: Vec<Medication>) -> Self {
        Self {
            medications: Mutex::new(medications),
            fail_on_save: false,
            fail_on_find_all: false,
            fail_on_find_by_id: false,
            fail_on_delete: false,
        }
    }

    pub fn failing() -> Self {
        Self {
            medications: Mutex::new(Vec::new()),
            fail_on_save: true,
            fail_on_find_all: false,
            fail_on_find_by_id: false,
            fail_on_delete: false,
        }
    }

    pub fn failing_on_find_all() -> Self {
        Self {
            medications: Mutex::new(Vec::new()),
            fail_on_save: false,
            fail_on_find_all: true,
            fail_on_find_by_id: false,
            fail_on_delete: false,
        }
    }

    pub fn failing_on_find_by_id() -> Self {
        Self {
            medications: Mutex::new(Vec::new()),
            fail_on_save: false,
            fail_on_find_all: false,
            fail_on_find_by_id: true,
            fail_on_delete: false,
        }
    }

    pub fn failing_on_delete() -> Self {
        Self {
            medications: Mutex::new(Vec::new()),
            fail_on_save: false,
            fail_on_find_all: false,
            fail_on_find_by_id: false,
            fail_on_delete: true,
            custom_find_by_id: Mutex::new(None),
        }
    }

    pub fn set_find_by_id_result(&mut self, medication: Option<Medication>) {
        *self.custom_find_by_id.lock().unwrap() = medication;
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
        if self.fail_on_find_by_id {
            return Err(StorageError("forced find_by_id failure".into()));
        }
        if let Some(med) = self.custom_find_by_id.lock().unwrap().take() {
            return Ok(Some(med));
        }
        Ok(self
            .medications
            .lock()
            .unwrap()
            .iter()
            .find(|m| m.id() == id)
            .cloned())
    }

    fn find_all(&self) -> Result<Vec<Medication>, StorageError> {
        if self.fail_on_find_all {
            return Err(StorageError("forced find_all failure".into()));
        }
        Ok(self.medications.lock().unwrap().clone())
    }

    fn delete(&self, id: &MedicationId) -> Result<(), StorageError> {
        if self.fail_on_delete {
            return Err(StorageError("forced delete failure".into()));
        }
        let mut meds = self.medications.lock().unwrap();
        meds.retain(|m| m.id() != id);
        Ok(())
    }
}
