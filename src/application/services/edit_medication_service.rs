use std::sync::Arc;

use uuid::Uuid;

use crate::{
    application::{
        dtos::{requests::EditMedicationRequest, responses::EditMedicationResponse},
        errors::ApplicationError,
        ports::{
            inbound::edit_medication_port::EditMedicationPort,
            outbound::medication_repository_port::MedicationRepository,
        },
    },
    domain::{
        entities::medication::Medication,
        value_objects::{
            dosage::Dosage, medication_frequency::DoseFrequency, medication_id::MedicationId,
            medication_name::MedicationName, scheduled_time::ScheduledTime,
        },
    },
};

pub struct EditMedicationService {
    repository: Arc<dyn MedicationRepository>,
}

impl EditMedicationService {
    pub fn new(repository: Arc<dyn MedicationRepository>) -> Self {
        Self { repository }
    }
}

impl EditMedicationPort for EditMedicationService {
    fn execute(
        &self,
        request: EditMedicationRequest,
    ) -> Result<EditMedicationResponse, ApplicationError> {
        let uuid = Uuid::parse_str(&request.id)
            .map_err(|_| ApplicationError::InvalidInput("invalid id".into()))?;
        let id = MedicationId::from(uuid);

        let mut errors = Vec::new();

        let name = match MedicationName::new(request.name) {
            Ok(n) => n,
            Err(e) => {
                errors.push(e);
                return Err(ApplicationError::MultipleDomainErrors { errors });
            }
        };

        let dosage = match Dosage::new(request.amount_mg) {
            Ok(d) => d,
            Err(e) => {
                errors.push(e);
                return Err(ApplicationError::MultipleDomainErrors { errors });
            }
        };

        let scheduled_times: Vec<ScheduledTime> = request
            .scheduled_time
            .into_iter()
            .filter_map(|(h, m)| match ScheduledTime::new(h, m) {
                Ok(st) => Some(st),
                Err(e) => {
                    errors.push(e);
                    None
                }
            })
            .collect();

        if !errors.is_empty() {
            return Err(ApplicationError::MultipleDomainErrors { errors });
        }

        let dose_frequency = match request.dose_frequency.as_str() {
            "TwiceDaily" => DoseFrequency::TwiceDaily,
            "ThriceDaily" => DoseFrequency::ThriceDaily,
            "Custom" => DoseFrequency::Custom(scheduled_times.clone()),
            _ => DoseFrequency::OnceDaily,
        };

        match Medication::with_id(id, name, dosage, scheduled_times, dose_frequency) {
            Ok(medication) => {
                self.repository.save(&medication)?;
                Ok(EditMedicationResponse { id: request.id })
            }
            Err(es) => Err(ApplicationError::MultipleDomainErrors { errors: es }),
        }
    }
}
