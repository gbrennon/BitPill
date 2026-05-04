use std::sync::Arc;

use bitpill::{
    application::{
        dtos::requests::UpdateMedicationBoxRequest, errors::ApplicationError,
        ports::inbound::update_medication_box_port::UpdateMedicationBoxPort,
        services::update_medication_box_service::UpdateMedicationBoxService,
    },
    domain::{
        entities::medication_box::MedicationBox,
        value_objects::{
            dosage::Dosage, medication_id::MedicationId, medication_name::MedicationName,
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
    fn execute_with_valid_data_updates_box() {
        let r#box = make_box();
        let box_id = r#box.id().to_string();
        let repo = Arc::new(FakeMedicationBoxRepository::with(r#box));
        let service = UpdateMedicationBoxService::new(repo);

        let result = service.execute(UpdateMedicationBoxRequest {
            id: box_id,
            name: "90-pack".to_string(),
            pills_per_box: 90,
            dosage_mg: 250,
        });

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.name, "90-pack");
        assert_eq!(response.pills_per_box, 90);
        assert_eq!(response.dosage_mg, 250);
    }

    #[test]
    fn execute_with_invalid_id_returns_invalid_input_error() {
        let repo = Arc::new(FakeMedicationBoxRepository::new());
        let service = UpdateMedicationBoxService::new(repo);

        let result = service.execute(UpdateMedicationBoxRequest {
            id: "invalid-uuid".to_string(),
            name: "Test".to_string(),
            pills_per_box: 30,
            dosage_mg: 500,
        });

        assert!(matches!(result, Err(ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn execute_with_invalid_name_returns_invalid_input_error() {
        let r#box = make_box();
        let box_id = r#box.id().to_string();
        let repo = Arc::new(FakeMedicationBoxRepository::with(r#box));
        let service = UpdateMedicationBoxService::new(repo);

        let result = service.execute(UpdateMedicationBoxRequest {
            id: box_id,
            name: "".to_string(),
            pills_per_box: 30,
            dosage_mg: 500,
        });

        assert!(matches!(result, Err(ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn execute_with_invalid_dosage_returns_invalid_input_error() {
        let r#box = make_box();
        let box_id = r#box.id().to_string();
        let repo = Arc::new(FakeMedicationBoxRepository::with(r#box));
        let service = UpdateMedicationBoxService::new(repo);

        let result = service.execute(UpdateMedicationBoxRequest {
            id: box_id,
            name: "Test".to_string(),
            pills_per_box: 30,
            dosage_mg: 0,
        });

        assert!(matches!(result, Err(ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn execute_when_not_found_returns_not_found_error() {
        let repo = Arc::new(FakeMedicationBoxRepository::new());
        let service = UpdateMedicationBoxService::new(repo);

        let result = service.execute(UpdateMedicationBoxRequest {
            id: "018f8a2e-0000-0000-0000-000000000001".to_string(),
            name: "Test".to_string(),
            pills_per_box: 30,
            dosage_mg: 500,
        });

        assert!(matches!(result, Err(ApplicationError::NotFound(_))));
    }
}
