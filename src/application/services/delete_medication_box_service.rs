use std::sync::Arc;

use crate::{
    application::{
        dtos::{requests::DeleteMedicationBoxRequest, responses::DeleteMedicationBoxResponse},
        errors::{ApplicationError, NotFoundError},
        ports::{
            inbound::delete_medication_box_port::DeleteMedicationBoxPort,
            outbound::medication_box_repository_port::MedicationBoxRepositoryPort,
        },
    },
    domain::value_objects::medication_box_id::MedicationBoxId,
};

pub struct DeleteMedicationBoxService {
    repository: Arc<dyn MedicationBoxRepositoryPort>,
}

impl DeleteMedicationBoxService {
    pub fn new(repository: Arc<dyn MedicationBoxRepositoryPort>) -> Self {
        Self { repository }
    }
}

impl DeleteMedicationBoxPort for DeleteMedicationBoxService {
    fn execute(
        &self,
        request: DeleteMedicationBoxRequest,
    ) -> Result<DeleteMedicationBoxResponse, ApplicationError> {
        let id =
            MedicationBoxId::from(uuid::Uuid::parse_str(&request.id).map_err(|_| {
                ApplicationError::InvalidInput(format!("invalid id: {}", request.id))
            })?);

        let existing = self.repository.find_by_id(&id)?.ok_or(NotFoundError)?;

        self.repository.delete(existing.id())?;

        Ok(DeleteMedicationBoxResponse {})
    }
}
