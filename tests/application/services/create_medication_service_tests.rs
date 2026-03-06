use crate::fakes::FakeMedicationRepository;
use bitpill::application::dtos::requests::CreateMedicationRequest;
use bitpill::application::errors::ApplicationError;
use bitpill::application::ports::inbound::create_medication_port::CreateMedicationPort;
use bitpill::application::services::create_medication_service::CreateMedicationService;
use bitpill::domain::errors::DomainError;
use std::sync::Arc;

fn make_request(
    name: &str,
    amount_mg: u32,
    scheduled_time: Vec<(u32, u32)>,
) -> CreateMedicationRequest {
    CreateMedicationRequest::new(name, amount_mg, scheduled_time, "OnceDaily")
}

#[test]
fn execute_with_valid_inputs_returns_response() {
    let service = CreateMedicationService::new(Arc::new(FakeMedicationRepository::new()));

    let result = service.execute(make_request("Levetiracetam", 500, vec![(8, 0), (20, 0)]));

    assert!(result.is_ok());
    assert!(!result.unwrap().id.is_empty());
}

#[test]
fn execute_saves_medication_to_repository() {
    let repo = Arc::new(FakeMedicationRepository::new());
    let service = CreateMedicationService::new(repo.clone());

    service
        .execute(make_request("Ibuprofen", 200, vec![(8, 0)]))
        .unwrap();

    assert_eq!(repo.saved_count(), 1);
}

#[test]
fn execute_with_empty_name_returns_domain_error() {
    let service = CreateMedicationService::new(Arc::new(FakeMedicationRepository::new()));

    let result = service.execute(make_request("", 500, vec![(8, 0)]));

    assert!(matches!(
        result,
        Err(ApplicationError::Domain(DomainError::EmptyMedicationName))
    ));
}

#[test]
fn execute_with_zero_dosage_returns_domain_error() {
    let service = CreateMedicationService::new(Arc::new(FakeMedicationRepository::new()));

    let result = service.execute(make_request("Levetiracetam", 0, vec![(8, 0)]));

    assert!(matches!(
        result,
        Err(ApplicationError::Domain(DomainError::InvalidDosage))
    ));
}

#[test]
fn execute_with_invalid_scheduled_time_returns_domain_error() {
    let service = CreateMedicationService::new(Arc::new(FakeMedicationRepository::new()));

    let result = service.execute(make_request("Levetiracetam", 500, vec![(25, 0)]));

    assert!(matches!(
        result,
        Err(ApplicationError::Domain(DomainError::InvalidScheduledTime))
    ));
}

#[test]
fn execute_when_repository_fails_returns_storage_error() {
    let service = CreateMedicationService::new(Arc::new(FakeMedicationRepository::failing()));

    let result = service.execute(make_request("Levetiracetam", 500, vec![(8, 0)]));

    assert!(matches!(result, Err(ApplicationError::Storage(_))));
}

#[test]
fn execute_with_twice_daily_frequency() {
    let service = CreateMedicationService::new(Arc::new(FakeMedicationRepository::new()));
    let req = CreateMedicationRequest::new("Med", 100, vec![(8, 0), (20, 0)], "TwiceDaily");

    let result = service.execute(req);

    assert!(result.is_ok());
}

#[test]
fn execute_with_thrice_daily_frequency() {
    let service = CreateMedicationService::new(Arc::new(FakeMedicationRepository::new()));
    let req =
        CreateMedicationRequest::new("Med", 100, vec![(8, 0), (14, 0), (20, 0)], "ThriceDaily");

    let result = service.execute(req);

    assert!(result.is_ok());
}

#[test]
fn execute_with_custom_frequency() {
    let service = CreateMedicationService::new(Arc::new(FakeMedicationRepository::new()));
    let req = CreateMedicationRequest::new("Med", 100, vec![(9, 0), (21, 0)], "Custom");

    let result = service.execute(req);

    assert!(result.is_ok());
}

#[test]
fn execute_with_unknown_frequency_defaults_to_once_daily() {
    let service = CreateMedicationService::new(Arc::new(FakeMedicationRepository::new()));
    let req = CreateMedicationRequest::new("Med", 100, vec![(8, 0)], "UnknownFrequency");

    let result = service.execute(req);

    assert!(result.is_ok());
}
