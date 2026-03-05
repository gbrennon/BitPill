use bitpill::application::errors::ApplicationError;
use bitpill::application::dtos::requests::MarkMedicationTakenRequest;
use bitpill::application::ports::inbound::mark_medication_taken_port::MarkMedicationTakenPort;
use bitpill::application::services::mark_medication_taken_service::MarkMedicationTakenService;
use bitpill::domain::value_objects::dose_record_id::DoseRecordId;
use crate::fakes::FakeDoseRecordRepository;
use chrono::NaiveDate;
use std::sync::Arc;

fn make_datetime(h: u32, m: u32) -> chrono::NaiveDateTime {
    NaiveDate::from_ymd_opt(2025, 1, 1).unwrap().and_hms_opt(h, m, 0).unwrap()
}

#[test]
fn execute_creates_and_saves_record() {
    let repo = Arc::new(FakeDoseRecordRepository::new());
    let service = MarkMedicationTakenService::new(repo.clone());
    let med_id = uuid::Uuid::nil().to_string();
    let req = MarkMedicationTakenRequest::new(med_id.clone(), make_datetime(9, 0));

    let res = service.execute(req).expect("execute should succeed");

    assert!(!res.id.is_empty());
    assert_eq!(repo.saved_count(), 1);
}

#[test]
fn execute_saves_record_as_taken() {
    let repo = Arc::new(FakeDoseRecordRepository::new());
    let service = MarkMedicationTakenService::new(repo.clone());
    let med_id = uuid::Uuid::nil().to_string();
    let req = MarkMedicationTakenRequest::new(med_id.clone(), make_datetime(9, 0));

    let res = service.execute(req).expect("execute should succeed");

    let record_id = DoseRecordId::from(uuid::Uuid::parse_str(&res.id).unwrap());
    let saved = repo.find_by_id(&record_id).unwrap().expect("record should exist");
    assert!(saved.is_taken());
}

#[test]
fn execute_with_invalid_medication_id_returns_error() {
    let repo = Arc::new(FakeDoseRecordRepository::new());
    let service = MarkMedicationTakenService::new(repo);
    let req = MarkMedicationTakenRequest::new("not-a-uuid", make_datetime(9, 0));

    let res = service.execute(req);

    assert!(matches!(res, Err(ApplicationError::InvalidInput(_))));
}

#[test]
fn execute_when_save_fails_returns_storage_error() {
    use bitpill::application::errors::ApplicationError;
    let repo = Arc::new(FakeDoseRecordRepository::failing());
    let service = MarkMedicationTakenService::new(repo);
    let med_id = uuid::Uuid::nil().to_string();
    let req = MarkMedicationTakenRequest::new(med_id, make_datetime(9, 0));

    let res = service.execute(req);

    assert!(matches!(res, Err(ApplicationError::Storage(_))));
}
