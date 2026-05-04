use std::sync::Arc;

use uuid::Uuid;

use crate::{
    application::{
        dtos::{
            requests::GetMedicationRequest,
            responses::{GetMedicationResponse, MedicationDto},
        },
        errors::{ApplicationError, NotFoundError},
        ports::{
            inbound::get_medication_port::GetMedicationPort,
            outbound::{
                dose_record_repository_port::DoseRecordRepository,
                medication_repository_port::MedicationRepository,
            },
        },
    },
    domain::value_objects::medication_id::MedicationId,
};

pub struct GetMedicationService {
    repository: Arc<dyn MedicationRepository>,
    dose_record_repository: Arc<dyn DoseRecordRepository>,
}

impl GetMedicationService {
    pub fn new(
        repository: Arc<dyn MedicationRepository>,
        dose_record_repository: Arc<dyn DoseRecordRepository>,
    ) -> Self {
        Self {
            repository,
            dose_record_repository,
        }
    }
}

impl GetMedicationPort for GetMedicationService {
    fn execute(
        &self,
        request: GetMedicationRequest,
    ) -> Result<GetMedicationResponse, ApplicationError> {
        let uuid = Uuid::parse_str(&request.id)
            .map_err(|_| ApplicationError::InvalidInput("invalid id".into()))?;
        let id = MedicationId::from(uuid);
        match self.repository.find_by_id(&id)? {
            Some(m) => {
                let med_id = m.id().clone();
                let scheduled_today = m.scheduled_time().len();
                let all_records = self
                    .dose_record_repository
                    .find_all_by_medication(&med_id)
                    .unwrap_or_default();
                let today = chrono::Local::now().date_naive();
                let taken_today = all_records
                    .iter()
                    .filter(|r| {
                        if let Some(taken) = r.taken_at() {
                            taken.date() == today
                        } else {
                            false
                        }
                    })
                    .count();
                Ok(GetMedicationResponse {
                    medication: MedicationDto {
                        id: m.id().to_string(),
                        name: m.name().value().to_string(),
                        amount_mg: m.dosage().amount_mg(),
                        scheduled_time: m
                            .scheduled_time()
                            .iter()
                            .map(|t| (t.hour(), t.minute()))
                            .collect(),
                        dose_frequency: m.dose_frequency().as_str().to_string(),
                        taken_today,
                        scheduled_today,
                    },
                })
            }
            None => Err(ApplicationError::NotFound(NotFoundError)),
        }
    }
}
