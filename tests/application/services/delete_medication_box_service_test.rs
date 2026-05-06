use std::sync::Arc;

use bitpill::{
    application::{
        dtos::requests::DeleteMedicationBoxRequest,
        errors::ApplicationError,
        ports::{
            inbound::delete_medication_box_port::DeleteMedicationBoxPort,
            outbound::medication_box_repository_port::MedicationBoxRepositoryPort,
        },
        services::delete_medication_box_service::DeleteMedicationBoxService,
    },
    domain::{
        entities::medication_box::MedicationBox,
        value_objects::{
            dosage::Dosage, medication_box_id::MedicationBoxId, medication_id::MedicationId,
            medication_name::MedicationName,
        },
    },
};

use crate::fakes::FakeMedicationBoxRepository;

#[cfg(test)]
mod tests {
    use super::*;

    fn make_box() -> MedicationBox {
        MedicationBox::new(
            MedicationId::generate(),
            MedicationName::new("30-pack").unwrap(),
            30,
            Dosage::new(500).unwrap(),
        )
    }

    #[test]
    fn execute_with_valid_id_deletes_and_returns_ok() {
        let r#box = make_box();
        let box_id = r#box.id().clone();
        let repo = Arc::new(FakeMedicationBoxRepository::with(r#box));
        let service = DeleteMedicationBoxService::new(repo.clone());

        let result = service.execute(DeleteMedicationBoxRequest {
            id: box_id.to_string(),
        });

        assert!(result.is_ok());
        let found = repo.find_by_id(&box_id).unwrap();
        assert!(found.is_none());
    }

    #[test]
    fn execute_with_invalid_id_returns_invalid_input_error() {
        let repo = Arc::new(FakeMedicationBoxRepository::new());
        let service = DeleteMedicationBoxService::new(repo);

        let result = service.execute(DeleteMedicationBoxRequest {
            id: "invalid-uuid".to_string(),
        });

        assert!(matches!(result, Err(ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn execute_when_not_found_returns_not_found_error() {
        let repo = Arc::new(FakeMedicationBoxRepository::new());
        let service = DeleteMedicationBoxService::new(repo);

        let result = service.execute(DeleteMedicationBoxRequest {
            id: MedicationBoxId::generate().to_string(),
        });

        assert!(matches!(result, Err(ApplicationError::NotFound(_))));
    }

    #[test]
    fn execute_when_repository_fails_returns_storage_error() {
        let r#box = make_box();
        let box_id = r#box.id().to_string();
        let repo = Arc::new(FakeMedicationBoxRepository::failing_on_delete(r#box));
        let service = DeleteMedicationBoxService::new(repo);

        let result = service.execute(DeleteMedicationBoxRequest { id: box_id });

        assert!(matches!(result, Err(ApplicationError::Storage(_))));
    }
}
