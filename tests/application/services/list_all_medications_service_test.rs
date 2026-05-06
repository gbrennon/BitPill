use std::sync::Arc;

use bitpill::application::{
    dtos::requests::ListAllMedicationsRequest,
    ports::inbound::list_all_medications_port::ListAllMedicationsPort,
    services::list_all_medications_service::ListAllMedicationsService,
};

use crate::fakes::{FakeDoseRecordRepository, FakeMedicationRepository};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_with_empty_repository_returns_empty_list() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let dose_repo = Arc::new(FakeDoseRecordRepository::new());
        let service = ListAllMedicationsService::new(repo, dose_repo);

        let result = service.execute(ListAllMedicationsRequest);

        assert!(result.is_ok());
        assert!(result.unwrap().medications.is_empty());
    }

    #[test]
    fn execute_returns_all_medications_as_dtos() {
        use bitpill::domain::{
            entities::medication::Medication,
            value_objects::{
                dosage::Dosage, medication_frequency::DoseFrequency, medication_id::MedicationId,
                medication_name::MedicationName, scheduled_time::ScheduledTime,
            },
        };

        let med = Medication::new(
            MedicationId::generate(),
            MedicationName::new("Aspirin").unwrap(),
            Dosage::new(500).unwrap(),
            vec![ScheduledTime::new(8, 0).unwrap()],
            DoseFrequency::OnceDaily,
        )
        .unwrap();
        let repo = Arc::new(FakeMedicationRepository::with(vec![med]));
        let dose_repo = Arc::new(FakeDoseRecordRepository::new());
        let service = ListAllMedicationsService::new(repo, dose_repo);

        let result = service.execute(ListAllMedicationsRequest).unwrap();

        assert_eq!(result.medications.len(), 1);
    }

    #[test]
    fn execute_maps_medication_fields_correctly() {
        use bitpill::domain::{
            entities::medication::Medication,
            value_objects::{
                dosage::Dosage, medication_frequency::DoseFrequency, medication_id::MedicationId,
                medication_name::MedicationName, scheduled_time::ScheduledTime,
            },
        };

        let med = Medication::new(
            MedicationId::generate(),
            MedicationName::new("Paracetamol").unwrap(),
            Dosage::new(1000).unwrap(),
            vec![ScheduledTime::new(8, 0).unwrap()],
            DoseFrequency::OnceDaily,
        )
        .unwrap();
        let repo = Arc::new(FakeMedicationRepository::with(vec![med]));
        let dose_repo = Arc::new(FakeDoseRecordRepository::new());
        let service = ListAllMedicationsService::new(repo, dose_repo);

        let result = service.execute(ListAllMedicationsRequest).unwrap();
        let dto = &result.medications[0];

        assert_eq!(dto.name, "Paracetamol");
        assert_eq!(dto.amount_mg, 1000);
        assert_eq!(dto.scheduled_time, vec![(8, 0)]);
    }

    #[test]
    fn execute_when_repository_fails_returns_storage_error() {
        use bitpill::application::errors::ApplicationError;
        let repo = Arc::new(crate::fakes::FakeMedicationRepository::failing_on_find_all());
        let dose_repo = Arc::new(FakeDoseRecordRepository::new());
        let service = ListAllMedicationsService::new(repo, dose_repo);

        let result = service.execute(ListAllMedicationsRequest);

        assert!(matches!(result, Err(ApplicationError::Storage(_))));
    }
}
