use std::sync::Mutex;

use crate::{
    application::{errors::StorageError, ports::outbound::MedicationBoxRepositoryPort},
    domain::{
        entities::medication_box::MedicationBox, value_objects::medication_box_id::MedicationBoxId,
    },
};

pub struct FakeMedicationBoxRepository {
    boxes: Mutex<Vec<MedicationBox>>,
    fail_on_save: bool,
    fail_on_delete: bool,
}

impl Default for FakeMedicationBoxRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl FakeMedicationBoxRepository {
    pub fn new() -> Self {
        Self {
            boxes: Mutex::new(Vec::new()),
            fail_on_save: false,
            fail_on_delete: false,
        }
    }

    pub fn with(r#box: MedicationBox) -> Self {
        Self {
            boxes: Mutex::new(vec![r#box]),
            fail_on_save: false,
            fail_on_delete: false,
        }
    }

    pub fn failing() -> Self {
        Self {
            boxes: Mutex::new(Vec::new()),
            fail_on_save: true,
            fail_on_delete: true,
        }
    }

    pub fn saved_count(&self) -> usize {
        self.boxes.lock().unwrap().len()
    }
}

impl MedicationBoxRepositoryPort for FakeMedicationBoxRepository {
    fn save(&self, medication_box: &MedicationBox) -> Result<(), StorageError> {
        if self.fail_on_save {
            return Err(StorageError("forced failure".into()));
        }
        let mut boxes = self.boxes.lock().unwrap();
        if let Some(existing) = boxes.iter_mut().find(|b| b.id() == medication_box.id()) {
            *existing = medication_box.clone();
        } else {
            boxes.push(medication_box.clone());
        }
        Ok(())
    }

    fn find_by_id(&self, id: &MedicationBoxId) -> Result<Option<MedicationBox>, StorageError> {
        Ok(self
            .boxes
            .lock()
            .unwrap()
            .iter()
            .find(|b| b.id() == id)
            .cloned())
    }

    fn find_all(&self) -> Result<Vec<MedicationBox>, StorageError> {
        Ok(self.boxes.lock().unwrap().clone())
    }

    fn delete(&self, id: &MedicationBoxId) -> Result<(), StorageError> {
        if self.fail_on_delete {
            return Err(StorageError("forced failure".into()));
        }
        let mut boxes = self.boxes.lock().unwrap();
        boxes.retain(|b| b.id() != id);
        Ok(())
    }
}
