use std::sync::Arc;

use bitpill::{
    application::{
        dtos::requests::GetMedicationBoxRequest, errors::ApplicationError,
        ports::inbound::get_medication_box_port::GetMedicationBoxPort,
        services::get_medication_box_service::GetMedicationBoxService,
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
    fn execute_with_valid_id_returns_box() {
        let r#box = make_box();
        let box_id = r#box.id().to_string();
        let repo = Arc::new(FakeMedicationBoxRepository::with(r#box));
        let service = GetMedicationBoxService::new(repo);

        let result = service.execute(GetMedicationBoxRequest { id: box_id });

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.name, "30-pack");
        assert_eq!(response.pills_per_box, 30);
        assert_eq!(response.dosage_mg, 500);
    }

    #[test]
    fn execute_with_invalid_id_returns_invalid_input_error() {
        let repo = Arc::new(FakeMedicationBoxRepository::new());
        let service = GetMedicationBoxService::new(repo);

        let result = service.execute(GetMedicationBoxRequest {
            id: "invalid-uuid".to_string(),
        });

        assert!(matches!(result, Err(ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn execute_when_not_found_returns_not_found_error() {
        let repo = Arc::new(FakeMedicationBoxRepository::new());
        let service = GetMedicationBoxService::new(repo);

        let result = service.execute(GetMedicationBoxRequest {
            id: "018f8a2e-0000-0000-0000-000000000001".to_string(),
        });

        assert!(matches!(result, Err(ApplicationError::NotFound(_))));
    }
}
