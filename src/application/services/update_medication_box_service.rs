use std::sync::Arc;

use crate::{
    application::{
        dtos::{requests::UpdateMedicationBoxRequest, responses::UpdateMedicationBoxResponse},
        errors::{ApplicationError, NotFoundError},
        ports::{
            inbound::update_medication_box_port::UpdateMedicationBoxPort,
            outbound::medication_box_repository_port::MedicationBoxRepositoryPort,
        },
    },
    domain::{
        entities::medication_box::MedicationBox,
        value_objects::{
            dosage::Dosage, medication_box_id::MedicationBoxId, medication_name::MedicationName,
        },
    },
};

pub struct UpdateMedicationBoxService {
    repository: Arc<dyn MedicationBoxRepositoryPort>,
}

impl UpdateMedicationBoxService {
    pub fn new(repository: Arc<dyn MedicationBoxRepositoryPort>) -> Self {
        Self { repository }
    }
}

impl UpdateMedicationBoxPort for UpdateMedicationBoxService {
    fn execute(
        &self,
        request: UpdateMedicationBoxRequest,
    ) -> Result<UpdateMedicationBoxResponse, ApplicationError> {
        let id =
            MedicationBoxId::from(uuid::Uuid::parse_str(&request.id).map_err(|_| {
                ApplicationError::InvalidInput(format!("invalid id: {}", request.id))
            })?);

        let name = MedicationName::new(request.name)
            .map_err(|e| ApplicationError::InvalidInput(format!("invalid name: {}", e)))?;

        let dosage = Dosage::new(request.dosage_mg.into())
            .map_err(|e| ApplicationError::InvalidInput(format!("invalid dosage: {}", e)))?;

        let existing = self.repository.find_by_id(&id)?.ok_or(NotFoundError)?;

        let updated = MedicationBox::with_id(
            existing.id().clone(),
            existing.medication_id().clone(),
            name,
            request.pills_per_box,
            dosage,
        );

        self.repository.save(&updated)?;

        Ok(UpdateMedicationBoxResponse {
            id: updated.id().to_string(),
            medication_id: updated.medication_id().to_string(),
            name: updated.name().to_string(),
            pills_per_box: updated.pills_per_box(),
            dosage_mg: updated.dosage_mg() as u16,
        })
    }
}
