use std::{convert::TryFrom, sync::Arc};

use crate::{
    application::{
        dtos::{requests::CreateMedicationRequest, responses::CreateMedicationResponse},
        errors::ApplicationError,
        ports::{
            create_medication_port::CreateMedicationPort,
            outbound::medication_repository_port::MedicationRepository,
        },
    },
    domain::entities::medication::Medication,
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
        let medication = Medication::try_from(request)?;

        self.repository.save(&medication)?;

        Ok(CreateMedicationResponse {
            id: medication.id().to_string(),
        })
    }
}
