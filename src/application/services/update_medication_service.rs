use std::{convert::TryFrom, sync::Arc};

use uuid::Uuid;

use crate::{
    application::{
        dtos::{requests::UpdateMedicationRequest, responses::UpdateMedicationResponse},
        errors::ApplicationError,
        ports::{
            inbound::update_medication_port::UpdateMedicationPort,
            outbound::medication_repository_port::MedicationRepository,
        },
    },
    domain::{entities::medication::Medication, value_objects::medication_id::MedicationId},
};

pub struct UpdateMedicationService {
    repository: Arc<dyn MedicationRepository>,
}

impl UpdateMedicationService {
    pub fn new(repository: Arc<dyn MedicationRepository>) -> Self {
        Self { repository }
    }
}

impl UpdateMedicationPort for UpdateMedicationService {
    fn execute(
        &self,
        request: UpdateMedicationRequest,
    ) -> Result<UpdateMedicationResponse, ApplicationError> {
        let id = Uuid::parse_str(&request.id)
            .map_err(|_| ApplicationError::InvalidInput("invalid id".into()))?;
        let id_str = request.id.clone();
        let med_id = MedicationId::from(id);

        let medication = Medication::try_from((request, med_id))?;

        self.repository.save(&medication)?;

        Ok(UpdateMedicationResponse { id: id_str })
    }
}
