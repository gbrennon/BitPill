use std::sync::Arc;

use crate::application::errors::ApplicationError;
use crate::application::ports::create_medication_port::{
    CreateMedicationPort, CreateMedicationRequest, CreateMedicationResponse,
};
use crate::application::ports::medication_repository_port::MedicationRepository;
use crate::domain::{
    entities::medication::Medication,
    value_objects::{
        dosage::Dosage, medication_id::MedicationId, medication_name::MedicationName,
        scheduled_time::ScheduledTime, medication_frequency::DoseFrequency,
    },
};

pub struct CreateMedicationService {
    repository: Arc<dyn MedicationRepository>,
}

impl CreateMedicationService {
    pub fn new(repository: Arc<dyn MedicationRepository>) -> Self {
        Self { repository }
    }
}

impl CreateMedicationPort for CreateMedicationService {
    fn execute(
        &self,
        request: CreateMedicationRequest,
    ) -> Result<CreateMedicationResponse, ApplicationError> {
        let id = MedicationId::generate();
        let name = MedicationName::new(request.name)?;
        let dosage = Dosage::new(request.amount_mg)?;
        let times = request
            .scheduled_time
            .into_iter()
            .map(|(h, m)| ScheduledTime::new(h, m))
            .collect::<Result<Vec<_>, _>>()?;

        let dose_frequency = match request.dose_frequency.as_str() {
            "OnceDaily" => DoseFrequency::OnceDaily,
            "TwiceDaily" => DoseFrequency::TwiceDaily,
            "ThriceDaily" => DoseFrequency::ThriceDaily,
            "Custom" => DoseFrequency::Custom(times.clone()),
            _ => DoseFrequency::OnceDaily,
        };

        let medication = Medication::new(id, name, dosage, times, dose_frequency);

        self.repository.save(&medication)?;

        Ok(CreateMedicationResponse {
            id: medication.id().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::fakes::FakeMedicationRepository;
    use crate::domain::errors::DomainError;

    fn make_request(
        name: &str,
        amount_mg: u32,
        scheduled_time: Vec<(u32, u32)>,
    ) -> CreateMedicationRequest {
        CreateMedicationRequest::new(name, amount_mg, scheduled_time, "OnceDaily")
    }

    #[test]
    fn execute_with_valid_inputs_returns_response() {
        let service = CreateMedicationService::new(Arc::new(FakeMedicationRepository::new()));

        let result = service.execute(make_request("Levetiracetam", 500, vec![(8, 0), (20, 0)]));

        assert!(result.is_ok());
        assert!(!result.unwrap().id.is_empty());
    }

    #[test]
    fn execute_saves_medication_to_repository() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let service = CreateMedicationService::new(repo.clone());

        service
            .execute(make_request("Ibuprofen", 200, vec![(8, 0)]))
            .unwrap();

        assert_eq!(repo.saved_count(), 1);
    }

    #[test]
    fn execute_with_empty_name_returns_domain_error() {
        let service = CreateMedicationService::new(Arc::new(FakeMedicationRepository::new()));

        let result = service.execute(make_request("", 500, vec![(8, 0)]));

        assert!(matches!(
            result,
            Err(ApplicationError::Domain(DomainError::EmptyMedicationName))
        ));
    }

    #[test]
    fn execute_with_zero_dosage_returns_domain_error() {
        let service = CreateMedicationService::new(Arc::new(FakeMedicationRepository::new()));

        let result = service.execute(make_request("Levetiracetam", 0, vec![(8, 0)]));

        assert!(matches!(
            result,
            Err(ApplicationError::Domain(DomainError::InvalidDosage))
        ));
    }

    #[test]
    fn execute_with_invalid_scheduled_time_returns_domain_error() {
        let service = CreateMedicationService::new(Arc::new(FakeMedicationRepository::new()));

        let result = service.execute(make_request("Levetiracetam", 500, vec![(25, 0)]));

        assert!(matches!(
            result,
            Err(ApplicationError::Domain(DomainError::InvalidScheduledTime))
        ));
    }

    #[test]
    fn execute_when_repository_fails_returns_storage_error() {
        let service = CreateMedicationService::new(Arc::new(FakeMedicationRepository::failing()));

        let result = service.execute(make_request("Levetiracetam", 500, vec![(8, 0)]));

        assert!(matches!(result, Err(ApplicationError::Storage(_))));
    }
}
