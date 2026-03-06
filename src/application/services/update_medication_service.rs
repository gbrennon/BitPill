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
        let uuid = Uuid::parse_str(&request.id)
            .map_err(|_| ApplicationError::InvalidInput("invalid id".into()))?;
        let id = MedicationId::from(uuid);

        let name = MedicationName::new(request.name)?;
        let dosage = Dosage::new(request.amount_mg)?;
        let mut scheduled_time = Vec::new();
        for (h, m) in request.scheduled_time {
            l
            scheduled_time.push(ScheduledTime::new(h, m)?);
        }

        let dose_frequency = match request.dose_frequency.as_str() {-
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
