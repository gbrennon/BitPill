use bitpill::application::ports::fakes::FakeDoseRecordRepository;
use bitpill::application::dtos::requests::{CreateDoseRecordRequest, MarkMedicationTakenRequest};
use bitpill::application::ports::inbound::create_dose_record_port::CreateDoseRecordPort;
use bitpill::application::ports::inbound::mark_medication_taken_port::MarkMedicationTakenPort;
use bitpill::application::ports::outbound::dose_record_repository_port::DoseRecordRepository;
use bitpill::application::services::mark_medication_taken_service::MarkMedicationTakenService;
use bitpill::domain::value_objects::dose_record_id::DoseRecordId;
use bitpill::infrastructure::container::Container;
use chrono::NaiveDate;
use std::fs;
use std::sync::Arc;
use tempfile::tempdir;

#[test]
fn create_dose_record_persists_to_disk() {
    let dir = tempdir().unwrap();
    let dose_path = dir.path().join("dose_records.json");
    let container = Container::new_with_paths(
        dir.path().join("medications.json"),
        dose_path.clone(),
        dir.path().join("settings.json"),
    );

    let med_id = uuid::Uuid::nil().to_string();
    let scheduled_at = NaiveDate::from_ymd_opt(2020, 1, 1)
        .unwrap()
        .and_hms_opt(9, 0, 0)
        .unwrap();
    let req = CreateDoseRecordRequest::new(med_id.clone(), scheduled_at);
    let res = CreateDoseRecordPort::execute(&container.create_dose_record_service, req)
        .expect("create should succeed");
    assert!(!res.id.is_empty());

    let data = fs::read_to_string(&dose_path).unwrap();
    assert!(data.trim().starts_with("["));
}

/// Verifies the DoseRecord untaken invariant end-to-end:
/// `DoseRecord::new()` alone produces an untaken record; only after
/// `MarkMedicationTakenService::execute()` is the record stored as taken.
#[test]
fn mark_medication_taken_service_stores_record_as_taken() {
    let fake_repo = Arc::new(FakeDoseRecordRepository::new());
    let repo_trait: Arc<dyn DoseRecordRepository> = fake_repo.clone();
    let service = MarkMedicationTakenService::new(repo_trait);

    let med_id = "019535c4-0000-7000-8000-000000000001".to_string();
    let taken_at = NaiveDate::from_ymd_opt(2025, 6, 1)
        .unwrap()
        .and_hms_opt(8, 0, 0)
        .unwrap();
    let req = MarkMedicationTakenRequest::new(med_id.clone(), taken_at);

    let res = service.execute(req).expect("should succeed");

    // Read the saved record back and assert it is taken
    let record_id = DoseRecordId::from(uuid::Uuid::parse_str(&res.id).unwrap());
    let saved = fake_repo
        .find_by_id(&record_id)
        .expect("repo call should succeed")
        .expect("record should exist");

    assert!(
        saved.is_taken(),
        "record saved by MarkMedicationTakenService must be taken"
    );
    assert_eq!(saved.taken_at(), Some(taken_at));
}
