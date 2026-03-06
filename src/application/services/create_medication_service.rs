use std::sync::Arc;

use crate::application::dtos::requests::CreateMedicationRequest;
use crate::application::dtos::responses::CreateMedicationResponse;
use crate::application::errors::ApplicationError;
use crate::application::ports::create_medication_port::CreateMedicationPort;
use crate::application::ports::medication_repository_port::MedicationRepository;
use crate::domain::{
    entities::medication::Medication,
    value_objects::{
        dosage::Dosage,
        medication_frequency::DoseFrequency,
        medication_id::MedicationId,
        medication_name::MedicationName,
        scheduled_time::ScheduledTime,
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
