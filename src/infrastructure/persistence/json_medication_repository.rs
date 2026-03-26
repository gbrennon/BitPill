use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::Mutex;

use crate::application::errors::StorageError;
use crate::application::ports::medication_repository_port::MedicationRepository;
use crate::domain::{entities::medication::Medication, value_objects::medication_id::MedicationId};

pub struct JsonMedicationRepository {
    path: PathBuf,
    medications: Mutex<Vec<Medication>>,
}

impl Default for JsonMedicationRepository {
    fn default() -> Self {
        Self::with_default_path()
    }
}

impl JsonMedicationRepository {
    pub fn new(path: PathBuf) -> Self {
        let medications = Self::load_from_path(&path).unwrap_or_default();
        Self {
            path,
            medications: Mutex::new(medications),
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn with_default_path() -> Self {
        let path = std::env::var("BITPILL_MEDICATIONS_FILE")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("medications.json"));
        Self::new(path)
    }

    fn load_from_path(path: &PathBuf) -> Result<Vec<Medication>, StorageError> {
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(_) => return Ok(Vec::new()),
        };
        let mut data = String::new();
        file.read_to_string(&mut data)
            .map_err(|e| StorageError(format!("IO error: {e}")))?;
        serde_json::from_str(&data).map_err(|e| StorageError(format!("Deserialization error: {e}")))
    }

    fn save_to_file(&self) -> Result<(), StorageError> {
        let data = {
            let medications = self.medications.lock().unwrap();
            serde_json::to_string(&*medications)
                .map_err(|e| StorageError(format!("Serialization error: {e}")))?
        };
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

impl MedicationRepository for JsonMedicationRepository {
    fn save(&self, medication: &Medication) -> Result<(), StorageError> {
        let mut medications = self.medications.lock().unwrap();
        if let Some(existing) = medications.iter_mut().find(|m| m.id() == medication.id()) {
            *existing = medication.clone();
        } else {
            medications.push(medication.clone());
        }
        // Drop the mutex guard before performing file I/O to avoid deadlock
        drop(medications);
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
        // Drop the mutex guard before performing file I/O to avoid deadlock
        drop(medications);
        self.save_to_file()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::medication_repository_port::MedicationRepository;
    use crate::domain::entities::medication::Medication;
    use tempfile::tempdir;

    fn make_medication(name: &str) -> Medication {
        use crate::domain::value_objects::{
            dosage::Dosage, medication_frequency::DoseFrequency, medication_id::MedicationId,
            medication_name::MedicationName,
        };
        Medication::new(
            MedicationId::generate(),
            MedicationName::new(name).unwrap(),
            Dosage::new(100).unwrap(),
            vec![],
            DoseFrequency::OnceDaily,
        )
    }

    #[test]
    fn save_and_find_by_id_roundtrip() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("medications.json");
        let repo = JsonMedicationRepository::new(path);
        let med = make_medication("Aspirin");

        repo.save(&med).expect("save should succeed");
        let found = repo.find_by_id(med.id()).expect("find should succeed");

        assert!(found.is_some());
        assert_eq!(found.unwrap().name(), med.name());
    }

    #[test]
    fn find_all_returns_all_saved_medications() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("medications.json");
        let repo = JsonMedicationRepository::new(path);
        let med1 = make_medication("Aspirin");
        let med2 = make_medication("Ibuprofen");

        repo.save(&med1).expect("save should succeed");
        repo.save(&med2).expect("save should succeed");
        let all = repo.find_all().expect("find_all should succeed");

        assert_eq!(all.len(), 2);
    }

    #[test]
    fn delete_removes_medication_from_store() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("medications.json");
        let repo = JsonMedicationRepository::new(path);
        let med = make_medication("Aspirin");

        repo.save(&med).expect("save should succeed");
        repo.delete(med.id()).expect("delete should succeed");
        let found = repo.find_by_id(med.id()).expect("find should succeed");

        assert!(found.is_none());
    }

    #[test]
    fn save_updates_existing_medication_when_saved_twice() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("medications.json");
        let repo = JsonMedicationRepository::new(path);
        let med = make_medication("Aspirin");

        repo.save(&med).expect("first save should succeed");
        repo.save(&med)
            .expect("second save (update path) should succeed");
        let all = repo.find_all().expect("find_all should succeed");

        assert_eq!(all.len(), 1); // update, not insert
    }

    #[test]
    fn with_default_path_uses_env_var_when_set() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("meds_default_test.json");
        unsafe {
            std::env::set_var("BITPILL_MEDICATIONS_FILE", path.to_str().unwrap());
        }

        let repo1 = JsonMedicationRepository::with_default_path();
        let _repo2 = JsonMedicationRepository::default();
        let med = make_medication("Aspirin");
        repo1.save(&med).expect("save should succeed");

        unsafe {
            std::env::remove_var("BITPILL_MEDICATIONS_FILE");
        }
        assert!(path.exists());
    }

    #[test]
    fn data_persists_across_separate_repository_instances() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("medications.json");
        let med = make_medication("Aspirin");

        {
            let repo = JsonMedicationRepository::new(path.clone());
            repo.save(&med).expect("save should succeed");
        }

        let repo2 = JsonMedicationRepository::new(path);
        let found = repo2.find_by_id(med.id()).expect("find should succeed");

        assert!(found.is_some());
        assert_eq!(found.unwrap().name(), med.name());
    }
}
