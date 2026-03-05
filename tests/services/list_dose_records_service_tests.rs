use bitpill::application::errors::ApplicationError;
use bitpill::application::dtos::requests::ListDoseRecordsRequest;
use bitpill::application::ports::inbound::list_dose_records_port::ListDoseRecordsPort;
use bitpill::application::services::list_dose_records_service::ListDoseRecordsService;
use bitpill::domain::{
    entities::dose_record::DoseRecord,
    value_objects::medication_id::MedicationId,
};
use crate::fakes::FakeDoseRecordRepository;
use chrono::NaiveDate;
use std::sync::Arc;

#[test]
fn list_dose_records_returns_records_for_medication() {
    let med_id = MedicationId::generate();
    let record = DoseRecord::new(
        med_id.clone(),
        NaiveDate::from_ymd_opt(2025, 1, 1).unwrap().and_hms_opt(9, 0, 0).unwrap(),
    );
    let repo = Arc::new(FakeDoseRecordRepository::with(record.clone()));
    let service = ListDoseRecordsService::new(repo);

    let req = ListDoseRecordsRequest { medication_id: med_id.to_string() };
    let res = service.execute(req).expect("should list records");

    assert_eq!(res.records.len(), 1);
    assert_eq!(res.records[0].id, record.id().to_string());
}

#[test]
fn list_dose_records_invalid_medication_id_returns_invalid_input() {
    let repo = Arc::new(FakeDoseRecordRepository::new());
    let service = ListDoseRecordsService::new(repo);

    let req = ListDoseRecordsRequest { medication_id: "not-a-uuid".into() };
    let res = service.execute(req);

    assert!(matches!(res, Err(ApplicationError::InvalidInput(_))));
}

#[test]
fn list_dose_records_when_repository_fails_returns_storage_error() {
    use bitpill::application::errors::ApplicationError;
    let repo = Arc::new(FakeDoseRecordRepository::failing_on_find_all_by_medication());
    let service = ListDoseRecordsService::new(repo);
    let med_id = bitpill::domain::value_objects::medication_id::MedicationId::generate().to_string();

    let res = service.execute(ListDoseRecordsRequest { medication_id: med_id });

    assert!(matches!(res, Err(ApplicationError::Storage(_))));
}
