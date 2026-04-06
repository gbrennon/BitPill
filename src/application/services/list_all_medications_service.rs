use std::sync::Arc;

use crate::application::{
    dtos::{
        requests::ListAllMedicationsRequest,
        responses::{ListAllMedicationsResponse, MedicationDto},
    },
    errors::ApplicationError,
    ports::{
        list_all_medications_port::ListAllMedicationsPort,
        outbound::{
            dose_record_repository_port::DoseRecordRepository,
            medication_repository_port::MedicationRepository,
        },
    },
};

pub struct ListAllMedicationsService {
    repository: Arc<dyn MedicationRepository>,
    dose_record_repository: Arc<dyn DoseRecordRepository>,
}

impl ListAllMedicationsService {
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

impl ListAllMedicationsPort for ListAllMedicationsService {
    fn execute(
        &self,
        _request: ListAllMedicationsRequest,
    ) -> Result<ListAllMedicationsResponse, ApplicationError> {
        let medications = self.repository.find_all()?;
        let today = chrono::Local::now().date_naive();
        let dtos = medications
            .into_iter()
            .map(|m| {
                let med_id = m.id().clone();
                let scheduled_today = m.scheduled_time().len();
                let all_records = self
                    .dose_record_repository
                    .find_all_by_medication(&med_id)
                    .unwrap_or_default();
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
                MedicationDto {
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
                }
            })
            .collect();
        Ok(ListAllMedicationsResponse { medications: dtos })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::fakes::{FakeDoseRecordRepository, FakeMedicationRepository};

    fn make_service(
        repo: std::sync::Arc<FakeMedicationRepository>,
        dose_repo: std::sync::Arc<FakeDoseRecordRepository>,
    ) -> ListAllMedicationsService {
        ListAllMedicationsService::new(repo, dose_repo)
    }

    #[test]
    fn execute_with_empty_repository_returns_empty_list() {
        let repo = std::sync::Arc::new(FakeMedicationRepository::new());
        let dose_repo = std::sync::Arc::new(FakeDoseRecordRepository::new());
        let service = make_service(repo, dose_repo);

        let res = service.execute(ListAllMedicationsRequest);

        assert!(res.is_ok());
        assert!(res.unwrap().medications.is_empty());
    }
}
