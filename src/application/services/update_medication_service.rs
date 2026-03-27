use std::convert::TryFrom;
use std::sync::Arc;

use uuid::Uuid;

use crate::application::{
    dtos::{requests::UpdateMedicationRequest, responses::UpdateMedicationResponse},
    errors::ApplicationError,
    ports::{
        inbound::update_medication_port::UpdateMedicationPort,
        outbound::medication_repository_port::MedicationRepository,
    },
};
use crate::domain::entities::medication::Medication;
use crate::domain::value_objects::medication_id::MedicationId;

pub struct UpdateMedicationService {
    repository: Arc<dyn MedicationRepository>,
}

impl UpdateMedicationService {
    pub fn new(repository: Arc<dyn MedicationRepository>) -> Self {
        Self { repository }
    }
}

impl UpdateMedicationPort for UpdateMedicationService {
    fn execute(
        &self,
        request: UpdateMedicationRequest,
    ) -> Result<UpdateMedicationResponse, ApplicationError> {
        let id = Uuid::parse_str(&request.id)
            .map_err(|_| ApplicationError::InvalidInput("invalid id".into()))?;
        let id_str = request.id.clone();
        let med_id = MedicationId::from(id);

        let medication = Medication::try_from((request, med_id))?;

        self.repository.save(&medication)?;

        Ok(UpdateMedicationResponse { id: id_str })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::fakes::FakeMedicationRepository;

    fn make_service(repo: Arc<FakeMedicationRepository>) -> UpdateMedicationService {
        UpdateMedicationService::new(repo)
    }

    fn valid_id() -> String {
        Uuid::now_v7().to_string()
    }

    fn valid_request(id: &str) -> UpdateMedicationRequest {
        UpdateMedicationRequest::new(id, "Ibuprofen", 200, vec![(8, 0)], "OnceDaily")
    }

    #[test]
    fn execute_with_valid_request_returns_same_id() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let service = make_service(repo);
        let id = valid_id();

        let result = service.execute(valid_request(&id));

        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, id);
    }

    #[test]
    fn execute_with_valid_request_saves_to_repository() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let service = make_service(repo.clone());

        service.execute(valid_request(&valid_id())).unwrap();

        assert_eq!(repo.saved_count(), 1);
    }

    #[test]
    fn execute_with_invalid_uuid_returns_invalid_input_error() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let service = make_service(repo);
        let request =
            UpdateMedicationRequest::new("not-a-uuid", "Ibuprofen", 200, vec![(8, 0)], "OnceDaily");

        let result = service.execute(request);

        assert!(matches!(result, Err(ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn execute_with_empty_name_returns_domain_error() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let service = make_service(repo);
        let request = UpdateMedicationRequest::new(valid_id(), "", 200, vec![(8, 0)], "OnceDaily");

        let result = service.execute(request);

        assert!(matches!(result, Err(ApplicationError::Domain(_))));
    }

    #[test]
    fn execute_with_zero_dosage_returns_domain_error() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let service = make_service(repo);
        let request =
            UpdateMedicationRequest::new(valid_id(), "Ibuprofen", 0, vec![(8, 0)], "OnceDaily");

        let result = service.execute(request);

        assert!(matches!(result, Err(ApplicationError::Domain(_))));
    }

    #[test]
    fn execute_when_repository_fails_returns_storage_error() {
        let repo = Arc::new(FakeMedicationRepository::failing());
        let service = make_service(repo);

        let result = service.execute(valid_request(&valid_id()));

        assert!(matches!(result, Err(ApplicationError::Storage(_))));
    }
}
