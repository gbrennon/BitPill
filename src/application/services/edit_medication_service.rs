use std::sync::Arc;
use uuid::Uuid;

use crate::application::errors::ApplicationError;
use crate::application::ports::inbound::edit_medication_port::{
    EditMedicationPort, EditMedicationRequest, EditMedicationResponse,
};
use crate::application::ports::outbound::medication_repository_port::MedicationRepository;
use crate::domain::entities::medication::Medication;
use crate::domain::value_objects::{
    dosage::Dosage, medication_frequency::DoseFrequency, medication_id::MedicationId,
    medication_name::MedicationName, scheduled_time::ScheduledTime,
};

pub struct EditMedicationService {
    repository: Arc<dyn MedicationRepository>,
}

impl EditMedicationService {
    pub fn new(repository: Arc<dyn MedicationRepository>) -> Self {
        Self { repository }
    }
}

impl EditMedicationPort for EditMedicationService {
    fn execute(
        &self,
        request: EditMedicationRequest,
    ) -> Result<EditMedicationResponse, ApplicationError> {
        let uuid = Uuid::parse_str(&request.id)
            .map_err(|_| ApplicationError::InvalidInput("invalid id".into()))?;
        let id = MedicationId::from(uuid);

        let name = MedicationName::new(request.name)?;
        let dosage = Dosage::new(request.amount_mg)?;
        let mut scheduled_times = Vec::new();
        for (h, m) in request.scheduled_time {
            scheduled_times.push(ScheduledTime::new(h, m)?);
        }

        let dose_frequency = match request.dose_frequency.as_str() {
            "TwiceDaily" => DoseFrequency::TwiceDaily,
            "ThriceDaily" => DoseFrequency::ThriceDaily,
            "Custom" => DoseFrequency::Custom(scheduled_times.clone()),
            _ => DoseFrequency::OnceDaily,
        };

        let medication = Medication::with_id(id, name, dosage, scheduled_times, dose_frequency);

        self.repository.save(&medication)?;

        Ok(EditMedicationResponse { id: request.id })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::fakes::FakeMedicationRepository;
    use crate::application::ports::inbound::create_medication_port::{
        CreateMedicationPort, CreateMedicationRequest,
    };
    use crate::application::services::create_medication_service::CreateMedicationService;

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
                vec![(10, 30)],
                "TwiceDaily",
            ))
            .unwrap();

        // FakeMedicationRepository appends on save; the last entry is the updated one
        let saved = repo.find_all().unwrap();
        let med = saved
            .iter()
            .rev()
            .find(|m| m.id().to_string() == id)
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
}
