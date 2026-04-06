use std::sync::Arc;

use chrono::Local;
use uuid::Uuid;

use crate::{
    application::{
        dtos::{requests::MarkDoseTakenRequest, responses::MarkDoseTakenResponse},
        errors::{ApplicationError, NotFoundError},
        ports::{
            inbound::mark_dose_taken_port::MarkDoseTakenPort,
            outbound::{DoseRecordRepository, MedicationRepository},
        },
    },
    domain::{
        entities::dose_record::DoseRecord,
        value_objects::{dose_record_id::DoseRecordId, medication_id::MedicationId},
    },
};

pub struct MarkDoseTakenService {
    repository: Arc<dyn DoseRecordRepository>,
    medication_repository: Arc<dyn MedicationRepository>,
}

impl MarkDoseTakenService {
    pub fn new(
        repository: Arc<dyn DoseRecordRepository>,
        medication_repository: Arc<dyn MedicationRepository>,
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
        let uuid = Uuid::parse_str(&request.record_id).map_err(|_| {
            ApplicationError::InvalidInput(format!("invalid id: {}", request.record_id))
        })?;
        let record_id = DoseRecordId::from(uuid);

        match self.repository.find_by_id(&record_id)? {
            Some(mut record) => {
                let now = Local::now().naive_local();
                record.mark_taken(now)?;
                self.repository.save(&record)?;
                Ok(MarkDoseTakenResponse {
                    record_id: record.id().to_string(),
                })
            }
            None => {
                let med_id = MedicationId::from(uuid);
                let maybe_med = self.medication_repository.find_by_id(&med_id)?;
                if maybe_med.is_none() {
                    return Err(ApplicationError::NotFound(NotFoundError));
                }

                // If a scheduled time was provided, check for existing records near that time
                if let Some(scheduled_at) = request.scheduled_at {
                    let all_records = self.repository.find_all_by_medication(&med_id)?;
                    for mut record in all_records {
                        if record.taken_at().is_none() {
                            let diff = (record.scheduled_at() - scheduled_at).num_minutes().abs();
                            if diff <= 15 {
                                record.mark_taken(chrono::Local::now().naive_local())?;
                                self.repository.save(&record)?;
                                return Ok(MarkDoseTakenResponse {
                                    record_id: record.id().to_string(),
                                });
                            }
                        }
                    }
                }

                // No existing record found - create a new one
                let scheduled_at = request
                    .scheduled_at
                    .unwrap_or_else(|| chrono::Local::now().naive_local());
                let mut record = DoseRecord::new(med_id, scheduled_at);
                record.mark_taken(chrono::Local::now().naive_local())?;
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
    use chrono::NaiveDate;
    use uuid::Uuid;

    use super::*;
    use crate::{
        application::{
            errors::StorageError,
            ports::{
                dose_record_repository_port::DoseRecordRepository,
                fakes::{FakeDoseRecordRepository, FakeMedicationRepository},
            },
        },
        domain::{
            entities::medication::Medication,
            value_objects::{Dosage, DoseFrequency, MedicationId, MedicationName},
        },
    };

    fn make_datetime(h: u32, m: u32) -> chrono::NaiveDateTime {
        NaiveDate::from_ymd_opt(2025, 1, 1)
            .unwrap()
            .and_hms_opt(h, m, 0)
            .unwrap()
    }

    fn make_service(
        repo: Arc<FakeDoseRecordRepository>,
        med_repo: Arc<FakeMedicationRepository>,
    ) -> MarkDoseTakenService {
        MarkDoseTakenService::new(repo, med_repo)
    }

    #[test]
    fn execute_with_invalid_uuid_returns_invalid_input() {
        let repo = Arc::new(FakeDoseRecordRepository::new());
        let med_repo = Arc::new(FakeMedicationRepository::new());
        let service = make_service(repo, med_repo);

        let req = MarkDoseTakenRequest {
            record_id: "not-a-uuid".into(),
            scheduled_at: None,
        };
        let res = service.execute(req);
        assert!(matches!(res, Err(ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn execute_when_record_exists_marks_and_saves() {
        let med_id = MedicationId::generate();
        let record = DoseRecord::new(med_id.clone(), make_datetime(8, 0));
        let repo = Arc::new(FakeDoseRecordRepository::with(record.clone()));
        let med_repo = Arc::new(FakeMedicationRepository::new());
        let service = make_service(repo.clone(), med_repo);

        let req = MarkDoseTakenRequest {
            record_id: record.id().to_string(),
            scheduled_at: None,
        };
        let res = service.execute(req).unwrap();
        assert_eq!(res.record_id, record.id().to_string());
    }

    #[test]
    fn execute_when_no_record_but_med_exists_creates_and_saves() {
        use crate::domain::value_objects::ScheduledTime;
        let med = Medication::new(
            MedicationId::generate(),
            MedicationName::new("Test").unwrap(),
            Dosage::new(100).unwrap(),
            vec![ScheduledTime::new(8, 0).unwrap()],
            DoseFrequency::OnceDaily,
        )
        .unwrap();
        let med_id = med.id().clone();
        let repo = Arc::new(FakeDoseRecordRepository::new());
        let med_repo = Arc::new(FakeMedicationRepository::with(vec![med]));
        let service = make_service(repo.clone(), med_repo);

        let req = MarkDoseTakenRequest {
            record_id: med_id.to_string(),
            scheduled_at: None,
        };
        let res = service.execute(req).unwrap();
        assert!(!res.record_id.is_empty());
        assert_eq!(repo.saved_count(), 1);
    }

    #[test]
    fn execute_when_no_record_and_med_missing_returns_not_found() {
        let repo = Arc::new(FakeDoseRecordRepository::new());
        let med_repo = Arc::new(FakeMedicationRepository::new());
        let service = make_service(repo, med_repo);

        let req = MarkDoseTakenRequest {
            record_id: Uuid::now_v7().to_string(),
            scheduled_at: None,
        };
        let res = service.execute(req);
        assert!(matches!(res, Err(ApplicationError::NotFound(_))));
    }

    #[test]
    fn execute_when_find_by_id_fails_returns_storage_error() {
        let med_id = MedicationId::generate();
        let record = DoseRecord::new(med_id.clone(), make_datetime(8, 0));
        let record_id = record.id().clone();
        let repo = Arc::new(
            FakeDoseRecordRepository::with(record)
                .with_find_by_id_error(StorageError("db error".to_string())),
        );
        let med_repo = Arc::new(FakeMedicationRepository::new());
        let service = make_service(repo.clone(), med_repo);

        let req = MarkDoseTakenRequest {
            record_id: record_id.to_string(),
            scheduled_at: None,
        };
        let res = service.execute(req);
        assert!(matches!(res, Err(ApplicationError::Storage(_))));
    }

    #[test]
    fn execute_when_repository_save_fails_returns_storage_error() {
        struct FailingSaveRepository {
            records: std::sync::Mutex<Vec<DoseRecord>>,
        }

        impl DoseRecordRepository for FailingSaveRepository {
            fn save(&self, _record: &DoseRecord) -> Result<(), StorageError> {
                Err(StorageError("save failed".to_string()))
            }
            fn find_by_id(&self, id: &DoseRecordId) -> Result<Option<DoseRecord>, StorageError> {
                let records = self.records.lock().unwrap();
                Ok(records.iter().find(|r| r.id() == id).cloned())
            }
            fn find_all_by_medication(
                &self,
                _id: &MedicationId,
            ) -> Result<Vec<DoseRecord>, StorageError> {
                Ok(vec![])
            }
            fn delete(&self, _id: &DoseRecordId) -> Result<(), StorageError> {
                Ok(())
            }
        }

        let med_id = MedicationId::generate();
        let record = DoseRecord::new(med_id.clone(), make_datetime(8, 0));
        let record_id = record.id().clone();
        let repo: Arc<FailingSaveRepository> = Arc::new(FailingSaveRepository {
            records: std::sync::Mutex::new(vec![record]),
        });
        let med_repo = Arc::new(FakeMedicationRepository::new());
        let service = MarkDoseTakenService::new(repo, med_repo);

        let req = MarkDoseTakenRequest {
            record_id: record_id.to_string(),
            scheduled_at: None,
        };
        let res = service.execute(req);
        assert!(matches!(res, Err(ApplicationError::Storage(_))));
    }
}
