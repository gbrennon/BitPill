use std::sync::Arc;

use uuid::Uuid;

use crate::{
    application::{
        dtos::{requests::EditMedicationRequest, responses::EditMedicationResponse},
        errors::ApplicationError,
        ports::{
            inbound::edit_medication_port::EditMedicationPort,
            outbound::medication_repository_port::MedicationRepository,
        },
    },
    domain::{
        entities::medication::Medication,
        value_objects::{
            dosage::Dosage, medication_frequency::DoseFrequency, medication_id::MedicationId,
            medication_name::MedicationName, scheduled_time::ScheduledTime,
        },
    },
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

    fn make_service(repo: std::sync::Arc<FakeMedicationRepository>) -> EditMedicationService {
        EditMedicationService::new(repo)
    }

    #[test]
    fn execute_with_invalid_uuid_returns_invalid_input() {
        let repo = std::sync::Arc::new(FakeMedicationRepository::new());
        let service = make_service(repo);
        let req = EditMedicationRequest::new("not-a-uuid", "Name", 100, vec![(8, 0)], "OnceDaily");

        let res = service.execute(req);
        assert!(matches!(res, Err(ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn execute_with_empty_name_returns_domain_error() {
        let repo = std::sync::Arc::new(FakeMedicationRepository::new());
        let service = make_service(repo);
        let id = uuid::Uuid::now_v7().to_string();
        let req = EditMedicationRequest::new(id, "", 100, vec![(8, 0)], "OnceDaily");

        let res = service.execute(req);
        assert!(matches!(res, Err(ApplicationError::Domain(_))));
    }

    #[test]
    fn execute_with_valid_request_saves() {
        let repo = std::sync::Arc::new(FakeMedicationRepository::new());
        let service = make_service(repo.clone());
        let id = uuid::Uuid::now_v7().to_string();
        let req = EditMedicationRequest::new(id.clone(), "Test", 100, vec![(8, 0)], "OnceDaily");

        let res = service.execute(req).unwrap();
        assert_eq!(res.id, id);
        assert_eq!(repo.saved_count(), 1);
    }
}
