use std::sync::Arc;

use uuid::Uuid;

use crate::application::dtos::requests::MarkDoseTakenRequest;
use crate::application::dtos::responses::MarkDoseTakenResponse;
use crate::application::errors::{ApplicationError, NotFoundError};
use crate::application::ports::dose_record_repository_port::DoseRecordRepository;
use crate::application::ports::mark_dose_taken_port::MarkDoseTakenPort;
use crate::domain::value_objects::dose_record_id::DoseRecordId;

pub struct MarkDoseTakenService {
    repository: Arc<dyn DoseRecordRepository>,
}

impl MarkDoseTakenService {
    pub fn new(repository: Arc<dyn DoseRecordRepository>) -> Self {
        Self { repository }
    }
}

impl MarkDoseTakenPort for MarkDoseTakenService {
    fn execute(
        &self,
        request: MarkDoseTakenRequest,
    ) -> Result<MarkDoseTakenResponse, ApplicationError> {
        let uuid = Uuid::parse_str(&request.record_id).map_err(|_| {
            ApplicationError::InvalidInput(format!("invalid record id: {}", request.record_id))
        })?;
        let record_id = DoseRecordId::from(uuid);

        let mut record = self
            .repository
            .find_by_id(&record_id)?
            .ok_or(NotFoundError)?;

        record.mark_taken(request.taken_at)?;
        self.repository.save(&record)?;

        Ok(MarkDoseTakenResponse {
            record_id: record.id().to_string(),
        })
    }
}
