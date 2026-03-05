use std::sync::Arc;

use crate::application::errors::ApplicationError;
use crate::application::ports::list_all_medications_port::{
    ListAllMedicationsPort, ListAllMedicationsRequest, ListAllMedicationsResponse, MedicationDto,
};
use crate::application::ports::medication_repository_port::MedicationRepository;

pub struct ListAllMedicationsService {
    repository: Arc<dyn MedicationRepository>,
}

impl ListAllMedicationsService {
    pub fn new(repository: Arc<dyn MedicationRepository>) -> Self {
        Self { repository }
    }
}

impl ListAllMedicationsPort for ListAllMedicationsService {
    fn execute(
        &self,
        _request: ListAllMedicationsRequest,
    ) -> Result<ListAllMedicationsResponse, ApplicationError> {
        let medications = self.repository.find_all()?;
        let dtos = medications
            .into_iter()
            .map(|m| MedicationDto {
                id: m.id().to_string(),
                name: m.name().value().to_string(),
                amount_mg: m.dosage().amount_mg(),
                scheduled_time: m
                    .scheduled_time()
                    .iter()
                    .map(|t| (t.hour(), t.minute()))
                    .collect(),
                dose_frequency: m.dose_frequency().to_string(),
            })
            .collect();
        Ok(ListAllMedicationsResponse { medications: dtos })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::fakes::FakeMedicationRepository;
    use crate::domain::{
        entities::medication::Medication,
        value_objects::{
            dosage::Dosage, medication_frequency::DoseFrequency, medication_id::MedicationId,
            medication_name::MedicationName, scheduled_time::ScheduledTime,
        },
    };

    fn make_medication(name: &str, amount_mg: u32) -> Medication {
        Medication::new(
            MedicationId::generate(),
            MedicationName::new(name).unwrap(),
            Dosage::new(amount_mg).unwrap(),
            vec![ScheduledTime::new(8, 0).unwrap()],
            DoseFrequency::OnceDaily,
        )
    }

    #[test]
    fn execute_with_empty_repository_returns_empty_list() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let service = ListAllMedicationsService::new(repo);

        let result = service.execute(ListAllMedicationsRequest);

        assert!(result.is_ok());
        assert!(result.unwrap().medications.is_empty());
    }

    #[test]
    fn execute_returns_all_medications_as_dtos() {
        let repo = Arc::new(FakeMedicationRepository::with(vec![
            make_medication("Aspirin", 500),
            make_medication("Ibuprofen", 200),
        ]));
        let service = ListAllMedicationsService::new(repo);

        let result = service.execute(ListAllMedicationsRequest).unwrap();

        assert_eq!(result.medications.len(), 2);
    }

    #[test]
    fn execute_maps_medication_fields_correctly() {
        let med = make_medication("Paracetamol", 1000);
        let repo = Arc::new(FakeMedicationRepository::with(vec![med]));
        let service = ListAllMedicationsService::new(repo);

        let result = service.execute(ListAllMedicationsRequest).unwrap();
        let dto = &result.medications[0];

        assert_eq!(dto.name, "Paracetamol");
        assert_eq!(dto.amount_mg, 1000);
        assert_eq!(dto.scheduled_time, vec![(8, 0)]);
    }
}
