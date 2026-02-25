use std::sync::Arc;

use chrono::NaiveDateTime;
use thiserror::Error;

use crate::application::ports::dose_record_repository::{
    DoseRecordRepository, DoseRecordRepositoryError,
};
use crate::domain::{
    entities::dose_record::DoseRecord, errors::DomainError,
    value_objects::dose_record_id::DoseRecordId,
};

#[derive(Debug, Error)]
pub enum MarkDoseTakenError {
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error(transparent)]
    Repository(#[from] DoseRecordRepositoryError),
    #[error("dose record not found")]
    NotFound,
}

pub struct MarkDoseTakenService {
    repository: Arc<dyn DoseRecordRepository>,
}

impl MarkDoseTakenService {
    pub fn new(repository: Arc<dyn DoseRecordRepository>) -> Self {
        Self { repository }
    }

    pub fn execute(
        &self,
        record_id: &DoseRecordId,
        taken_at: NaiveDateTime,
    ) -> Result<DoseRecord, MarkDoseTakenError> {
        let mut record = self
            .repository
            .find_by_id(record_id)?
            .ok_or(MarkDoseTakenError::NotFound)?;
        record.mark_taken(taken_at)?;
        self.repository.save(&record)?;
        Ok(record)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::medication_id::MedicationId;
    use chrono::NaiveDate;
    use std::sync::Mutex;

    fn make_datetime(h: u32, m: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2025, 1, 1)
            .unwrap()
            .and_hms_opt(h, m, 0)
            .unwrap()
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
        fn save(&self, record: &DoseRecord) -> Result<(), DoseRecordRepositoryError> {
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
        ) -> Result<Option<DoseRecord>, DoseRecordRepositoryError> {
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
        ) -> Result<Vec<DoseRecord>, DoseRecordRepositoryError> {
            Ok(self
                .records
                .lock()
                .unwrap()
                .iter()
                .filter(|r| r.medication_id() == medication_id)
                .cloned()
                .collect())
        }

        fn delete(&self, _id: &DoseRecordId) -> Result<(), DoseRecordRepositoryError> {
            Ok(())
        }
    }

    #[test]
    fn execute_marks_existing_dose_record_as_taken() {
        let record = DoseRecord::new(MedicationId::new(), make_datetime(8, 0));
        let record_id = record.id().clone();
        let service = MarkDoseTakenService::new(Arc::new(FakeDoseRecordRepository::with(record)));

        let result = service.execute(&record_id, make_datetime(8, 5));

        assert!(result.is_ok());
        assert!(result.unwrap().is_taken());
    }

    #[test]
    fn execute_with_unknown_id_returns_not_found_error() {
        let service = MarkDoseTakenService::new(Arc::new(FakeDoseRecordRepository::new()));
        let unknown_id = DoseRecordId::new();

        let result = service.execute(&unknown_id, make_datetime(8, 5));

        assert!(matches!(result, Err(MarkDoseTakenError::NotFound)));
    }

    #[test]
    fn execute_on_already_taken_dose_returns_domain_error() {
        let mut record = DoseRecord::new(MedicationId::new(), make_datetime(8, 0));
        record.mark_taken(make_datetime(8, 5)).unwrap();
        let record_id = record.id().clone();
        let service = MarkDoseTakenService::new(Arc::new(FakeDoseRecordRepository::with(record)));

        let result = service.execute(&record_id, make_datetime(8, 10));

        assert!(matches!(
            result,
            Err(MarkDoseTakenError::Domain(DomainError::DoseAlreadyTaken))
        ));
    }
}
