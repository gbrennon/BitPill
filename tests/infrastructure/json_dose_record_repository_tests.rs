use bitpill::{
    application::ports::outbound::dose_record_repository_port::DoseRecordRepository,
    domain::{
        entities::dose_record::DoseRecord,
        value_objects::{dose_record_id::DoseRecordId, medication_id::MedicationId},
    },
    infrastructure::persistence::json_dose_record_repository::JsonDoseRecordRepository,
};
use chrono::NaiveDate;
use tempfile::tempdir;

fn make_record(med_id: &str, hour: u32, min: u32) -> DoseRecord {
    let scheduled_at = NaiveDate::from_ymd_opt(2024, 1, 1)
        .unwrap()
        .and_hms_opt(hour, min, 0)
        .unwrap();
    DoseRecord::new(
        MedicationId::from(uuid::Uuid::parse_str(med_id).unwrap()),
        scheduled_at,
    )
}

#[test]
fn new_creates_repo() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("dose_records.json");
    let repo = JsonDoseRecordRepository::new(path.clone());
    assert_eq!(repo.path().as_os_str(), path.as_os_str());
}

#[test]
fn path_returns_stored_path() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("test_doses.json");
    let repo = JsonDoseRecordRepository::new(path.clone());
    assert_eq!(repo.path(), &path);
}

#[test]
fn default_uses_with_default_path() {
    let _repo = JsonDoseRecordRepository::default();
}

#[test]
fn save_persists_record() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("doses.json");
    let repo = JsonDoseRecordRepository::new(path);
    let med_id = "a1a2a3a4-b5b6-4747-b8b8-c9c0d1d2d3d4";
    let record = make_record(med_id, 9, 0);
    repo.save(&record).expect("save should succeed");
    let found = repo.find_by_id(record.id()).expect("find_by_id ok");
    assert!(found.is_some());
    assert_eq!(found.unwrap().medication_id(), record.medication_id());
}

#[test]
fn save_updates_existing_record() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("doses.json");
    let repo = JsonDoseRecordRepository::new(path);
    let med_id = "a1a2a3a4-b5b6-4747-b8b8-c9c0d1d2d3d4";
    let mut record = make_record(med_id, 9, 0);
    repo.save(&record).expect("save ok");
    // Mark as taken and save again
    let now = NaiveDate::from_ymd_opt(2024, 1, 2)
        .unwrap()
        .and_hms_opt(9, 0, 0)
        .unwrap();
    record.mark_taken(now).expect("mark_taken ok");
    repo.save(&record).expect("save ok");
    let found = repo.find_by_id(record.id()).expect("find_by_id ok");
    assert!(found.unwrap().is_taken());
}

#[test]
fn find_by_id_returns_none_for_unknown() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("doses.json");
    let repo = JsonDoseRecordRepository::new(path);
    let med_id = "b1b2b3b4-c5c6-4747-d8d8-e9e0f1f2f3f4";
    let record = make_record(med_id, 9, 0);
    repo.save(&record).unwrap();
    let unknown = DoseRecordId::generate();
    let found = repo.find_by_id(&unknown).expect("find_by_id ok");
    assert!(found.is_none());
}

#[test]
fn find_all_by_medication_filters_correctly() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("doses.json");
    let repo = JsonDoseRecordRepository::new(path);
    let ma = "a1a2a3a4-b5b6-4747-b8b8-c9c0d1d2d3d4";
    let mb = "b1b2b3b4-c5c6-4747-d8d8-e9e0f1f2f3f4";
    repo.save(&make_record(ma, 9, 0)).unwrap();
    repo.save(&make_record(ma, 10, 0)).unwrap();
    repo.save(&make_record(mb, 11, 0)).unwrap();
    let results = repo
        .find_all_by_medication(&MedicationId::from(uuid::Uuid::parse_str(ma).unwrap()))
        .expect("find_all_by_medication ok");
    assert_eq!(results.len(), 2);
}

#[test]
fn find_all_by_medication_empty_for_no_match() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("doses.json");
    let repo = JsonDoseRecordRepository::new(path);
    let ma = "a1a2a3a4-b5b6-4747-b8b8-c9c0d1d2d3d4";
    repo.save(&make_record(ma, 9, 0)).unwrap();
    let unmatched = MedicationId::generate();
    let results = repo
        .find_all_by_medication(&unmatched)
        .expect("find_all_by_medication ok");
    assert!(results.is_empty());
}

#[test]
fn delete_removes_record() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("doses.json");
    let repo = JsonDoseRecordRepository::new(path);
    let med_id = "a1a2a3a4-b5b6-4747-b8b8-c9c0d1d2d3d4";
    let record = make_record(med_id, 9, 0);
    let id = record.id().clone();
    repo.save(&record).unwrap();
    repo.delete(&id).expect("delete ok");
    let found = repo.find_by_id(&id).expect("find_by_id ok");
    assert!(found.is_none());
}

#[test]
fn delete_nonexistent_no_error() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("doses.json");
    let repo = JsonDoseRecordRepository::new(path);
    repo.delete(&DoseRecordId::generate()).expect("delete ok");
}

#[test]
fn load_from_existing_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("doses.json");
    let repo = JsonDoseRecordRepository::new(path.clone());
    let med_id = "a1a2a3a4-b5b6-4747-b8b8-c9c0d1d2d3d4";
    let record = make_record(med_id, 9, 0);
    repo.save(&record).unwrap();
    let repo2 = JsonDoseRecordRepository::new(path);
    let found = repo2.find_by_id(record.id()).expect("find_by_id ok");
    assert!(found.is_some());
}

#[test]
fn save_appends_multiple() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("doses.json");
    let repo = JsonDoseRecordRepository::new(path);
    let med_id = "a1a2a3a4-b5b6-4747-b8b8-c9c0d1d2d3d4";
    repo.save(&make_record(med_id, 9, 0)).unwrap();
    repo.save(&make_record(med_id, 12, 0)).unwrap();
    repo.save(&make_record(med_id, 18, 0)).unwrap();
    let results = repo
        .find_all_by_medication(&MedicationId::from(uuid::Uuid::parse_str(med_id).unwrap()))
        .expect("find_all ok");
    assert_eq!(results.len(), 3);
}

#[test]
fn with_default_path_constructs() {
    let repo = JsonDoseRecordRepository::with_default_path();
    let _p = repo.path();
}

#[test]
fn corrupt_file_loads_empty() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("doses.json");
    std::fs::write(&path, "not valid json {{{{{").unwrap();
    let repo = JsonDoseRecordRepository::new(path);
    let results = repo
        .find_all_by_medication(&MedicationId::generate())
        .expect("find_all ok");
    assert!(results.is_empty());
}
