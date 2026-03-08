use std::sync::Arc;
use uuid::Uuid;

use crate::application::dtos::requests::GetMedicationRequest;
use crate::application::dtos::responses::{GetMedicationResponse, MedicationDto};
use crate::application::errors::{ApplicationError, NotFoundError};
use crate::application::ports::inbound::get_medication_port::GetMedicationPort;
use crate::application::ports::outbound::medication_repository_port::MedicationRepository;
use crate::domain::value_objects::medication_id::MedicationId;

pub struct GetMedicationService {
    repository: Arc<dyn MedicationRepository>,
}

impl GetMedicationService {
    pub fn new(repository: Arc<dyn MedicationRepository>) -> Self {
        Self { repository }
    }
}

impl GetMedicationPort for GetMedicationService {
    fn execute(
        &self,
        request: GetMedicationRequest,
    ) -> Result<GetMedicationResponse, ApplicationError> {
        let uuid = Uuid::parse_str(&request.id)
            .map_err(|_| ApplicationError::InvalidInput("invalid id".into()))?;
        let id = MedicationId::from(uuid);
        match self.repository.find_by_id(&id)? {
            Some(m) => Ok(GetMedicationResponse {
                medication: MedicationDto {
                    id: m.id().to_string(),
                    name: m.name().value().to_string(),
                    amount_mg: m.dosage().amount_mg(),
                    scheduled_time: m
                        .scheduled_time()
                        .iter()
                        .map(|t| (t.hour(), t.minute()))
                        .collect(),
                    dose_frequency: m.dose_frequency().as_str().to_string(),
                },
            }),
            None => Err(ApplicationError::NotFound(NotFoundError)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::fakes::FakeMedicationRepository;
    use crate::domain::entities::medication::Medication;
    use crate::domain::value_objects::{
        dosage::Dosage, medication_frequency::DoseFrequency, medication_id::MedicationId,
        medication_name::MedicationName, scheduled_time::ScheduledTime,
    };

    fn make_service(repo: std::sync::Arc<FakeMedicationRepository>) -> GetMedicationService {
        GetMedicationService::new(repo)
    }

    #[test]
    fn execute_when_not_found_returns_not_found_error() {
        let repo = std::sync::Arc::new(FakeMedicationRepository::new());
        let service = make_service(repo);
        let req = super::GetMedicationRequest {
            id: uuid::Uuid::now_v7().to_string(),
        };

        let res = service.execute(req);
        assert!(matches!(res, Err(ApplicationError::NotFound(_))));
    }

    #[test]
    fn execute_when_found_returns_medication_dto() {
        let med = Medication::new(
            MedicationId::generate(),
            MedicationName::new("Test").unwrap(),
            Dosage::new(123).unwrap(),
            vec![ScheduledTime::new(8, 0).unwrap()],
            DoseFrequency::OnceDaily,
        );
        let repo = std::sync::Arc::new(FakeMedicationRepository::with(vec![med.clone()]));
        let service = make_service(repo);
        let req = super::GetMedicationRequest {
            id: med.id().to_string(),
        };

        let res = service.execute(req).unwrap();
        assert_eq!(res.medication.id, med.id().to_string());
        assert_eq!(res.medication.name, med.name().value());
    }
}
