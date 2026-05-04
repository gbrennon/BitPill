use std::sync::Arc;

use crate::{
    application::{
        dtos::{requests::GetMedicationBoxRequest, responses::GetMedicationBoxResponse},
        errors::{ApplicationError, NotFoundError},
        ports::{
            inbound::get_medication_box_port::GetMedicationBoxPort,
            outbound::medication_box_repository_port::MedicationBoxRepositoryPort,
        },
    },
    domain::value_objects::medication_box_id::MedicationBoxId,
};

pub struct GetMedicationBoxService {
    repository: Arc<dyn MedicationBoxRepositoryPort>,
}

impl GetMedicationBoxService {
    pub fn new(repository: Arc<dyn MedicationBoxRepositoryPort>) -> Self {
        Self { repository }
    }
}

impl GetMedicationBoxPort for GetMedicationBoxService {
    fn execute(
        &self,
        request: GetMedicationBoxRequest,
    ) -> Result<GetMedicationBoxResponse, ApplicationError> {
        let id =
            MedicationBoxId::from(uuid::Uuid::parse_str(&request.id).map_err(|_| {
                ApplicationError::InvalidInput(format!("invalid id: {}", request.id))
            })?);

        let r#box = self.repository.find_by_id(&id)?.ok_or(NotFoundError)?;

        Ok(GetMedicationBoxResponse {
            id: r#box.id().to_string(),
            medication_id: r#box.medication_id().to_string(),
            name: r#box.name().to_string(),
            pills_per_box: r#box.pills_per_box(),
            dosage_mg: r#box.dosage_mg() as u16,
        })
    }
}
