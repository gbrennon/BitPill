use std::sync::Arc;

use uuid::Uuid;

use crate::{
    application::{
        dtos::{requests::DeleteMedicationRequest, responses::DeleteMedicationResponse},
        errors::ApplicationError,
        ports::{
            inbound::delete_medication_port::DeleteMedicationPort,
            outbound::medication_repository_port::MedicationRepository,
        },
    },
    domain::value_objects::medication_id::MedicationId,
};

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
