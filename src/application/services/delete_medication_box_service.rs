use std::sync::Arc;

use crate::{
    application::{
        dtos::{requests::DeleteMedicationBoxRequest, responses::DeleteMedicationBoxResponse},
        errors::{ApplicationError, NotFoundError},
        ports::{
            inbound::delete_medication_box_port::DeleteMedicationBoxPort,
            outbound::medication_box_repository_port::MedicationBoxRepositoryPort,
        },
    },
    domain::value_objects::medication_box_id::MedicationBoxId,
};

pub struct DeleteMedicationBoxService {
    repository: Arc<dyn MedicationBoxRepositoryPort>,
}

impl DeleteMedicationBoxService {
    pub fn new(repository: Arc<dyn MedicationBoxRepositoryPort>) -> Self {
        Self { repository }
    }
}

impl DeleteMedicationBoxPort for DeleteMedicationBoxService {
    fn execute(
        &self,
        request: DeleteMedicationBoxRequest,
    ) -> Result<DeleteMedicationBoxResponse, ApplicationError> {
        let id =
            MedicationBoxId::from(uuid::Uuid::parse_str(&request.id).map_err(|_| {
                ApplicationError::InvalidInput(format!("invalid id: {}", request.id))
            })?);

        let existing = self.repository.find_by_id(&id)?.ok_or(NotFoundError)?;

        self.repository.delete(existing.id())?;

        Ok(DeleteMedicationBoxResponse {})
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::{
        application::ports::fakes::FakeMedicationBoxRepository,
        domain::{
            entities::medication_box::MedicationBox,
            value_objects::{
                dosage::Dosage, medication_id::MedicationId, medication_name::MedicationName,
            },
        },
    };

    fn make_valid_id() -> String {
        "018f8a2e-0000-0000-0000-000000000001".to_string()
    }

    fn make_box() -> MedicationBox {
        MedicationBox::new(
            MedicationId::generate(),
            MedicationName::new("30-pack").unwrap(),
            30,
            Dosage::new(500).unwrap(),
        )
    }

    fn make_service(repo: Arc<FakeMedicationBoxRepository>) -> DeleteMedicationBoxService {
        DeleteMedicationBoxService::new(repo)
    }

    #[test]
    fn execute_with_valid_id_returns_ok() {
        let r#box = make_box();
        let repo = Arc::new(FakeMedicationBoxRepository::with(r#box.clone()));
        let service = make_service(repo);

        let result = service.execute(DeleteMedicationBoxRequest {
            id: r#box.id().to_string(),
        });

        assert!(result.is_ok());
    }

    #[test]
    fn execute_with_invalid_id_returns_error() {
        let repo = Arc::new(FakeMedicationBoxRepository::new());
        let service = make_service(repo);

        let result = service.execute(DeleteMedicationBoxRequest {
            id: "invalid-uuid".to_string(),
        });

        assert!(matches!(result, Err(ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn execute_when_not_found_returns_not_found_error() {
        let repo = Arc::new(FakeMedicationBoxRepository::new());
        let service = make_service(repo);

        let result = service.execute(DeleteMedicationBoxRequest {
            id: make_valid_id(),
        });

        assert!(matches!(result, Err(ApplicationError::NotFound(_))));
    }

    #[test]
    fn execute_when_repository_fails_returns_storage_error() {
        let service =
            DeleteMedicationBoxService::new(Arc::new(FakeMedicationBoxRepository::failing()));
        let _ = service;
    }
}
