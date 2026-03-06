use crate::common::fixtures;
use crate::fakes::FakeMedicationRepository;
use bitpill::application::dtos::requests::ListAllMedicationsRequest;
use bitpill::application::ports::inbound::list_all_medications_port::ListAllMedicationsPort;
use bitpill::application::services::list_all_medications_service::ListAllMedicationsService;
use std::sync::Arc;

#[test]
fn execute_with_empty_repository_returns_empty_list() {
    let repo = Arc::new(FakeMedicationRepository::new());
    let service = ListAllMedicationsService::new(repo);

    let result = service.execute(ListAllMedicationsRequest);

    assert!(result.is_ok());
    assert!(result.unwrap().medications.is_empty());
}

#[test]
fn execute_returns_all_medications_as_dtos() {
    let repo = Arc::new(FakeMedicationRepository::with(vec![
        fixtures::medication("Aspirin", 500),
        fixtures::medication("Ibuprofen", 200),
    ]));
    let service = ListAllMedicationsService::new(repo);

    let result = service.execute(ListAllMedicationsRequest).unwrap();

    assert_eq!(result.medications.len(), 2);
}

#[test]
fn execute_maps_medication_fields_correctly() {
    let med = fixtures::medication("Paracetamol", 1000);
    let repo = Arc::new(FakeMedicationRepository::with(vec![med]));
    let service = ListAllMedicationsService::new(repo);

    let result = service.execute(ListAllMedicationsRequest).unwrap();
    let dto = &result.medications[0];

    assert_eq!(dto.name, "Paracetamol");
    assert_eq!(dto.amount_mg, 1000);
    assert_eq!(dto.scheduled_time, vec![(8, 0)]);
}

#[test]
fn execute_when_repository_fails_returns_storage_error() {
    use bitpill::application::errors::ApplicationError;
    let repo = Arc::new(crate::fakes::FakeMedicationRepository::failing_on_find_all());
    let service = ListAllMedicationsService::new(repo);

    let result = service.execute(ListAllMedicationsRequest);

    assert!(matches!(result, Err(ApplicationError::Storage(_))));
}
