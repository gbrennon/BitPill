use std::sync::Arc;

use crate::{
    application::{
        dtos::{requests::RegisterMedicationBoxRequest, responses::RegisterMedicationBoxResponse},
        errors::{ApplicationError, NotFoundError},
        ports::{
            inbound::register_medication_box_port::RegisterMedicationBoxPort,
            outbound::{MedicationBoxRepositoryPort, MedicationRepository},
        },
    },
    domain::{
        entities::medication_box::MedicationBox,
        value_objects::{
            dosage::Dosage, medication_id::MedicationId, medication_name::MedicationName,
        },
    },
};

pub struct RegisterMedicationBoxService {
    medication_box_repository: Arc<dyn MedicationBoxRepositoryPort>,
    medication_repository: Arc<dyn MedicationRepository>,
}

impl RegisterMedicationBoxService {
    pub fn new(
        medication_box_repository: Arc<dyn MedicationBoxRepositoryPort>,
        medication_repository: Arc<dyn MedicationRepository>,
    ) -> Self {
        Self {
            medication_box_repository,
            medication_repository,
        }
    }
}

impl RegisterMedicationBoxPort for RegisterMedicationBoxService {
    fn execute(
        &self,
        request: RegisterMedicationBoxRequest,
    ) -> Result<RegisterMedicationBoxResponse, ApplicationError> {
        let medication_id =
            MedicationId::from(uuid::Uuid::parse_str(&request.medication_id).map_err(|_| {
                ApplicationError::InvalidInput(format!(
                    "invalid medication id: {}",
                    request.medication_id
                ))
            })?);

        let _medication = self
            .medication_repository
            .find_by_id(&medication_id)?
            .ok_or(NotFoundError)?;

        let name = MedicationName::new(request.name)
            .map_err(|e| ApplicationError::InvalidInput(format!("invalid name: {}", e)))?;

        let dosage = Dosage::new(request.dosage_mg.into())
            .map_err(|e| ApplicationError::InvalidInput(format!("invalid dosage: {}", e)))?;

        let medication_box = MedicationBox::new(medication_id, name, request.pills_per_box, dosage);

        self.medication_box_repository.save(&medication_box)?;

        Ok(RegisterMedicationBoxResponse {
            id: medication_box.id().to_string(),
            medication_id: request.medication_id,
            name: medication_box.name().to_string(),
            pills_per_box: medication_box.pills_per_box(),
            dosage_mg: medication_box.dosage_mg() as u16,
        })
    }
}
