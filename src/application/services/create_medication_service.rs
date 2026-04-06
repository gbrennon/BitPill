use std::{convert::TryFrom, sync::Arc};

use crate::{
    application::{
        dtos::{requests::CreateMedicationRequest, responses::CreateMedicationResponse},
        errors::ApplicationError,
        ports::{
            create_medication_port::CreateMedicationPort,
            outbound::medication_repository_port::MedicationRepository,
        },
    },
    domain::entities::medication::Medication,
};

pub struct CreateMedicationService {
    repository: Arc<dyn MedicationRepository>,
}

impl CreateMedicationService {
    pub fn new(repository: Arc<dyn MedicationRepository>) -> Self {
        Self { repository }
    }
}

impl CreateMedicationPort for CreateMedicationService {
    fn execute(
        &self,
        request: CreateMedicationRequest,
    ) -> Result<CreateMedicationResponse, ApplicationError> {
        let medication = Medication::try_from(request)?;

        self.repository.save(&medication)?;

        Ok(CreateMedicationResponse {
            id: medication.id().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::fakes::FakeMedicationRepository;

    fn make_service(repo: Arc<FakeMedicationRepository>) -> CreateMedicationService {
        CreateMedicationService::new(repo)
    }

    fn valid_request() -> CreateMedicationRequest {
        CreateMedicationRequest::new("Aspirin", 500, vec![(8, 0)], "OnceDaily")
    }

    #[test]
    fn execute_with_valid_request_returns_non_empty_id() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let service = make_service(repo);

        let result = service.execute(valid_request());

        assert!(result.is_ok());
        assert!(!result.unwrap().id.is_empty());
    }

    #[test]
    fn execute_with_valid_request_saves_to_repository() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let service = make_service(repo.clone());

        service.execute(valid_request()).unwrap();

        assert_eq!(repo.saved_count(), 1);
    }

    #[test]
    fn execute_with_empty_name_returns_domain_error() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let service = make_service(repo);
        let request = CreateMedicationRequest::new("", 500, vec![(8, 0)], "OnceDaily");

        let result = service.execute(request);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, ApplicationError::MultipleDomainErrors { .. }));
    }

    #[test]
    fn execute_with_zero_dosage_returns_domain_error() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let service = make_service(repo);
        let request = CreateMedicationRequest::new("Aspirin", 0, vec![(8, 0)], "OnceDaily");

        let result = service.execute(request);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, ApplicationError::MultipleDomainErrors { .. }));
    }

    #[test]
    fn execute_when_repository_fails_returns_storage_error() {
        let repo = Arc::new(FakeMedicationRepository::failing());
        let service = make_service(repo);

        let result = service.execute(valid_request());

        assert!(matches!(result, Err(ApplicationError::Storage(_))));
    }
}
