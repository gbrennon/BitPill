use std::sync::Arc;

use uuid::Uuid;

use crate::application::dtos::requests::MarkDoseTakenRequest;
use crate::application::dtos::responses::MarkDoseTakenResponse;
use crate::application::errors::{ApplicationError, NotFoundError};
use crate::application::ports::dose_record_repository_port::DoseRecordRepository;
use crate::application::ports::mark_dose_taken_port::MarkDoseTakenPort;
use crate::domain::value_objects::dose_record_id::DoseRecordId;
use crate::domain::value_objects::medication_id::MedicationId;
use crate::domain::entities::dose_record::DoseRecord;

pub struct MarkDoseTakenService {
    repository: Arc<dyn DoseRecordRepository>,
    medication_repository: Arc<dyn crate::application::ports::outbound::medication_repository_port::MedicationRepository>,
}

impl MarkDoseTakenService {
    pub fn new(
        repository: Arc<dyn DoseRecordRepository>,
        medication_repository: Arc<dyn crate::application::ports::outbound::medication_repository_port::MedicationRepository>,
    ) -> Self {
        Self { repository, medication_repository }
    }
}

impl MarkDoseTakenPort for MarkDoseTakenService {
    fn execute(
        &self,
        request: MarkDoseTakenRequest,
    ) -> Result<MarkDoseTakenResponse, ApplicationError> {
        // Try to interpret the provided id as a DoseRecord id and lookup.
        let uuid = Uuid::parse_str(&request.record_id).map_err(|_| {
            ApplicationError::InvalidInput(format!("invalid id: {}", request.record_id))
        })?;
        let record_id = DoseRecordId::from(uuid);

        match self.repository.find_by_id(&record_id)? {
            Some(mut record) => {
                // existing record found -> mark as taken
                record.mark_taken(request.taken_at)?;
                self.repository.save(&record)?;
                Ok(MarkDoseTakenResponse {
                    record_id: record.id().to_string(),
                })
            }
            None => {
                // No existing dose record with that id; interpret the provided id as a MedicationId
                let med_id = MedicationId::from(uuid);
                // check medication exists
                let maybe_med = self.medication_repository.find_by_id(&med_id)?;
                if maybe_med.is_none() {
                    return Err(ApplicationError::NotFound(NotFoundError));
                }
                let mut record = DoseRecord::new(med_id, request.taken_at);
                // newly created record should be marked as taken at the provided time
                record.mark_taken(request.taken_at)?;
                self.repository.save(&record)?;
                Ok(MarkDoseTakenResponse {
                    record_id: record.id().to_string(),
                })
            }
        }
    }
}
