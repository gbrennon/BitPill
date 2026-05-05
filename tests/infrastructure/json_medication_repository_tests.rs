use bitpill::{
    application::ports::outbound::medication_repository_port::MedicationRepository,
    domain::{
        entities::medication::Medication,
        value_objects::{
            dosage::Dosage, medication_frequency::DoseFrequency, medication_id::MedicationId,
            medication_name::MedicationName, scheduled_time::ScheduledTime,
        },
    },
    infrastructure::persistence::json_medication_repository::JsonMedicationRepository,
};
use tempfile::tempdir;

fn make_med(name: &str, dosage: u32) -> Medication {
    Medication::new(
        MedicationId::generate(),
        MedicationName::new(name.to_string()).unwrap(),
        Dosage::new(dosage).unwrap(),
        vec![ScheduledTime::new(9, 0).unwrap()],
        DoseFrequency::OnceDaily,
    )
    .unwrap()
}

#[test]
fn new_works_with_fresh_path() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("meds.json");
    let repo = JsonMedicationRepository::new(path.clone());
    assert_eq!(repo.path(), &path);
}

#[test]
fn default_constructs() {
    let _repo = JsonMedicationRepository::default();
}

#[test]
fn save_and_find_by_id_roundtrip() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("meds.json");
    let repo = JsonMedicationRepository::new(path);
    let med = make_med("Aspirin", 100);
    repo.save(&med).expect("save ok");
    let found = repo.find_by_id(med.id()).expect("find ok");
    assert!(found.is_some());
}

#[test]
fn save_updates_existing() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("meds.json");
    let repo = JsonMedicationRepository::new(path);
    let med = make_med("Ibuprofen", 200);
    repo.save(&med).unwrap();
    repo.save(&med).unwrap();
    let found = repo.find_by_id(med.id()).unwrap();
    assert!(found.is_some());
}

#[test]
fn find_by_id_returns_none_for_missing() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("meds.json");
    let repo = JsonMedicationRepository::new(path);
    let med = make_med("Paracetamol", 500);
    repo.save(&med).unwrap();
    let found = repo.find_by_id(&MedicationId::generate()).unwrap();
    assert!(found.is_none());
}

#[test]
fn find_all_returns_all() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("meds.json");
    let repo = JsonMedicationRepository::new(path);
    repo.save(&make_med("A", 10)).unwrap();
    repo.save(&make_med("B", 20)).unwrap();
    repo.save(&make_med("C", 30)).unwrap();
    assert_eq!(repo.find_all().unwrap().len(), 3);
}

#[test]
fn find_all_empty_when_no_data() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("meds.json");
    let repo = JsonMedicationRepository::new(path);
    assert!(repo.find_all().unwrap().is_empty());
}

#[test]
fn delete_removes_medication() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("meds.json");
    let repo = JsonMedicationRepository::new(path);
    let med = make_med("DeleteMe", 50);
    let id = med.id().clone();
    repo.save(&med).unwrap();
    repo.delete(&id).unwrap();
    assert!(repo.find_by_id(&id).unwrap().is_none());
}

#[test]
fn delete_nonexistent_does_not_error() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("meds.json");
    let repo = JsonMedicationRepository::new(path);
    repo.delete(&MedicationId::generate()).unwrap();
}

#[test]
fn load_from_existing_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("meds.json");
    let repo = JsonMedicationRepository::new(path.clone());
    let med = make_med("Persisted", 75);
    repo.save(&med).unwrap();
    let repo2 = JsonMedicationRepository::new(path);
    let found = repo2.find_by_id(med.id()).unwrap();
    assert!(found.is_some());
}

#[test]
fn with_default_path_works() {
    let repo = JsonMedicationRepository::with_default_path();
    let _p = repo.path();
}

#[test]
fn corrupt_file_loads_empty() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("meds.json");
    std::fs::write(&path, "{{{ bad json").unwrap();
    let repo = JsonMedicationRepository::new(path);
    assert!(repo.find_all().unwrap().is_empty());
}
