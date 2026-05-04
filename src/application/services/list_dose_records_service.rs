use std::sync::Arc;

use crate::{
    application::{
        dtos::{
            requests::ListDoseRecordsRequest,
            responses::{DoseRecordDto, ListDoseRecordsResponse},
        },
        errors::ApplicationError,
        ports::{
            inbound::list_dose_records_port::ListDoseRecordsPort,
            outbound::dose_record_repository_port::DoseRecordRepository,
        },
    },
    domain::value_objects::medication_id::MedicationId,
};

pub struct ListDoseRecordsService {
    repository: Arc<dyn DoseRecordRepository>,
}

impl ListDoseRecordsService {
    pub fn new(repository: Arc<dyn DoseRecordRepository>) -> Self {
        Self { repository }
    }
}

impl ListDoseRecordsPort for ListDoseRecordsService {
    fn execute(
        &self,
        request: ListDoseRecordsRequest,
    ) -> Result<ListDoseRecordsResponse, ApplicationError> {
        let medication_id =
            MedicationId::from(uuid::Uuid::parse_str(&request.medication_id).map_err(|_| {
                ApplicationError::InvalidInput(format!(
                    "invalid medication id: {}",
                    request.medication_id
                ))
            })?);
        let records = self.repository.find_all_by_medication(&medication_id)?;
        let dtos = records
            .into_iter()
            .rev()
            .map(|r| DoseRecordDto {
                id: r.id().to_string(),
                medication_id: r.medication_id().to_string(),
                scheduled_at: r.scheduled_at(),
                taken_at: r.taken_at(),
            })
            .collect();
        Ok(ListDoseRecordsResponse { records: dtos })
    }
}
