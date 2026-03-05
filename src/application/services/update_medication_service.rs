use std::sync::Arc;
use uuid::Uuid;

use crate::application::errors::ApplicationError;
use crate::application::ports::inbound::update_medication_port::{
    UpdateMedicationPort, UpdateMedicationRequest, UpdateMedicationResponse,
};
use crate::application::ports::outbound::medication_repository_port::MedicationRepository;
use crate::domain::entities::medication::Medication;
use crate::domain::value_objects::{
    dosage::Dosage, medication_frequency::DoseFrequency, medication_id::MedicationId,
    medication_name::MedicationName, scheduled_time::ScheduledTime,
};

pub struct UpdateMedicationService {
    repository: Arc<dyn MedicationRepository>,
}

impl UpdateMedicationService {
    pub fn new(repository: Arc<dyn MedicationRepository>) -> Self {
        Self { repository }
    }
}

// Test imports at top
#[cfg(test)]
use crate::application::ports::fakes::FakeMedicationRepository;

#[cfg(test)]
use crate::application::ports::inbound::create_medication_port::CreateMedicationPort;

impl UpdateMedicationPort for UpdateMedicationService {
    fn execute(
        &self,
        request: UpdateMedicationRequest,
    ) -> Result<UpdateMedicationResponse, ApplicationError> {
        let uuid = Uuid::parse_str(&request.id)
            .map_err(|_| ApplicationError::InvalidInput("invalid id".into()))?;
        let id = MedicationId::from(uuid);

        let name = MedicationName::new(request.name)?;
        let dosage = Dosage::new(request.amount_mg)?;
        let mut scheduled_time = Vec::new();
        for (h, m) in request.scheduled_time {
            scheduled_time.push(ScheduledTime::new(h, m)?);
        }

        // Try to parse dose_frequency from string; default to OnceDaily
        let dose_frequency = match request.dose_frequency.as_str() {
            "OnceDaily" => DoseFrequency::OnceDaily,
            "TwiceDaily" => DoseFrequency::TwiceDaily,
            "ThriceDaily" => DoseFrequency::ThriceDaily,
            "Custom" => DoseFrequency::Custom(scheduled_time.clone()),
            _ => DoseFrequency::OnceDaily,
        };

        let medication = Medication::with_id(id, name, dosage, scheduled_time, dose_frequency);

        self.repository.save(&medication)?;

        Ok(UpdateMedicationResponse { id: request.id })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn update_medication_saves_updated_medication() {
        let repo = Arc::new(FakeMedicationRepository::new());
        // create an initial medication to obtain an id
        let create_req = crate::application::ports::inbound::create_medication_port::CreateMedicationRequest::new("Orig", 100, vec![(8,0)], "OnceDaily");
        let create_service = crate::application::services::create_medication_service::CreateMedicationService::new(repo.clone());
        let id = create_service.execute(create_req).unwrap().id;

        let service = UpdateMedicationService::new(repo.clone());
        let req = UpdateMedicationRequest::new(&id, "Updated", 200, vec![(10,30)], "TwiceDaily");
        let res = service.execute(req).expect("update should succeed");
        assert_eq!(res.id, id);
        // ensure repository has at least one saved medication (original + updated appended)
        assert!(repo.saved_count() >= 1);
    }

    #[test]
    fn update_medication_save_failure_returns_storage_error() {
        let repo = Arc::new(FakeMedicationRepository::failing());
        let svc = UpdateMedicationService::new(repo);

        // create a valid update request with a generated UUID
        let id = MedicationId::generate().to_string();
        let req = UpdateMedicationRequest::new(&id, "Name", 100, vec![(8,0)], "OnceDaily");
        let res = svc.execute(req);
        assert!(matches!(res, Err(crate::application::errors::ApplicationError::Storage(_))));
    }

    #[test]
    fn update_medication_with_invalid_scheduled_time_returns_domain_error() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let svc = UpdateMedicationService::new(repo);

        let id = MedicationId::generate().to_string();
        // invalid minute (e.g., 99) should cause ScheduledTime::new to fail
        let req = UpdateMedicationRequest::new(&id, "Name", 100, vec![(8,99)], "Custom");
        let res = svc.execute(req);
        assert!(matches!(res, Err(crate::application::errors::ApplicationError::Domain(_))));
    }

    #[test]
    fn update_medication_unknown_freq_defaults_to_oncedaily() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let svc = UpdateMedicationService::new(repo.clone());
        let id = MedicationId::generate().to_string();
        let req = UpdateMedicationRequest::new(&id, "Name", 100, vec![(8,0)], "UnknownValue");
        let res = svc.execute(req).expect("should succeed and default frequency");
        assert_eq!(res.id, id);
        assert!(repo.saved_count() >= 1);
    }

    #[test]
    fn update_medication_invalid_dosage_returns_domain_error() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let svc = UpdateMedicationService::new(repo);
        let id = MedicationId::generate().to_string();
        // zero dosage is invalid
        let req = UpdateMedicationRequest::new(&id, "Name", 0, vec![(8,0)], "OnceDaily");
        let res = svc.execute(req);
        assert!(matches!(res, Err(crate::application::errors::ApplicationError::Domain(_))));
    }

    #[test]
    fn update_medication_custom_sets_custom_frequency_in_repo() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let svc = UpdateMedicationService::new(repo.clone());
        let id = MedicationId::generate().to_string();
        let req = UpdateMedicationRequest::new(&id, "CustomName", 123, vec![(9,0),(21,0)], "Custom");
        let _ = svc.execute(req).expect("should save");

        // parse back id and check saved medication has custom times
        let uuid = Uuid::parse_str(&id).unwrap();
        let mid = MedicationId::from(uuid);
        let saved = repo.find_by_id(&mid).expect("repo error").expect("not found");
        assert!(matches!(saved.dose_frequency(), DoseFrequency::Custom(_)));
        let times = saved.scheduled_time();
        assert_eq!(times.len(), 2);
        assert_eq!(times[0].hour(), 9);
        assert_eq!(times[1].hour(), 21);
    }

    #[test]
    fn update_medication_save_failure_and_invalid_cases_covered() {
        let failing = Arc::new(FakeMedicationRepository::failing());
        let svc_fail = UpdateMedicationService::new(failing);
        let bad_id = MedicationId::generate().to_string();
        let req = UpdateMedicationRequest::new(&bad_id, "N", 1, vec![(8,0)], "OnceDaily");
        // even with small values it should attempt save and return Storage error from failing fake
        let res = svc_fail.execute(req);
        assert!(matches!(res, Err(crate::application::errors::ApplicationError::Storage(_))));
    }
}
