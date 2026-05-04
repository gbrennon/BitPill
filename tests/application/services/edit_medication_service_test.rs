use std::sync::Arc;

use bitpill::application::{
    dtos::requests::{CreateMedicationRequest, EditMedicationRequest},
    ports::{
        inbound::{
            create_medication_port::CreateMedicationPort, edit_medication_port::EditMedicationPort,
        },
        outbound::medication_repository_port::MedicationRepository,
    },
    services::{
        create_medication_service::CreateMedicationService,
        edit_medication_service::EditMedicationService,
    },
};

use crate::fakes::FakeMedicationRepository;

#[cfg(test)]
mod tests {
    use super::*;

    fn seed_medication(repo: Arc<FakeMedicationRepository>) -> String {
        let create_service = CreateMedicationService::new(repo);
        let req = CreateMedicationRequest::new("Original", 100, vec![(8, 0)], "OnceDaily");
        create_service.execute(req).unwrap().id
    }

    #[test]
    fn execute_with_valid_inputs_returns_response() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let id = seed_medication(repo.clone());
        let service = EditMedicationService::new(repo);

        let result = service.execute(EditMedicationRequest::new(
            &id,
            "Updated",
            200,
            vec![(9, 0)],
            "OnceDaily",
        ));

        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, id);
    }

    #[test]
    fn execute_saves_updated_medication_to_repository() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let id = seed_medication(repo.clone());
        let service = EditMedicationService::new(repo.clone());

        service
            .execute(EditMedicationRequest::new(
                &id,
                "UpdatedName",
                250,
                vec![(10, 30), (22, 0)],
                "TwiceDaily",
            ))
            .unwrap();

        let saved = repo.find_all().unwrap();
        let med = saved
            .iter()
            .rev()
            .find(|m: &&bitpill::domain::entities::medication::Medication| m.id().to_string() == id)
            .unwrap();
        assert_eq!(med.name().value(), "UpdatedName");
        assert_eq!(med.dosage().amount_mg(), 250);
    }

    #[test]
    fn execute_with_invalid_uuid_returns_error() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let service = EditMedicationService::new(repo);

        let result = service.execute(EditMedicationRequest::new(
            "not-a-uuid",
            "Name",
            100,
            vec![(8, 0)],
            "OnceDaily",
        ));

        assert!(result.is_err());
    }

    #[test]
    fn execute_with_empty_name_returns_error() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let id = seed_medication(repo.clone());
        let service = EditMedicationService::new(repo);

        let result = service.execute(EditMedicationRequest::new(
            &id,
            "",
            100,
            vec![(8, 0)],
            "OnceDaily",
        ));

        assert!(result.is_err());
    }

    #[test]
    fn execute_with_thrice_daily_frequency() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let id = seed_medication(repo.clone());
        let service = EditMedicationService::new(repo);

        let result = service.execute(EditMedicationRequest::new(
            &id,
            "Med",
            100,
            vec![(8, 0), (14, 0), (20, 0)],
            "ThriceDaily",
        ));

        assert!(result.is_ok());
    }

    #[test]
    fn execute_with_custom_frequency() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let id = seed_medication(repo.clone());
        let service = EditMedicationService::new(repo);

        let result = service.execute(EditMedicationRequest::new(
            &id,
            "Med",
            100,
            vec![(8, 0), (12, 0), (16, 0), (20, 0)],
            "Custom",
        ));

        assert!(result.is_ok());
    }

    #[test]
    fn execute_with_invalid_dosage_returns_domain_error() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let id = seed_medication(repo.clone());
        let service = EditMedicationService::new(repo);

        let result = service.execute(EditMedicationRequest::new(
            &id,
            "Med",
            0,
            vec![(8, 0)],
            "OnceDaily",
        ));

        assert!(result.is_err());
    }

    #[test]
    fn execute_with_invalid_time_returns_domain_error() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let id = seed_medication(repo.clone());
        let service = EditMedicationService::new(repo);

        let result = service.execute(EditMedicationRequest::new(
            &id,
            "Med",
            100,
            vec![(25, 0)],
            "OnceDaily",
        ));

        assert!(result.is_err());
    }

    #[test]
    fn execute_when_repository_save_fails_returns_storage_error() {
        use bitpill::application::errors::ApplicationError;
        let repo = Arc::new(FakeMedicationRepository::failing());
        let id =
            bitpill::domain::value_objects::medication_id::MedicationId::generate().to_string();
        let service = EditMedicationService::new(repo);

        let result = service.execute(EditMedicationRequest::new(
            &id,
            "Med",
            100,
            vec![(8, 0)],
            "OnceDaily",
        ));

        assert!(matches!(result, Err(ApplicationError::Storage(_))));
    }
}
