use std::sync::Arc;

use uuid::Uuid;

use crate::application::errors::{ApplicationError, NotFoundError};
use crate::application::ports::dose_record_repository::DoseRecordRepository;
use crate::application::ports::mark_dose_taken_port::{
    MarkDoseTakenPort, MarkDoseTakenRequest, MarkDoseTakenResponse,
};
use crate::domain::{
    errors::DomainError,
    value_objects::dose_record_id::DoseRecordId,
};

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
        let uuid = Uuid::parse_str(&request.record_id)
            .map_err(|_| ApplicationError::InvalidInput(
                format!("invalid record id: {}", request.record_id),
            ))?;
        let record_id = DoseRecordId::from_uuid(uuid);

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::errors::StorageError;
    use crate::domain::{
        entities::dose_record::DoseRecord,
        value_objects::medication_id::MedicationId,
    };
    use chrono::NaiveDate;
    use std::sync::Mutex;

    fn make_datetime(h: u32, m: u32) -> chrono::NaiveDateTime {
        NaiveDate::from_ymd_opt(2025, 1, 1)
            .unwrap()
            .and_hms_opt(h, m, 0)
            .unwrap()
    }

    fn make_request(record_id: &DoseRecordId, h: u32, m: u32) -> MarkDoseTakenRequest {
        MarkDoseTakenRequest::new(record_id.to_string(), make_datetime(h, m))
    }

    struct FakeDoseRecordRepository {
        records: Mutex<Vec<DoseRecord>>,
    }

    impl FakeDoseRecordRepository {
        fn new() -> Self {
            Self {
                records: Mutex::new(Vec::new()),
            }
        }

        fn with(record: DoseRecord) -> Self {
            Self {
                records: Mutex::new(vec![record]),
            }
        }
    }

    impl DoseRecordRepository for FakeDoseRecordRepository {
        fn save(&self, record: &DoseRecord) -> Result<(), StorageError> {
            let mut records = self.records.lock().unwrap();
            if let Some(existing) = records.iter_mut().find(|r| r.id() == record.id()) {
                *existing = record.clone();
            } else {
                records.push(record.clone());
            }
            Ok(())
        }

        fn find_by_id(
            &self,
            id: &DoseRecordId,
        ) -> Result<Option<DoseRecord>, StorageError> {
            Ok(self
                .records
                .lock()
                .unwrap()
                .iter()
                .find(|r| r.id() == id)
                .cloned())
        }

        fn find_all_by_medication(
            &self,
            medication_id: &MedicationId,
        ) -> Result<Vec<DoseRecord>, StorageError> {
            Ok(self
                .records
                .lock()
                .unwrap()
                .iter()
                .filter(|r| r.medication_id() == medication_id)
                .cloned()
                .collect())
        }

        fn delete(&self, _id: &DoseRecordId) -> Result<(), StorageError> {
            Ok(())
        }
    }

    #[test]
    fn execute_marks_existing_dose_record_as_taken() {
        let record = DoseRecord::new(MedicationId::create(), make_datetime(8, 0));
        let record_id = record.id().clone();
        let service = MarkDoseTakenService::new(Arc::new(FakeDoseRecordRepository::with(record)));

        let result = service.execute(make_request(&record_id, 8, 5));

        assert!(result.is_ok());
        assert!(!result.unwrap().record_id.is_empty());
    }

    #[test]
    fn execute_with_unknown_id_returns_not_found_error() {
        let service = MarkDoseTakenService::new(Arc::new(FakeDoseRecordRepository::new()));
        let unknown_id = DoseRecordId::create();

        let result = service.execute(make_request(&unknown_id, 8, 5));

        assert!(matches!(result, Err(ApplicationError::NotFound(NotFoundError))));
    }

    #[test]
    fn execute_with_invalid_record_id_returns_invalid_input_error() {
        let service = MarkDoseTakenService::new(Arc::new(FakeDoseRecordRepository::new()));
        let request = MarkDoseTakenRequest::new("not-a-uuid", make_datetime(8, 5));

        let result = service.execute(request);

        assert!(matches!(result, Err(ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn execute_on_already_taken_dose_returns_domain_error() {
        let mut record = DoseRecord::new(MedicationId::create(), make_datetime(8, 0));
        record.mark_taken(make_datetime(8, 5)).unwrap();
        let record_id = record.id().clone();
        let service = MarkDoseTakenService::new(Arc::new(FakeDoseRecordRepository::with(record)));

        let result = service.execute(make_request(&record_id, 8, 10));

        assert!(matches!(
            result,
            Err(ApplicationError::Domain(DomainError::DoseAlreadyTaken))
        ));
    }
}
