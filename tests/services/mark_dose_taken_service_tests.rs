use bitpill::application::errors::ApplicationError;
use bitpill::application::errors::NotFoundError;
use bitpill::application::dtos::requests::MarkDoseTakenRequest;
use bitpill::application::ports::inbound::mark_dose_taken_port::MarkDoseTakenPort;
use bitpill::application::services::mark_dose_taken_service::MarkDoseTakenService;
use bitpill::domain::{
    entities::dose_record::DoseRecord,
    errors::DomainError,
    value_objects::{dose_record_id::DoseRecordId, medication_id::MedicationId},
};
use crate::fakes::FakeDoseRecordRepository;
use chrono::NaiveDate;
use std::sync::Arc;

fn make_datetime(h: u32, m: u32) -> chrono::NaiveDateTime {
    NaiveDate::from_ymd_opt(2025, 1, 1).unwrap().and_hms_opt(h, m, 0).unwrap()
}

fn make_request(record_id: &DoseRecordId, h: u32, m: u32) -> MarkDoseTakenRequest {
    MarkDoseTakenRequest::new(record_id.to_string(), make_datetime(h, m))
}

#[test]
fn execute_marks_existing_dose_record_as_taken() {
    let record = DoseRecord::new(MedicationId::generate(), make_datetime(8, 0));
    let record_id = record.id().clone();
    let service = MarkDoseTakenService::new(Arc::new(FakeDoseRecordRepository::with(record)));

    let result = service.execute(make_request(&record_id, 8, 5));

    assert!(result.is_ok());
    assert!(!result.unwrap().record_id.is_empty());
}

#[test]
fn execute_with_unknown_id_returns_not_found_error() {
    let service = MarkDoseTakenService::new(Arc::new(FakeDoseRecordRepository::new()));
    let unknown_id = DoseRecordId::generate();

    let result = service.execute(make_request(&unknown_id, 8, 5));

    assert!(matches!(result, Err(ApplicationError::NotFound(NotFoundError))));
}

#[test]
fn execute_with_invalid_record_id_returns_invalid_input_error() {
    let service = MarkDoseTakenService::new(Arc::new(FakeDoseRecordRepository::new()));
    let request = MarkDoseTakenRequest::new("not-a-uuid", make_datetime(8, 5));

    let result = service.execute(request);

    assert!(matches!(result, Err(ApplicationError::InvalidInput(_))));
}

#[test]
fn execute_on_already_taken_dose_returns_domain_error() {
    let mut record = DoseRecord::new(MedicationId::generate(), make_datetime(8, 0));
    record.mark_taken(make_datetime(8, 5)).unwrap();
    let record_id = record.id().clone();
    let service = MarkDoseTakenService::new(Arc::new(FakeDoseRecordRepository::with(record)));

    let result = service.execute(make_request(&record_id, 8, 10));

    assert!(matches!(result, Err(ApplicationError::Domain(DomainError::DoseAlreadyTaken))));
}

#[test]
fn execute_when_find_by_id_fails_returns_storage_error() {
    use bitpill::application::errors::ApplicationError;
    let service = MarkDoseTakenService::new(Arc::new(FakeDoseRecordRepository::failing_on_find_by_id()));
    let id = bitpill::domain::value_objects::dose_record_id::DoseRecordId::generate();

    let result = service.execute(make_request(&id, 8, 5));

    assert!(matches!(result, Err(ApplicationError::Storage(_))));
}

#[test]
fn execute_when_save_fails_after_mark_taken_returns_storage_error() {
    use bitpill::application::errors::ApplicationError;
    let record = DoseRecord::new(MedicationId::generate(), make_datetime(8, 0));
    let record_id = record.id().clone();
    let service = MarkDoseTakenService::new(Arc::new(FakeDoseRecordRepository::with_failing(record)));

    let result = service.execute(make_request(&record_id, 8, 5));

    assert!(matches!(result, Err(ApplicationError::Storage(_))));
}
