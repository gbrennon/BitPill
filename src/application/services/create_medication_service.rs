use std::{convert::TryFrom, path::PathBuf};

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
    infrastructure::persistence::json_medication_repository::JsonMedicationRepository,
};

pub struct CreateMedicationService {
    repository: JsonMedicationRepository,
}

impl CreateMedicationService {
    pub fn new() -> Self {
        Self {
            repository: JsonMedicationRepository::new(PathBuf::from("medications.json")),
        }
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
