use std::sync::Arc;

use crate::application::dtos::requests::CreateDoseRecordRequest;
use crate::application::dtos::responses::CreateDoseRecordResponse;
use crate::application::errors::ApplicationError;
use crate::application::ports::dose_record_repository_port::DoseRecordRepository;
use crate::application::ports::inbound::create_dose_record_port::CreateDoseRecordPort;
use crate::domain::entities::dose_record::DoseRecord;
use crate::domain::value_objects::medication_id::MedicationId;

pub struct CreateDoseRecordsService {
    repository: Arc<dyn DoseRecordRepository>,
}

impl CreateDoseRecordsService {
    pub fn new(repository: Arc<dyn DoseRecordRepository>) -> Self {
        Self { repository }
    }
}

impl CreateDoseRecordPort for CreateDoseRecordsService {
    fn execute(
        &self,
        request: CreateDoseRecordRequest,
    ) -> Result<CreateDoseRecordResponse, ApplicationError> {
        let med_id =
            MedicationId::from(uuid::Uuid::parse_str(&request.medication_id).map_err(|_| {
                ApplicationError::InvalidInput(format!(
                    "invalid medication id: {}",
                    request.medication_id
                ))
            })?);
        let record = DoseRecord::new(med_id, request.scheduled_at);
        self.repository.save(&record)?;
        Ok(CreateDoseRecordResponse {
            id: record.id().to_string(),
        })
    }
}
