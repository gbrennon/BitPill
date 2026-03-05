use std::sync::Arc;

use crate::application::dtos::requests::MarkMedicationTakenRequest;
use crate::application::dtos::responses::MarkMedicationTakenResponse;
use crate::application::errors::ApplicationError;
use crate::application::ports::dose_record_repository_port::DoseRecordRepository;
use crate::application::ports::inbound::mark_medication_taken_port::MarkMedicationTakenPort;
use crate::domain::entities::dose_record::DoseRecord;
use crate::domain::value_objects::medication_id::MedicationId;

pub struct MarkMedicationTakenService {
    repository: Arc<dyn DoseRecordRepository>,
}

impl MarkMedicationTakenService {
    pub fn new(repository: Arc<dyn DoseRecordRepository>) -> Self {
        Self { repository }
    }
}

impl MarkMedicationTakenPort for MarkMedicationTakenService {
    fn execute(
        &self,
        request: MarkMedicationTakenRequest,
    ) -> Result<MarkMedicationTakenResponse, ApplicationError> {
        let med_id =
            MedicationId::from(uuid::Uuid::parse_str(&request.medication_id).map_err(|_| {
                ApplicationError::InvalidInput(format!(
                    "invalid medication id: {}",
                    request.medication_id
                ))
            })?);

        let mut record = DoseRecord::new(med_id, request.taken_at);
        record
            .mark_taken(request.taken_at)
            .expect("newly created dose record is never pre-taken");
        self.repository.save(&record)?;

        Ok(MarkMedicationTakenResponse {
            id: record.id().to_string(),
        })
    }
}
