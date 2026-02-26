use std::sync::Arc;

use crate::application::errors::ApplicationError;
use crate::application::ports::create_medication_port::{
    CreateMedicationPort,
    CreateMedicationRequest,
    CreateMedicationResponse,
};
use crate::application::ports::medication_repository_port::MedicationRepository;
use crate::domain::{
    entities::medication::Medication,
    value_objects::{
        dosage::Dosage,
        medication_id::MedicationId,
        medication_name::MedicationName,
        scheduled_time::ScheduledTime,
    },
};

pub struct CreateMedicationService {
    repository: Arc<dyn MedicationRepository>,
}

impl CreateMedicationService {
    pub fn new(repository: Arc<dyn MedicationRepository>) -> Self {
        Self { repository }
    }
}

impl CreateMedicationPort for CreateMedicationService {
    fn execute(
        &self,
        request: CreateMedicationRequest,
    ) -> Result<CreateMedicationResponse, ApplicationError> {
        let id = MedicationId::create();
        let name = MedicationName::new(request.name)?;
        let dosage = Dosage::new(request.amount_mg)?;
        let times = request
            .scheduled_times
            .into_iter()
            .map(|(h, m)| ScheduledTime::new(h, m))
            .collect::<Result<Vec<_>, _>>()?;

        let medication = Medication::new(id, name, dosage, times);

        self.repository.save(&medication)?;

        Ok(CreateMedicationResponse {
            id: medication.id().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::errors::StorageError;
    use crate::domain::errors::DomainError;
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
        fn save(&self, medication: &Medication) -> Result<(), StorageError> {
            if self.fail_on_save {
                return Err(StorageError("forced failure".into()));
            }
            self.medications.lock().unwrap().push(medication.clone());
            Ok(())
        }

        fn find_by_id(&self, _id: &MedicationId) -> Result<Option<Medication>, StorageError> {
            Ok(None)
        }

        fn find_all(&self) -> Result<Vec<Medication>, StorageError> {
            Ok(self.medications.lock().unwrap().clone())
        }

        fn delete(&self, _id: &MedicationId) -> Result<(), StorageError> {
            Ok(())
        }
    }

    fn make_request(
        name: &str,
        amount_mg: u32,
        scheduled_times: Vec<(u32, u32)>,
    ) -> CreateMedicationRequest {
        CreateMedicationRequest::new(name, amount_mg, scheduled_times)
    }

    #[test]
    fn execute_with_valid_inputs_returns_response() {
        let service = CreateMedicationService::new(Arc::new(FakeMedicationRepository::new()));

        let result = service.execute(make_request("Levetiracetam", 500, vec![(8, 0), (20, 0)]));

        assert!(result.is_ok());
        assert!(!result.unwrap().id.is_empty());
    }

    #[test]
    fn execute_saves_medication_to_repository() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let service = CreateMedicationService::new(repo.clone());

        service.execute(make_request("Ibuprofen", 200, vec![(8, 0)])).unwrap();

        assert_eq!(repo.saved_count(), 1);
    }

    #[test]
    fn execute_with_empty_name_returns_domain_error() {
        let service = CreateMedicationService::new(Arc::new(FakeMedicationRepository::new()));

        let result = service.execute(make_request("", 500, vec![(8, 0)]));

        assert!(matches!(
            result,
            Err(ApplicationError::Domain(DomainError::EmptyMedicationName))
        ));
    }

    #[test]
    fn execute_with_zero_dosage_returns_domain_error() {
        let service = CreateMedicationService::new(Arc::new(FakeMedicationRepository::new()));

        let result = service.execute(make_request("Levetiracetam", 0, vec![(8, 0)]));

        assert!(matches!(
            result,
            Err(ApplicationError::Domain(DomainError::InvalidDosage))
        ));
    }

    #[test]
    fn execute_with_invalid_scheduled_time_returns_domain_error() {
        let service = CreateMedicationService::new(Arc::new(FakeMedicationRepository::new()));

        let result = service.execute(make_request("Levetiracetam", 500, vec![(25, 0)]));

        assert!(matches!(
            result,
            Err(ApplicationError::Domain(DomainError::InvalidScheduledTime))
        ));
    }

    #[test]
    fn execute_when_repository_fails_returns_storage_error() {
        let service = CreateMedicationService::new(Arc::new(FakeMedicationRepository::failing()));

        let result = service.execute(make_request("Levetiracetam", 500, vec![(8, 0)]));

        assert!(matches!(result, Err(ApplicationError::Storage(_))));
    }
}
