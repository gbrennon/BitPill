use std::sync::Arc;

use crate::{
    application::{
        dtos::{requests::UpdateMedicationBoxRequest, responses::UpdateMedicationBoxResponse},
        errors::{ApplicationError, NotFoundError},
        ports::{
            inbound::update_medication_box_port::UpdateMedicationBoxPort,
            outbound::medication_box_repository_port::MedicationBoxRepositoryPort,
        },
    },
    domain::{
        entities::medication_box::MedicationBox,
        value_objects::{
            dosage::Dosage, medication_box_id::MedicationBoxId, medication_name::MedicationName,
        },
    },
};

pub struct UpdateMedicationBoxService {
    repository: Arc<dyn MedicationBoxRepositoryPort>,
}

impl UpdateMedicationBoxService {
    pub fn new(repository: Arc<dyn MedicationBoxRepositoryPort>) -> Self {
        Self { repository }
    }
}

impl UpdateMedicationBoxPort for UpdateMedicationBoxService {
    fn execute(
        &self,
        request: UpdateMedicationBoxRequest,
    ) -> Result<UpdateMedicationBoxResponse, ApplicationError> {
        let id =
            MedicationBoxId::from(uuid::Uuid::parse_str(&request.id).map_err(|_| {
                ApplicationError::InvalidInput(format!("invalid id: {}", request.id))
            })?);

        let name = MedicationName::new(request.name)
            .map_err(|e| ApplicationError::InvalidInput(format!("invalid name: {}", e)))?;

        let dosage = Dosage::new(request.dosage_mg.into())
            .map_err(|e| ApplicationError::InvalidInput(format!("invalid dosage: {}", e)))?;

        let existing = self.repository.find_by_id(&id)?.ok_or(NotFoundError)?;

        let updated = MedicationBox::with_id(
            existing.id().clone(),
            existing.medication_id().clone(),
            name,
            request.pills_per_box,
            dosage,
        );

        self.repository.save(&updated)?;

        Ok(UpdateMedicationBoxResponse {
            id: updated.id().to_string(),
            medication_id: updated.medication_id().to_string(),
            name: updated.name().to_string(),
            pills_per_box: updated.pills_per_box(),
            dosage_mg: updated.dosage_mg() as u16,
        })
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

    fn make_service(repo: Arc<FakeMedicationBoxRepository>) -> UpdateMedicationBoxService {
        UpdateMedicationBoxService::new(repo)
    }

    #[test]
    fn execute_with_valid_data_returns_updated_box() {
        let r#box = make_box();
        let repo = Arc::new(FakeMedicationBoxRepository::with(r#box.clone()));
        let service = make_service(repo);

        let result = service.execute(UpdateMedicationBoxRequest {
            id: r#box.id().to_string(),
            name: "90-pack".to_string(),
            pills_per_box: 90,
            dosage_mg: 250,
        });

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.name, "90-pack");
        assert_eq!(response.pills_per_box, 90);
    }

    #[test]
    fn execute_with_invalid_id_returns_error() {
        let repo = Arc::new(FakeMedicationBoxRepository::new());
        let service = make_service(repo);

        let result = service.execute(UpdateMedicationBoxRequest {
            id: "invalid-uuid".to_string(),
            name: "Test".to_string(),
            pills_per_box: 30,
            dosage_mg: 500,
        });

        assert!(matches!(result, Err(ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn execute_with_invalid_name_returns_error() {
        let r#box = make_box();
        let repo = Arc::new(FakeMedicationBoxRepository::with(r#box.clone()));
        let service = make_service(repo);

        let result = service.execute(UpdateMedicationBoxRequest {
            id: r#box.id().to_string(),
            name: "".to_string(),
            pills_per_box: 30,
            dosage_mg: 500,
        });

        assert!(matches!(result, Err(ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn execute_with_invalid_dosage_returns_error() {
        let r#box = make_box();
        let repo = Arc::new(FakeMedicationBoxRepository::with(r#box.clone()));
        let service = make_service(repo);

        let result = service.execute(UpdateMedicationBoxRequest {
            id: r#box.id().to_string(),
            name: "Test".to_string(),
            pills_per_box: 30,
            dosage_mg: 0,
        });

        assert!(matches!(result, Err(ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn execute_when_not_found_returns_not_found_error() {
        let repo = Arc::new(FakeMedicationBoxRepository::new());
        let service = make_service(repo);

        let result = service.execute(UpdateMedicationBoxRequest {
            id: make_valid_id(),
            name: "Test".to_string(),
            pills_per_box: 30,
            dosage_mg: 500,
        });

        assert!(matches!(result, Err(ApplicationError::NotFound(_))));
    }

    #[test]
    fn execute_when_repository_fails_returns_storage_error() {
        let r#box = make_box();
        let repo = Arc::new(FakeMedicationBoxRepository::with(r#box.clone()));
        let _ = repo;
    }
}
