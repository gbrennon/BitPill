use std::sync::Arc;
use uuid::Uuid;

use crate::application::dtos::requests::DeleteMedicationRequest;
use crate::application::dtos::responses::DeleteMedicationResponse;
use crate::application::errors::ApplicationError;
use crate::application::ports::inbound::delete_medication_port::DeleteMedicationPort;
use crate::application::ports::outbound::medication_repository_port::MedicationRepository;
use crate::domain::value_objects::medication_id::MedicationId;

pub struct DeleteMedicationService {
    repository: Arc<dyn MedicationRepository>,
}

impl DeleteMedicationService {
    pub fn new(repository: Arc<dyn MedicationRepository>) -> Self {
        Self { repository }
    }
}

impl DeleteMedicationPort for DeleteMedicationService {
    fn execute(
        &self,
        request: DeleteMedicationRequest,
    ) -> Result<DeleteMedicationResponse, ApplicationError> {
        let uuid = Uuid::parse_str(&request.id)
            .map_err(|_| ApplicationError::InvalidInput("invalid id".into()))?;
        let id = MedicationId::from(uuid);
        self.repository.delete(&id)?;
        Ok(DeleteMedicationResponse {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::fakes::FakeMedicationRepository;

    fn make_service(repo: std::sync::Arc<FakeMedicationRepository>) -> DeleteMedicationService {
        DeleteMedicationService::new(repo)
    }

    #[test]
    fn execute_with_invalid_uuid_returns_invalid_input() {
        let repo = std::sync::Arc::new(FakeMedicationRepository::new());
        let service = make_service(repo);
        let req = DeleteMedicationRequest {
            id: "not-a-uuid".into(),
        };

        let res = service.execute(req);
        assert!(matches!(res, Err(ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn execute_with_valid_uuid_calls_delete() {
        let med = crate::domain::entities::medication::Medication::new(
            crate::domain::value_objects::medication_id::MedicationId::generate(),
            crate::domain::value_objects::medication_name::MedicationName::new("Test").unwrap(),
            crate::domain::value_objects::dosage::Dosage::new(100).unwrap(),
            vec![],
            crate::domain::value_objects::medication_frequency::DoseFrequency::OnceDaily,
        );
        let id = med.id().to_string();
        let repo = std::sync::Arc::new(FakeMedicationRepository::with(vec![med]));
        let service = make_service(repo.clone());

        let res = service.execute(DeleteMedicationRequest { id });
        assert!(res.is_ok());
        assert_eq!(repo.deleted_count(), 1);
    }
}
