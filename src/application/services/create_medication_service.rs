use std::sync::Arc;

use thiserror::Error;

use crate::application::ports::medication_repository::{MedicationRepository, RepositoryError};
use crate::domain::{
    entities::medication::Medication,
    errors::DomainError,
    value_objects::{
        dosage::Dosage, medication_name::MedicationName, scheduled_time::ScheduledTime,
    },
};

#[derive(Debug, Error)]
pub enum CreateMedicationError {
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

pub struct CreateMedicationService {
    repository: Arc<dyn MedicationRepository>,
}

impl CreateMedicationService {
    pub fn new(repository: Arc<dyn MedicationRepository>) -> Self {
        Self { repository }
    }

    pub fn execute(
        &self,
        name: impl Into<String>,
        amount_mg: u32,
        scheduled_times: Vec<(u32, u32)>,
    ) -> Result<Medication, CreateMedicationError> {
        let name = MedicationName::new(name)?;
        let dosage = Dosage::new(amount_mg)?;
        let times = scheduled_times
            .into_iter()
            .map(|(h, m)| ScheduledTime::new(h, m))
            .collect::<Result<Vec<_>, _>>()?;
        let medication = Medication::new(name, dosage, times);
        self.repository.save(&medication)?;
        Ok(medication)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::medication_id::MedicationId;
    use std::sync::Mutex;

    struct FakeMedicationRepository {
        medications: Mutex<Vec<Medication>>,
        fail_on_save: bool,
    }

    impl FakeMedicationRepository {
        fn new() -> Self {
            Self {
                medications: Mutex::new(Vec::new()),
                fail_on_save: false,
            }
        }

        fn failing() -> Self {
            Self {
                medications: Mutex::new(Vec::new()),
                fail_on_save: true,
            }
        }

        fn saved_count(&self) -> usize {
            self.medications.lock().unwrap().len()
        }
    }

    impl MedicationRepository for FakeMedicationRepository {
        fn save(&self, medication: &Medication) -> Result<(), RepositoryError> {
            if self.fail_on_save {
                return Err(RepositoryError::StorageError("forced failure".into()));
            }
            self.medications.lock().unwrap().push(medication.clone());
            Ok(())
        }

        fn find_by_id(&self, _id: &MedicationId) -> Result<Option<Medication>, RepositoryError> {
            Ok(None)
        }

        fn find_all(&self) -> Result<Vec<Medication>, RepositoryError> {
            Ok(self.medications.lock().unwrap().clone())
        }

        fn delete(&self, _id: &MedicationId) -> Result<(), RepositoryError> {
            Ok(())
        }
    }

    #[test]
    fn execute_with_valid_inputs_returns_medication() {
        let service = CreateMedicationService::new(Arc::new(FakeMedicationRepository::new()));

        let result = service.execute("Aspirin", 500, vec![(8, 0), (20, 0)]);

        assert!(result.is_ok());
        let med = result.unwrap();
        assert_eq!(med.name().value(), "Aspirin");
        assert_eq!(med.dosage().amount_mg(), 500);
        assert_eq!(med.scheduled_times().len(), 2);
    }

    #[test]
    fn execute_saves_medication_to_repository() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let service = CreateMedicationService::new(repo.clone());

        service.execute("Ibuprofen", 200, vec![(8, 0)]).unwrap();

        assert_eq!(repo.saved_count(), 1);
    }

    #[test]
    fn execute_with_empty_name_returns_domain_error() {
        let service = CreateMedicationService::new(Arc::new(FakeMedicationRepository::new()));

        let result = service.execute("", 500, vec![(8, 0)]);

        assert!(matches!(
            result,
            Err(CreateMedicationError::Domain(
                DomainError::EmptyMedicationName
            ))
        ));
    }

    #[test]
    fn execute_with_zero_dosage_returns_domain_error() {
        let service = CreateMedicationService::new(Arc::new(FakeMedicationRepository::new()));

        let result = service.execute("Aspirin", 0, vec![(8, 0)]);

        assert!(matches!(
            result,
            Err(CreateMedicationError::Domain(DomainError::InvalidDosage))
        ));
    }

    #[test]
    fn execute_with_invalid_scheduled_time_returns_domain_error() {
        let service = CreateMedicationService::new(Arc::new(FakeMedicationRepository::new()));

        let result = service.execute("Aspirin", 500, vec![(25, 0)]);

        assert!(matches!(
            result,
            Err(CreateMedicationError::Domain(
                DomainError::InvalidScheduledTime
            ))
        ));
    }

    #[test]
    fn execute_when_repository_fails_returns_repository_error() {
        let service = CreateMedicationService::new(Arc::new(FakeMedicationRepository::failing()));

        let result = service.execute("Aspirin", 500, vec![(8, 0)]);

        assert!(matches!(result, Err(CreateMedicationError::Repository(_))));
    }
}
