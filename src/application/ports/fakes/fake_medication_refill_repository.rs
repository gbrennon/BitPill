use std::sync::Mutex;

use crate::{
    application::{
        errors::StorageError,
        ports::medication_refill_repository_port::MedicationRefillRepositoryPort,
    },
    domain::entities::medication_refill::MedicationRefill,
};

pub struct FakeMedicationRefillRepository {
    refills: Mutex<Vec<MedicationRefill>>,
    fail_on_save: bool,
}

impl Default for FakeMedicationRefillRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl FakeMedicationRefillRepository {
    pub fn new() -> Self {
        Self {
            refills: Mutex::new(Vec::new()),
            fail_on_save: false,
        }
    }

    pub fn with(refill: MedicationRefill) -> Self {
        Self {
            refills: Mutex::new(vec![refill]),
            fail_on_save: false,
        }
    }

    pub fn failing() -> Self {
        Self {
            refills: Mutex::new(Vec::new()),
            fail_on_save: true,
        }
    }

    pub fn saved_count(&self) -> usize {
        self.refills.lock().unwrap().len()
    }
}

impl MedicationRefillRepositoryPort for FakeMedicationRefillRepository {
    fn save(&self, refill: &MedicationRefill) -> Result<(), StorageError> {
        if self.fail_on_save {
            return Err(StorageError("forced failure".into()));
        }
        self.refills.lock().unwrap().push(refill.clone());
        Ok(())
    }

    fn find_by_id(&self, _id: &MedicationRefill) -> Result<Option<MedicationRefill>, StorageError> {
        Ok(None)
    }
}
