use std::sync::Arc;
use uuid::Uuid;

use crate::application::dtos::requests::EditMedicationRequest;
use crate::application::dtos::responses::EditMedicationResponse;
use crate::application::errors::ApplicationError;
use crate::application::ports::inbound::edit_medication_port::EditMedicationPort;
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
