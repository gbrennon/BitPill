use std::sync::Arc;

use bitpill::{
    application::{
        dtos::requests::MarkDoseTakenRequest, errors::ApplicationError,
        ports::inbound::mark_dose_taken_port::MarkDoseTakenPort,
        services::mark_dose_taken_service::MarkDoseTakenService,
    },
    domain::value_objects::dose_record_id::DoseRecordId,
};
use chrono::NaiveDate;

use crate::fakes::FakeDoseRecordRepository;

#[cfg(test)]
mod tests {
    use super::*;

    fn make_datetime(h: u32, m: u32) -> chrono::NaiveDateTime {
        NaiveDate::from_ymd_opt(2025, 1, 1)
            .unwrap()
            .and_hms_opt(h, m, 0)
            .unwrap()
    }

    fn make_med() -> bitpill::domain::entities::medication::Medication {
        bitpill::domain::entities::medication::Medication::new(
            bitpill::domain::value_objects::medication_id::MedicationId::from(uuid::Uuid::nil()),
            bitpill::domain::value_objects::medication_name::MedicationName::new("TestMed")
                .unwrap(),
            bitpill::domain::value_objects::dosage::Dosage::new(1).unwrap(),
            vec![bitpill::domain::value_objects::scheduled_time::ScheduledTime::new(8, 0).unwrap()],
            bitpill::domain::value_objects::medication_frequency::DoseFrequency::OnceDaily,
        )
        .unwrap()
    }

    #[test]
    fn execute_creates_and_saves_record() {
        let repo = Arc::new(FakeDoseRecordRepository::new());
        let med = make_med();
        let med_repo = Arc::new(crate::fakes::FakeMedicationRepository::with(vec![med]));
        let service = MarkDoseTakenService::new(repo.clone(), med_repo);
        let med_id = uuid::Uuid::nil().to_string();
        let req = MarkDoseTakenRequest::new(med_id.clone());

        let res = service.execute(req).expect("execute should succeed");

        assert!(!res.record_id.is_empty());
        assert_eq!(repo.saved_count(), 1);
    }

    #[test]
    fn execute_saves_record_as_taken() {
        let repo = Arc::new(FakeDoseRecordRepository::new());
        let med = make_med();
        let med_repo = Arc::new(crate::fakes::FakeMedicationRepository::with(vec![med]));
        let service = MarkDoseTakenService::new(repo.clone(), med_repo);
        let med_id = uuid::Uuid::nil().to_string();
        let req = MarkDoseTakenRequest::new(med_id.clone());

        let res = service.execute(req).expect("execute should succeed");

        let record_id = DoseRecordId::from(uuid::Uuid::parse_str(&res.record_id).unwrap());
        let saved = repo
            .find_by_id(&record_id)
            .unwrap()
            .expect("record should exist");
        assert!(saved.is_taken());
    }

    #[test]
    fn execute_with_invalid_medication_id_returns_error() {
        let repo = Arc::new(FakeDoseRecordRepository::new());
        let med_repo = Arc::new(crate::fakes::FakeMedicationRepository::new());
        let service = MarkDoseTakenService::new(repo, med_repo);
        let req = MarkDoseTakenRequest::new("not-a-uuid");

        let res = service.execute(req);

        assert!(matches!(res, Err(ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn execute_when_save_fails_returns_storage_error() {
        use bitpill::application::errors::ApplicationError;
        let repo = Arc::new(FakeDoseRecordRepository::failing());
        let med = make_med();
        let med_repo = Arc::new(crate::fakes::FakeMedicationRepository::with(vec![med]));
        let service = MarkDoseTakenService::new(repo, med_repo);
        let med_id = uuid::Uuid::nil().to_string();
        let req = MarkDoseTakenRequest::new(med_id);

        let res = service.execute(req);

        assert!(matches!(res, Err(ApplicationError::Storage(_))));
    }
}

fn make_med() -> bitpill::domain::entities::medication::Medication {
    bitpill::domain::entities::medication::Medication::new(
        bitpill::domain::value_objects::medication_id::MedicationId::from(uuid::Uuid::nil()),
        bitpill::domain::value_objects::medication_name::MedicationName::new("TestMed").unwrap(),
        bitpill::domain::value_objects::dosage::Dosage::new(1).unwrap(),
        vec![bitpill::domain::value_objects::scheduled_time::ScheduledTime::new(8, 0).unwrap()],
        bitpill::domain::value_objects::medication_frequency::DoseFrequency::OnceDaily,
    )
    .unwrap()
}

#[test]
fn execute_creates_and_saves_record() {
    let repo = Arc::new(FakeDoseRecordRepository::new());
    let med = make_med();
    let med_repo = Arc::new(crate::fakes::FakeMedicationRepository::with(vec![med]));
    let service = MarkDoseTakenService::new(repo.clone(), med_repo);
    let med_id = uuid::Uuid::nil().to_string();
    let req = MarkDoseTakenRequest::new(med_id.clone());

    let res = service.execute(req).expect("execute should succeed");

    assert!(!res.record_id.is_empty());
    assert_eq!(repo.saved_count(), 1);
}

#[test]
fn execute_saves_record_as_taken() {
    let repo = Arc::new(FakeDoseRecordRepository::new());
    let med = make_med();
    let med_repo = Arc::new(crate::fakes::FakeMedicationRepository::with(vec![med]));
    let service = MarkDoseTakenService::new(repo.clone(), med_repo);
    let med_id = uuid::Uuid::nil().to_string();
    let req = MarkDoseTakenRequest::new(med_id.clone());

    let res = service.execute(req).expect("execute should succeed");

    let record_id = DoseRecordId::from(uuid::Uuid::parse_str(&res.record_id).unwrap());
    let saved = repo
        .find_by_id(&record_id)
        .unwrap()
        .expect("record should exist");
    assert!(saved.is_taken());
}

#[test]
fn execute_with_invalid_medication_id_returns_error() {
    let repo = Arc::new(FakeDoseRecordRepository::new());
    let med_repo = Arc::new(crate::fakes::FakeMedicationRepository::new());
    let service = MarkDoseTakenService::new(repo, med_repo);
    let req = MarkDoseTakenRequest::new("not-a-uuid");

    let res = service.execute(req);

    assert!(matches!(res, Err(ApplicationError::InvalidInput(_))));
}

#[test]
fn execute_when_save_fails_returns_storage_error() {
    use bitpill::application::errors::ApplicationError;
    let repo = Arc::new(FakeDoseRecordRepository::failing());
    let med = make_med();
    let med_repo = Arc::new(crate::fakes::FakeMedicationRepository::with(vec![med]));
    let service = MarkDoseTakenService::new(repo, med_repo);
    let med_id = uuid::Uuid::nil().to_string();
    let req = MarkDoseTakenRequest::new(med_id);

    let res = service.execute(req);

    assert!(matches!(res, Err(ApplicationError::Storage(_))));
}
