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
