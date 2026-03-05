use std::sync::Arc;
use uuid::Uuid;

use crate::application::errors::{ApplicationError, NotFoundError};
use crate::application::ports::inbound::get_medication_port::{
    GetMedicationPort, GetMedicationRequest, GetMedicationResponse, MedicationDto,
};
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
                    dose_frequency: match m.dose_frequency() {
                        crate::domain::value_objects::medication_frequency::DoseFrequency::OnceDaily => "OnceDaily".into(),
                        crate::domain::value_objects::medication_frequency::DoseFrequency::TwiceDaily => "TwiceDaily".into(),
                        crate::domain::value_objects::medication_frequency::DoseFrequency::ThriceDaily => "ThriceDaily".into(),
                        crate::domain::value_objects::medication_frequency::DoseFrequency::Custom(_) => "Custom".into(),
                        crate::domain::value_objects::medication_frequency::DoseFrequency::EveryXHours(_) => "EveryXHours".into(),
                    },
                },
            }),
            None => Err(ApplicationError::NotFound(NotFoundError)),
        }
    }
}
