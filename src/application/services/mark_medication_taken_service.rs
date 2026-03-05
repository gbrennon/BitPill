use std::sync::Arc;

use crate::application::errors::ApplicationError;
use crate::application::ports::dose_record_repository_port::DoseRecordRepository;
use crate::application::ports::inbound::mark_medication_taken_port::{
    MarkMedicationTakenPort, MarkMedicationTakenRequest, MarkMedicationTakenResponse,
};
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
        record.mark_taken(request.taken_at)?;
        self.repository.save(&record)?;

        Ok(MarkMedicationTakenResponse {
            id: record.id().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::fakes::FakeDoseRecordRepository;
    use chrono::{NaiveDate, NaiveDateTime};
    use std::sync::Arc;

    fn make_datetime(h: u32, m: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2025, 1, 1)
            .unwrap()
            .and_hms_opt(h, m, 0)
            .unwrap()
    }

    #[test]
    fn execute_creates_and_saves_record() {
        let repo = Arc::new(FakeDoseRecordRepository::new());
        let service = MarkMedicationTakenService::new(repo.clone());
        let med_id = uuid::Uuid::nil().to_string();
        let req = MarkMedicationTakenRequest::new(med_id.clone(), make_datetime(9, 0));
        let res = service.execute(req).expect("execute should succeed");
        assert!(!res.id.is_empty());
        assert_eq!(repo.saved_count(), 1);
    }

    #[test]
    fn execute_saves_record_as_taken() {
        use crate::domain::value_objects::dose_record_id::DoseRecordId;
        let repo = Arc::new(FakeDoseRecordRepository::new());
        let service = MarkMedicationTakenService::new(repo.clone());
        let med_id = uuid::Uuid::nil().to_string();
        let req = MarkMedicationTakenRequest::new(med_id.clone(), make_datetime(9, 0));

        let res = service.execute(req).expect("execute should succeed");

        let record_id = DoseRecordId::from(uuid::Uuid::parse_str(&res.id).unwrap());
        let saved = repo
            .find_by_id(&record_id)
            .unwrap()
            .expect("record should exist");
        assert!(saved.is_taken());
    }

    #[test]
    fn execute_with_invalid_medication_id_returns_error() {
        let repo = Arc::new(FakeDoseRecordRepository::new());
        let service = MarkMedicationTakenService::new(repo);
        let req = MarkMedicationTakenRequest::new("not-a-uuid", make_datetime(9, 0));
        let res = service.execute(req);
        assert!(matches!(res, Err(ApplicationError::InvalidInput(_))));
    }
}
