use std::sync::Arc;

use crate::application::dtos::requests::ListAllMedicationsRequest;
use crate::application::dtos::responses::{ListAllMedicationsResponse, MedicationDto};
use crate::application::errors::ApplicationError;
use crate::application::ports::list_all_medications_port::ListAllMedicationsPort;
use crate::application::ports::medication_repository_port::MedicationRepository;

pub struct ListAllMedicationsService {
    repository: Arc<dyn MedicationRepository>,
}

impl ListAllMedicationsService {
    pub fn new(repository: Arc<dyn MedicationRepository>) -> Self {
        Self { repository }
    }
}

impl ListAllMedicationsPort for ListAllMedicationsService {
    fn execute(
        &self,
        _request: ListAllMedicationsRequest,
    ) -> Result<ListAllMedicationsResponse, ApplicationError> {
        let medications = self.repository.find_all()?;
        let dtos = medications
            .into_iter()
            .map(|m| MedicationDto {
                id: m.id().to_string(),
                name: m.name().value().to_string(),
                amount_mg: m.dosage().amount_mg(),
                scheduled_time: m
                    .scheduled_time()
                    .iter()
                    .map(|t| (t.hour(), t.minute()))
                    .collect(),
                dose_frequency: m.dose_frequency().to_string(),
            })
            .collect();
        Ok(ListAllMedicationsResponse { medications: dtos })
    }
}
