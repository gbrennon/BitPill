use bitpill::application::dtos::requests::CreateDoseRecordRequest;
use bitpill::application::ports::inbound::create_dose_record_port::CreateDoseRecordPort;
use bitpill::application::services::create_dose_record_service::CreateDoseRecordsService;
use crate::fakes::FakeDoseRecordRepository;
use chrono::NaiveDate;
use std::sync::Arc;

#[test]
fn execute_with_invalid_medication_id_returns_invalid_input() {
    let repo = Arc::new(FakeDoseRecordRepository::new());
    let service = CreateDoseRecordsService::new(repo);
    let scheduled_at = NaiveDate::from_ymd_opt(2020, 1, 1)
        .unwrap()
        .and_hms_opt(9, 0, 0)
        .unwrap();
    let req = CreateDoseRecordRequest::new("not-a-valid-uuid".to_string(), scheduled_at);

    let result = service.execute(req);

    assert!(matches!(result, Err(bitpill::application::errors::ApplicationError::InvalidInput(_))));
}

#[test]
fn create_dose_record_saves_to_repository() {
    let repo = Arc::new(FakeDoseRecordRepository::new());
    let service = CreateDoseRecordsService::new(repo.clone());
    let med_id = uuid::Uuid::nil().to_string();
    let scheduled_at = NaiveDate::from_ymd_opt(2020, 1, 1)
        .unwrap()
        .and_hms_opt(9, 0, 0)
        .unwrap();
    let req = CreateDoseRecordRequest::new(med_id.clone(), scheduled_at);

    let res = service.execute(req).expect("execute should succeed");

    assert!(!res.id.is_empty());
    assert_eq!(repo.saved_count(), 1);
}

#[test]
fn execute_when_repository_fails_returns_storage_error() {
    use bitpill::application::errors::ApplicationError;
    let repo = Arc::new(FakeDoseRecordRepository::failing());
    let service = CreateDoseRecordsService::new(repo);
    let scheduled_at = NaiveDate::from_ymd_opt(2020, 1, 1)
        .unwrap()
        .and_hms_opt(9, 0, 0)
        .unwrap();
    let req = CreateDoseRecordRequest::new(uuid::Uuid::nil().to_string(), scheduled_at);

    let result = service.execute(req);

    assert!(matches!(result, Err(ApplicationError::Storage(_))));
}
