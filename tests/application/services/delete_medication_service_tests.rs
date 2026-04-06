use crate::fakes::FakeMedicationRepository;
use bitpill::application::dtos::requests::DeleteMedicationRequest;
use bitpill::application::errors::ApplicationError;
use bitpill::application::ports::inbound::delete_medication_port::DeleteMedicationPort;
use bitpill::application::ports::outbound::medication_repository_port::MedicationRepository;
use bitpill::application::services::delete_medication_service::DeleteMedicationService;
use bitpill::domain::{
    entities::medication::Medication,
    value_objects::{
        dosage::Dosage, medication_frequency::DoseFrequency, medication_id::MedicationId,
        medication_name::MedicationName, scheduled_time::ScheduledTime,
    },
};
use std::sync::Arc;

#[test]
fn delete_medication_success_removes_from_repo() {
    let med = Medication::new(
        MedicationId::generate(),
        MedicationName::new("DelMed").unwrap(),
        Dosage::new(10).unwrap(),
        vec![ScheduledTime::new(8, 0).unwrap()],
        DoseFrequency::OnceDaily,
    )
    .unwrap();
    let repo = Arc::new(FakeMedicationRepository::with(vec![med.clone()]));
    let svc = DeleteMedicationService::new(repo.clone());
    let req = DeleteMedicationRequest {
        id: med.id().to_string(),
    };

    svc.execute(req).expect("should delete");

    let found = repo.find_by_id(med.id()).unwrap();
    assert!(found.is_none());
}

#[test]
fn delete_medication_invalid_id_returns_error() {
    let repo = Arc::new(FakeMedicationRepository::new());
    let svc = DeleteMedicationService::new(repo);
    let req = DeleteMedicationRequest {
        id: "not-a-uuid".into(),
    };

    let res = svc.execute(req);

    assert!(matches!(res, Err(ApplicationError::InvalidInput(_))));
}

#[test]
fn delete_medication_when_repository_delete_fails_returns_storage_error() {
    use bitpill::domain::value_objects::medication_id::MedicationId;
    let repo = Arc::new(FakeMedicationRepository::failing_on_delete());
    let svc = DeleteMedicationService::new(repo);
    let req = DeleteMedicationRequest {
        id: MedicationId::generate().to_string(),
    };

    let res = svc.execute(req);

    assert!(matches!(res, Err(ApplicationError::Storage(_))));
}
