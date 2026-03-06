use std::sync::Arc;

use uuid::Uuid;

use crate::application::dtos::requests::MarkDoseTakenRequest;
use crate::application::dtos::responses::MarkDoseTakenResponse;
use crate::application::errors::{ApplicationError, NotFoundError};
use crate::application::ports::dose_record_repository_port::DoseRecordRepository;
use crate::application::ports::mark_dose_taken_port::MarkDoseTakenPort;
use crate::domain::entities::dose_record::DoseRecord;
use crate::domain::value_objects::dose_record_id::DoseRecordId;
use crate::domain::value_objects::medication_id::MedicationId;

pub struct MarkDoseTakenService {
    repository: Arc<dyn DoseRecordRepository>,
    medication_repository: Arc<
        dyn crate::application::ports::outbound::medication_repository_port::MedicationRepository,
    >,
}

impl MarkDoseTakenService {
    pub fn new(
        repository: Arc<dyn DoseRecordRepository>,
        medication_repository: Arc<dyn crate::application::ports::outbound::medication_repository_port::MedicationRepository>,
    ) -> Self {
        Self {
            repository,
            medication_repository,
        }
    }
}

impl MarkDoseTakenPort for MarkDoseTakenService {
    fn execute(
        &self,
        request: MarkDoseTakenRequest,
    ) -> Result<MarkDoseTakenResponse, ApplicationError> {
        // Try to interpret the provided id as a DoseRecord id and lookup.
        let uuid = Uuid::parse_str(&request.record_id).map_err(|_| {
            ApplicationError::InvalidInput(format!("invalid id: {}", request.record_id))
        })?;
        let record_id = DoseRecordId::from(uuid);

        match self.repository.find_by_id(&record_id)? {
            Some(mut record) => {
                // existing record found -> mark as taken
                record.mark_taken(request.taken_at)?;
                self.repository.save(&record)?;
                Ok(MarkDoseTakenResponse {
                    record_id: record.id().to_string(),
                })
            }
            None => {
                // No existing dose record with that id; interpret the provided id as a MedicationId
                let med_id = MedicationId::from(uuid);
                // check medication exists
                let maybe_med = self.medication_repository.find_by_id(&med_id)?;
                if maybe_med.is_none() {
                    return Err(ApplicationError::NotFound(NotFoundError));
                }
                let mut record = DoseRecord::new(med_id, request.taken_at);
                // newly created record should be marked as taken at the provided time
                record.mark_taken(request.taken_at)?;
                self.repository.save(&record)?;
                Ok(MarkDoseTakenResponse {
                    record_id: record.id().to_string(),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::fakes::{FakeDoseRecordRepository, FakeMedicationRepository};
    use chrono::NaiveDate;

    fn make_datetime(h: u32, m: u32) -> chrono::NaiveDateTime {
        NaiveDate::from_ymd_opt(2025, 1, 1)
            .unwrap()
            .and_hms_opt(h, m, 0)
            .unwrap()
    }

    fn make_service(
        repo: std::sync::Arc<FakeDoseRecordRepository>,
        med_repo: std::sync::Arc<FakeMedicationRepository>,
    ) -> MarkDoseTakenService {
        MarkDoseTakenService::new(repo, med_repo)
    }

    #[test]
    fn execute_with_invalid_uuid_returns_invalid_input() {
        let repo = std::sync::Arc::new(FakeDoseRecordRepository::new());
        let med_repo = std::sync::Arc::new(FakeMedicationRepository::new());
        let service = make_service(repo, med_repo);

        let req = MarkDoseTakenRequest { record_id: "not-a-uuid".into(), taken_at: make_datetime(8,0) };
        let res = service.execute(req);
        assert!(matches!(res, Err(ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn execute_when_record_exists_marks_and_saves() {
        let med_id = crate::domain::value_objects::medication_id::MedicationId::generate();
        let record = crate::domain::entities::dose_record::DoseRecord::new(med_id.clone(), make_datetime(8,0));
        let repo = std::sync::Arc::new(FakeDoseRecordRepository::with(record.clone()));
        let med_repo = std::sync::Arc::new(FakeMedicationRepository::new());
        let service = make_service(repo.clone(), med_repo);

        let req = MarkDoseTakenRequest { record_id: record.id().to_string(), taken_at: make_datetime(8,5) };
        let res = service.execute(req).unwrap();
        assert_eq!(res.record_id, record.id().to_string());
        assert_eq!(repo.saved_count(), 1);
    }

    #[test]
    fn execute_when_no_record_but_med_exists_creates_and_saves() {
        let med = crate::domain::entities::medication::Medication::new(
            crate::domain::value_objects::medication_id::MedicationId::generate(),
            crate::domain::value_objects::medication_name::MedicationName::new("Test").unwrap(),
            crate::domain::value_objects::dosage::Dosage::new(100).unwrap(),
            vec![],
            crate::domain::value_objects::medication_frequency::DoseFrequency::OnceDaily,
        );
        let med_id = med.id().clone();
        let repo = std::sync::Arc::new(FakeDoseRecordRepository::new());
        let med_repo = std::sync::Arc::new(FakeMedicationRepository::with(vec![med]));
        let service = make_service(repo.clone(), med_repo);

        let req = MarkDoseTakenRequest { record_id: med_id.to_string(), taken_at: make_datetime(9,0) };
        let res = service.execute(req).unwrap();
        assert!(!res.record_id.is_empty());
        assert_eq!(repo.saved_count(), 1);
    }

    #[test]
    fn execute_when_no_record_and_med_missing_returns_not_found() {
        let repo = std::sync::Arc::new(FakeDoseRecordRepository::new());
        let med_repo = std::sync::Arc::new(FakeMedicationRepository::new());
        let service = make_service(repo, med_repo);

        let req = MarkDoseTakenRequest { record_id: uuid::Uuid::now_v7().to_string(), taken_at: make_datetime(9,0) };
        let res = service.execute(req);
        assert!(matches!(res, Err(ApplicationError::NotFound(_))));
    }
}
