use std::sync::Arc;

use crate::{
    application::{
        dtos::{requests::RegisterMedicationBoxRequest, responses::RegisterMedicationBoxResponse},
        errors::{ApplicationError, NotFoundError},
        ports::{
            inbound::register_medication_box_port::RegisterMedicationBoxPort,
            outbound::{MedicationBoxRepositoryPort, MedicationRepository},
        },
    },
    domain::{
        entities::medication_box::MedicationBox,
        value_objects::{
            dosage::Dosage, medication_id::MedicationId, medication_name::MedicationName,
        },
    },
};

pub struct RegisterMedicationBoxService {
    medication_box_repository: Arc<dyn MedicationBoxRepositoryPort>,
    medication_repository: Arc<dyn MedicationRepository>,
}

impl RegisterMedicationBoxService {
    pub fn new(
        medication_box_repository: Arc<dyn MedicationBoxRepositoryPort>,
        medication_repository: Arc<dyn MedicationRepository>,
    ) -> Self {
        Self {
            medication_box_repository,
            medication_repository,
        }
    }
}

impl RegisterMedicationBoxPort for RegisterMedicationBoxService {
    fn execute(
        &self,
        request: RegisterMedicationBoxRequest,
    ) -> Result<RegisterMedicationBoxResponse, ApplicationError> {
        let medication_id =
            MedicationId::from(uuid::Uuid::parse_str(&request.medication_id).map_err(|_| {
                ApplicationError::InvalidInput(format!(
                    "invalid medication id: {}",
                    request.medication_id
                ))
            })?);

        let _medication = self
            .medication_repository
            .find_by_id(&medication_id)?
            .ok_or(NotFoundError)?;

        let name = MedicationName::new(request.name)
            .map_err(|e| ApplicationError::InvalidInput(format!("invalid name: {}", e)))?;

        let dosage = Dosage::new(request.dosage_mg.into())
            .map_err(|e| ApplicationError::InvalidInput(format!("invalid dosage: {}", e)))?;

        let medication_box = MedicationBox::new(medication_id, name, request.pills_per_box, dosage);

        self.medication_box_repository.save(&medication_box)?;

        Ok(RegisterMedicationBoxResponse {
            id: medication_box.id().to_string(),
            medication_id: request.medication_id,
            name: medication_box.name().to_string(),
            pills_per_box: medication_box.pills_per_box(),
            dosage_mg: medication_box.dosage_mg() as u16,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::{
        application::ports::fakes::{FakeMedicationBoxRepository, FakeMedicationRepository},
        domain::{
            entities::medication::Medication,
            value_objects::{medication_frequency::DoseFrequency, scheduled_time::ScheduledTime},
        },
    };

    fn make_valid_medication_id() -> String {
        "018f8a2e-0000-0000-0000-000000000001".to_string()
    }

    fn make_request(
        medication_id: &str,
        name: &str,
        pills_per_box: u16,
        dosage_mg: u16,
    ) -> RegisterMedicationBoxRequest {
        RegisterMedicationBoxRequest {
            medication_id: medication_id.to_string(),
            name: name.to_string(),
            pills_per_box,
            dosage_mg,
        }
    }

    fn make_service(
        box_repo: Arc<FakeMedicationBoxRepository>,
        med_repo: Arc<FakeMedicationRepository>,
    ) -> RegisterMedicationBoxService {
        RegisterMedicationBoxService::new(box_repo, med_repo)
    }

    #[test]
    fn execute_with_valid_inputs_returns_response() {
        let medication_id = make_valid_medication_id();
        let med_id = MedicationId::from(uuid::Uuid::parse_str(&medication_id).unwrap());

        let box_repo = Arc::new(FakeMedicationBoxRepository::new());
        let mut med_repo = FakeMedicationRepository::new();
        med_repo.set_find_by_id_result(Some(
            Medication::new(
                med_id,
                MedicationName::new("TestMed").unwrap(),
                Dosage::new(500).unwrap(),
                vec![ScheduledTime::new(8, 0).unwrap()],
                DoseFrequency::OnceDaily,
            )
            .unwrap(),
        ));

        let service = make_service(box_repo.clone(), Arc::new(med_repo));
        let result = service.execute(make_request(&medication_id, "30-pack", 30, 500));

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.medication_id, medication_id);
        assert_eq!(response.name, "30-pack");
        assert_eq!(response.pills_per_box, 30);
        assert_eq!(response.dosage_mg, 500);
        assert!(!response.id.is_empty());
    }

    #[test]
    fn execute_saves_box_to_repository() {
        let medication_id = make_valid_medication_id();
        let med_id = MedicationId::from(uuid::Uuid::parse_str(&medication_id).unwrap());

        let box_repo = Arc::new(FakeMedicationBoxRepository::new());
        let mut med_repo = FakeMedicationRepository::new();
        med_repo.set_find_by_id_result(Some(
            Medication::new(
                med_id,
                MedicationName::new("TestMed").unwrap(),
                Dosage::new(500).unwrap(),
                vec![ScheduledTime::new(8, 0).unwrap()],
                DoseFrequency::OnceDaily,
            )
            .unwrap(),
        ));

        let service = make_service(box_repo.clone(), Arc::new(med_repo));
        service
            .execute(make_request(&medication_id, "30-pack", 30, 500))
            .unwrap();

        assert_eq!(box_repo.saved_count(), 1);
    }

    #[test]
    fn execute_with_invalid_medication_id_returns_error() {
        let box_repo = Arc::new(FakeMedicationBoxRepository::new());
        let med_repo = Arc::new(FakeMedicationRepository::new());

        let service = make_service(box_repo, med_repo);
        let result = service.execute(make_request("not-a-valid-uuid", "30-pack", 30, 500));

        assert!(matches!(result, Err(ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn execute_when_medication_not_found_returns_not_found_error() {
        let medication_id = make_valid_medication_id();

        let box_repo = Arc::new(FakeMedicationBoxRepository::new());
        let med_repo = Arc::new(FakeMedicationRepository::new());

        let service = make_service(box_repo, med_repo);
        let result = service.execute(make_request(&medication_id, "30-pack", 30, 500));

        assert!(matches!(result, Err(ApplicationError::NotFound(_))));
    }

    #[test]
    fn execute_with_invalid_name_returns_error() {
        let medication_id = make_valid_medication_id();
        let med_id = MedicationId::from(uuid::Uuid::parse_str(&medication_id).unwrap());

        let box_repo = Arc::new(FakeMedicationBoxRepository::new());
        let mut med_repo = FakeMedicationRepository::new();
        med_repo.set_find_by_id_result(Some(
            Medication::new(
                med_id,
                MedicationName::new("TestMed").unwrap(),
                Dosage::new(500).unwrap(),
                vec![ScheduledTime::new(8, 0).unwrap()],
                DoseFrequency::OnceDaily,
            )
            .unwrap(),
        ));

        let service = make_service(box_repo, Arc::new(med_repo));
        let result = service.execute(make_request(&medication_id, "", 30, 500));

        assert!(matches!(result, Err(ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn execute_with_invalid_dosage_returns_error() {
        let medication_id = make_valid_medication_id();
        let med_id = MedicationId::from(uuid::Uuid::parse_str(&medication_id).unwrap());

        let box_repo = Arc::new(FakeMedicationBoxRepository::new());
        let mut med_repo = FakeMedicationRepository::new();
        med_repo.set_find_by_id_result(Some(
            Medication::new(
                med_id,
                MedicationName::new("TestMed").unwrap(),
                Dosage::new(500).unwrap(),
                vec![ScheduledTime::new(8, 0).unwrap()],
                DoseFrequency::OnceDaily,
            )
            .unwrap(),
        ));

        let service = make_service(box_repo, Arc::new(med_repo));
        let result = service.execute(make_request(&medication_id, "30-pack", 30, 0));

        assert!(matches!(result, Err(ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn execute_when_repository_fails_returns_storage_error() {
        let medication_id = make_valid_medication_id();
        let med_id = MedicationId::from(uuid::Uuid::parse_str(&medication_id).unwrap());

        let box_repo = Arc::new(FakeMedicationBoxRepository::failing());
        let mut med_repo = FakeMedicationRepository::new();
        med_repo.set_find_by_id_result(Some(
            Medication::new(
                med_id,
                MedicationName::new("TestMed").unwrap(),
                Dosage::new(500).unwrap(),
                vec![ScheduledTime::new(8, 0).unwrap()],
                DoseFrequency::OnceDaily,
            )
            .unwrap(),
        ));

        let service = make_service(box_repo, Arc::new(med_repo));
        let result = service.execute(make_request(&medication_id, "30-pack", 30, 500));

        assert!(matches!(result, Err(ApplicationError::Storage(_))));
    }
}
