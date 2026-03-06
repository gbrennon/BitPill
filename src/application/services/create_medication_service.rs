use std::sync::Arc;

use crate::application::dtos::requests::CreateMedicationRequest;
use crate::application::dtos::responses::CreateMedicationResponse;
use crate::application::errors::ApplicationError;
use crate::application::ports::create_medication_port::CreateMedicationPort;
use crate::application::ports::medication_repository_port::MedicationRepository;
use crate::domain::{
    entities::medication::Medication,
    value_objects::{
        dosage::Dosage, medication_frequency::DoseFrequency, medication_id::MedicationId,
        medication_name::MedicationName, scheduled_time::ScheduledTime,
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
        let medication = Medication::from(request)?;

        self.repository.save(&medication)?;

        Ok(CreateMedicationResponse {
            id: medication.id().to_string(),
        })
    }
}
