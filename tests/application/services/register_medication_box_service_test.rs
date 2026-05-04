use std::sync::Arc;

use bitpill::{
    application::{
        dtos::requests::RegisterMedicationBoxRequest, errors::ApplicationError,
        ports::inbound::register_medication_box_port::RegisterMedicationBoxPort,
        services::register_medication_box_service::RegisterMedicationBoxService,
    },
    domain::{
        entities::medication::Medication,
        value_objects::{
            dosage::Dosage, medication_frequency::DoseFrequency, medication_id::MedicationId,
            medication_name::MedicationName, scheduled_time::ScheduledTime,
        },
    },
};

use crate::fakes::{FakeMedicationBoxRepository, FakeMedicationRepository};

#[cfg(test)]
mod tests {
    use super::*;

    fn make_medication_repo_with_med(medication_id: MedicationId) -> FakeMedicationRepository {
        let mut repo = FakeMedicationRepository::new();
        repo.set_find_by_id_result(Some(
            Medication::new(
                medication_id,
                MedicationName::new("TestMed").unwrap(),
                Dosage::new(500).unwrap(),
                vec![ScheduledTime::new(8, 0).unwrap()],
                DoseFrequency::OnceDaily,
            )
            .unwrap(),
        ));
        repo
    }

    #[test]
    fn execute_with_valid_inputs_returns_response() {
        let med_id = MedicationId::generate();
        let med_id_str = med_id.to_string();
        let box_repo = Arc::new(FakeMedicationBoxRepository::new());
        let med_repo = Arc::new(make_medication_repo_with_med(med_id.clone()));
        let service = RegisterMedicationBoxService::new(box_repo, med_repo);

        let result = service.execute(RegisterMedicationBoxRequest {
            medication_id: med_id_str,
            name: "30-pack".to_string(),
            pills_per_box: 30,
            dosage_mg: 500,
        });

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.name, "30-pack");
        assert_eq!(response.pills_per_box, 30);
        assert_eq!(response.dosage_mg, 500);
    }

    #[test]
    fn execute_saves_box_to_repository() {
        let med_id = MedicationId::generate();
        let med_id_str = med_id.to_string();
        let box_repo = Arc::new(FakeMedicationBoxRepository::new());
        let med_repo = Arc::new(make_medication_repo_with_med(med_id.clone()));
        let service = RegisterMedicationBoxService::new(box_repo.clone(), med_repo);

        let result = service.execute(RegisterMedicationBoxRequest {
            medication_id: med_id_str,
            name: "30-pack".to_string(),
            pills_per_box: 30,
            dosage_mg: 500,
        });

        assert!(result.is_ok());
        assert_eq!(box_repo.saved_count(), 1);
    }

    #[test]
    fn execute_with_invalid_medication_id_returns_invalid_input_error() {
        let box_repo = Arc::new(FakeMedicationBoxRepository::new());
        let med_repo = Arc::new(FakeMedicationRepository::new());
        let service = RegisterMedicationBoxService::new(box_repo, med_repo);

        let result = service.execute(RegisterMedicationBoxRequest {
            medication_id: "not-a-valid-uuid".to_string(),
            name: "30-pack".to_string(),
            pills_per_box: 30,
            dosage_mg: 500,
        });

        assert!(matches!(result, Err(ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn execute_when_medication_not_found_returns_not_found_error() {
        let med_id = MedicationId::generate();
        let box_repo = Arc::new(FakeMedicationBoxRepository::new());
        let med_repo = Arc::new(FakeMedicationRepository::new());
        let service = RegisterMedicationBoxService::new(box_repo, med_repo);

        let result = service.execute(RegisterMedicationBoxRequest {
            medication_id: med_id.to_string(),
            name: "30-pack".to_string(),
            pills_per_box: 30,
            dosage_mg: 500,
        });

        assert!(matches!(result, Err(ApplicationError::NotFound(_))));
    }

    #[test]
    fn execute_with_invalid_name_returns_invalid_input_error() {
        let med_id = MedicationId::generate();
        let box_repo = Arc::new(FakeMedicationBoxRepository::new());
        let med_repo = Arc::new(make_medication_repo_with_med(med_id.clone()));
        let service = RegisterMedicationBoxService::new(box_repo, med_repo);

        let result = service.execute(RegisterMedicationBoxRequest {
            medication_id: med_id.to_string(),
            name: "".to_string(),
            pills_per_box: 30,
            dosage_mg: 500,
        });

        assert!(matches!(result, Err(ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn execute_with_invalid_dosage_returns_invalid_input_error() {
        let med_id = MedicationId::generate();
        let box_repo = Arc::new(FakeMedicationBoxRepository::new());
        let med_repo = Arc::new(make_medication_repo_with_med(med_id.clone()));
        let service = RegisterMedicationBoxService::new(box_repo, med_repo);

        let result = service.execute(RegisterMedicationBoxRequest {
            medication_id: med_id.to_string(),
            name: "30-pack".to_string(),
            pills_per_box: 30,
            dosage_mg: 0,
        });

        assert!(matches!(result, Err(ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn execute_when_box_repository_fails_returns_storage_error() {
        let med_id = MedicationId::generate();
        let box_repo = Arc::new(FakeMedicationBoxRepository::failing());
        let med_repo = Arc::new(make_medication_repo_with_med(med_id.clone()));
        let service = RegisterMedicationBoxService::new(box_repo, med_repo);

        let result = service.execute(RegisterMedicationBoxRequest {
            medication_id: med_id.to_string(),
            name: "30-pack".to_string(),
            pills_per_box: 30,
            dosage_mg: 500,
        });

        assert!(matches!(result, Err(ApplicationError::Storage(_))));
    }
}
