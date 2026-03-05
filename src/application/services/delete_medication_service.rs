use std::sync::Arc;
use uuid::Uuid;

use crate::application::errors::ApplicationError;
use crate::application::ports::inbound::delete_medication_port::{
    DeleteMedicationPort, DeleteMedicationRequest, DeleteMedicationResponse,
};
use crate::application::ports::outbound::medication_repository_port::MedicationRepository;
use crate::domain::value_objects::medication_id::MedicationId;

pub struct DeleteMedicationService {
    repository: Arc<dyn MedicationRepository>,
}

impl DeleteMedicationService {
    pub fn new(repository: Arc<dyn MedicationRepository>) -> Self {
        Self { repository }
    }
}

impl DeleteMedicationPort for DeleteMedicationService {
    fn execute(
        &self,
        request: DeleteMedicationRequest,
    ) -> Result<DeleteMedicationResponse, ApplicationError> {
        let uuid = Uuid::parse_str(&request.id)
            .map_err(|_| ApplicationError::InvalidInput("invalid id".into()))?;
        let id = MedicationId::from(uuid);
        self.repository.delete(&id)?;
        Ok(DeleteMedicationResponse {})
    }
}

// Unit tests for DeleteMedicationService placed in same file as impl
#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::fakes::FakeMedicationRepository;
    use crate::domain::entities::medication::Medication as DomainMedication;
    use crate::domain::value_objects::{medication_name::MedicationName, dosage::Dosage, scheduled_time::ScheduledTime, medication_frequency::DoseFrequency};

    #[test]
    fn delete_medication_success_removes_from_repo() {
        let med = DomainMedication::new(
            MedicationId::generate(),
            MedicationName::new("DelMed").unwrap(),
            Dosage::new(10).unwrap(),
            vec![ScheduledTime::new(8, 0).unwrap()],
            DoseFrequency::OnceDaily,
        );
        let repo = std::sync::Arc::new(FakeMedicationRepository::with(vec![med.clone()]));
        let svc = DeleteMedicationService::new(repo.clone());
        let req = DeleteMedicationRequest { id: med.id().to_string() };
        let _res = svc.execute(req).expect("should delete");
        // ensure it's removed
        let found = repo.find_by_id(&med.id()).unwrap();
        assert!(found.is_none());
    }

    #[test]
    fn delete_medication_invalid_id_returns_error() {
        let repo = std::sync::Arc::new(FakeMedicationRepository::new());
        let svc = DeleteMedicationService::new(repo);
        let req = DeleteMedicationRequest { id: "not-a-uuid".into() };
        let res = svc.execute(req);
        assert!(matches!(res, Err(crate::application::errors::ApplicationError::InvalidInput(_))));
    }
}
