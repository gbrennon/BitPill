use std::sync::Arc;


use crate::application::errors::ApplicationError;
use crate::application::ports::inbound::create_dose_record_port::{
    CreateDoseRecordPort, CreateDoseRecordRequest, CreateDoseRecordResponse,
};
use crate::application::ports::dose_record_repository_port::DoseRecordRepository;
use crate::domain::value_objects::medication_id::MedicationId;
use crate::domain::entities::dose_record::DoseRecord;

pub struct CreateDoseRecordsService {
    repository: Arc<dyn DoseRecordRepository>,
}

impl CreateDoseRecordsService {
    pub fn new(repository: Arc<dyn DoseRecordRepository>) -> Self {
        Self { repository }
    }
}

impl CreateDoseRecordPort for CreateDoseRecordsService {
    fn execute(&self, request: CreateDoseRecordRequest) -> Result<CreateDoseRecordResponse, ApplicationError> {
        let med_id = MedicationId::from(
            uuid::Uuid::parse_str(&request.medication_id).map_err(|_| ApplicationError::InvalidInput(format!("invalid medication id: {}", request.medication_id)))?
        );
        let record = DoseRecord::new(med_id, request.scheduled_at);
        self.repository.save(&record)?;
        Ok(CreateDoseRecordResponse { id: record.id().to_string() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use chrono::{NaiveDate, NaiveDateTime};
    use crate::application::ports::fakes::FakeDoseRecordRepository;

    #[test]
    fn create_dose_record_saves_to_repository() {
        let repo = Arc::new(FakeDoseRecordRepository::new());
        let service = CreateDoseRecordsService::new(repo.clone());
        let med_id = uuid::Uuid::nil().to_string();
        let scheduled_at = NaiveDate::from_ymd(2020, 1, 1).and_hms(9, 0, 0);
        let req = CreateDoseRecordRequest::new(med_id.clone(), scheduled_at);
        let res = service.execute(req).expect("execute should succeed");
        assert!(!res.id.is_empty());
        // concrete repo still accessible to assert saved count
        assert_eq!(repo.saved_count(), 1);
    }
}
