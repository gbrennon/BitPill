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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::fakes::FakeDoseRecordRepository;
    use chrono::NaiveDate;

    fn make_service(repo: std::sync::Arc<FakeDoseRecordRepository>) -> CreateDoseRecordService {
        CreateDoseRecordService::new(repo)
    }

    fn make_datetime(h: u32, m: u32) -> chrono::NaiveDateTime {
        NaiveDate::from_ymd_opt(2025, 1, 1)
            .unwrap()
            .and_hms_opt(h, m, 0)
            .unwrap()
    }

    #[test]
    fn execute_with_valid_request_saves_record() {
        let repo = std::sync::Arc::new(FakeDoseRecordRepository::new());
        let service = make_service(repo.clone());
        let med_id = crate::domain::value_objects::medication_id::MedicationId::generate();
        let req = CreateDoseRecordRequest::new(med_id.to_string(), make_datetime(8, 0));

        let res = service.execute(req).unwrap();
        assert!(!res.id.is_empty());
        assert_eq!(repo.saved_count(), 1);
    }

    #[test]
    fn execute_with_invalid_medication_id_returns_invalid_input() {
        let repo = std::sync::Arc::new(FakeDoseRecordRepository::new());
        let service = make_service(repo);
        let req = CreateDoseRecordRequest::new("not-a-uuid", make_datetime(8, 0));

        let res = service.execute(req);
        assert!(matches!(res, Err(ApplicationError::InvalidInput(_))));
    }
}
