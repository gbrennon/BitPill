use std::sync::Arc;

use crate::{
    application::{
        dtos::{requests::CreateDoseRecordRequest, responses::CreateDoseRecordResponse},
        errors::ApplicationError,
        ports::{
            create_dose_record_port::CreateDoseRecordPort,
            dose_record_repository_port::DoseRecordRepository,
        },
    },
    domain::{entities::dose_record::DoseRecord, value_objects::medication_id::MedicationId},
};

pub struct CreateDoseRecordService {
    repository: Arc<dyn DoseRecordRepository>,
}

impl CreateDoseRecordService {
    pub fn new(repository: Arc<dyn DoseRecordRepository>) -> Self {
        Self { repository }
    }
}

impl CreateDoseRecordPort for CreateDoseRecordService {
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
