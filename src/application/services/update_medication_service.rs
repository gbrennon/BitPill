use std::sync::Arc;
use uuid::Uuid;

use crate::application::dtos::requests::UpdateMedicationRequest;
use crate::application::dtos::responses::UpdateMedicationResponse;
use crate::application::errors::ApplicationError;
use crate::application::ports::inbound::update_medication_port::UpdateMedicationPort;
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

impl UpdateMedicationPort for UpdateMedicationService {
    fn execute(
        &self,
        request: UpdateMedicationRequest,
    ) -> Result<UpdateMedicationResponse, ApplicationError> {
        let id = Uuid::parse_str(&request.id)
            .map_err(|_| ApplicationError::InvalidInput("invalid id".into()))?;

        let medication = MedicationMapper::from_request(request, Some(MedicationId::from(id)))?;

        self.repository.save(&medication)?;

        Ok(UpdateMedicationResponse { id: request.id })
    }
}
