use std::sync::Arc;

use bitpill::{
    application::{
        dtos::requests::ReplenishMedicationStockRequest, errors::ApplicationError,
        ports::inbound::replenish_medication_stock_port::ReplenishMedicationStockPort,
        services::replenish_medication_stock_service::ReplenishMedicationStockService,
    },
    domain::{
        entities::medication::Medication,
        value_objects::{
            dosage::Dosage, medication_frequency::DoseFrequency, medication_id::MedicationId,
            medication_name::MedicationName, scheduled_time::ScheduledTime,
        },
    },
};

use crate::fakes::{
    FakeMedicationRefillRepository, FakeMedicationRepository, FakeMedicationStockRepository,
};

#[cfg(test)]
mod tests {
    use super::*;

    fn make_medication_repo_with_med(medication_id: MedicationId) -> FakeMedicationRepository {
        let mut repo = FakeMedicationRepository::new();
        repo.set_find_by_id_result(Some(
            Medication::new(
                medication_id,
                MedicationName::new("TestMed").unwrap(),
                Dosage::new(500).unwrap(),
                vec![ScheduledTime::new(8, 0).unwrap()],
                DoseFrequency::OnceDaily,
            )
            .unwrap(),
        ));
        repo
    }

    #[test]
    fn execute_with_valid_inputs_returns_response() {
        let med_id = MedicationId::generate();
        let med_id_str = med_id.to_string();
        let med_repo = Arc::new(make_medication_repo_with_med(med_id.clone()));
        let stock_repo = Arc::new(FakeMedicationStockRepository::new());
        let refill_repo = Arc::new(FakeMedicationRefillRepository::new());
        let service =
            ReplenishMedicationStockService::new(stock_repo, refill_repo.clone(), med_repo);

        let result = service.execute(ReplenishMedicationStockRequest {
            medication_id: med_id_str.clone(),
            box_count: 2,
            pills_per_box: 30,
            pill_dosage_mg: 500,
        });

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.medication_id, med_id_str);
        assert_eq!(response.total_pills, 60);
        assert_eq!(response.pill_dosage_mg, 500);
        assert_eq!(refill_repo.saved_count(), 1);
    }

    #[test]
    fn execute_with_invalid_medication_id_returns_invalid_input_error() {
        let med_repo = Arc::new(FakeMedicationRepository::new());
        let stock_repo = Arc::new(FakeMedicationStockRepository::new());
        let refill_repo = Arc::new(FakeMedicationRefillRepository::new());
        let service = ReplenishMedicationStockService::new(stock_repo, refill_repo, med_repo);

        let result = service.execute(ReplenishMedicationStockRequest {
            medication_id: "not-a-uuid".to_string(),
            box_count: 2,
            pills_per_box: 30,
            pill_dosage_mg: 500,
        });

        assert!(matches!(result, Err(ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn execute_when_medication_not_found_returns_not_found_error() {
        let med_id = MedicationId::generate();
        let med_repo = Arc::new(FakeMedicationRepository::new());
        let stock_repo = Arc::new(FakeMedicationStockRepository::new());
        let refill_repo = Arc::new(FakeMedicationRefillRepository::new());
        let service = ReplenishMedicationStockService::new(stock_repo, refill_repo, med_repo);

        let result = service.execute(ReplenishMedicationStockRequest {
            medication_id: med_id.to_string(),
            box_count: 2,
            pills_per_box: 30,
            pill_dosage_mg: 500,
        });

        assert!(matches!(result, Err(ApplicationError::NotFound(_))));
    }

    #[test]
    fn execute_with_invalid_pill_dosage_returns_invalid_input_error() {
        let med_id = MedicationId::generate();
        let med_repo = Arc::new(make_medication_repo_with_med(med_id.clone()));
        let stock_repo = Arc::new(FakeMedicationStockRepository::new());
        let refill_repo = Arc::new(FakeMedicationRefillRepository::new());
        let service = ReplenishMedicationStockService::new(stock_repo, refill_repo, med_repo);

        let result = service.execute(ReplenishMedicationStockRequest {
            medication_id: med_id.to_string(),
            box_count: 2,
            pills_per_box: 30,
            pill_dosage_mg: 0,
        });

        assert!(matches!(result, Err(ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn execute_when_stock_repository_fails_returns_storage_error() {
        let med_id = MedicationId::generate();
        let med_repo = Arc::new(make_medication_repo_with_med(med_id.clone()));
        let stock_repo = Arc::new(FakeMedicationStockRepository::failing());
        let refill_repo = Arc::new(FakeMedicationRefillRepository::new());
        let service = ReplenishMedicationStockService::new(stock_repo, refill_repo, med_repo);

        let result = service.execute(ReplenishMedicationStockRequest {
            medication_id: med_id.to_string(),
            box_count: 2,
            pills_per_box: 30,
            pill_dosage_mg: 500,
        });

        assert!(matches!(result, Err(ApplicationError::Storage(_))));
    }

    #[test]
    fn execute_saves_stock_and_refill_to_repositories() {
        let med_id = MedicationId::generate();
        let med_id_str = med_id.to_string();
        let med_repo = Arc::new(make_medication_repo_with_med(med_id.clone()));
        let stock_repo = Arc::new(FakeMedicationStockRepository::new());
        let refill_repo = Arc::new(FakeMedicationRefillRepository::new());
        let service =
            ReplenishMedicationStockService::new(stock_repo.clone(), refill_repo.clone(), med_repo);

        service
            .execute(ReplenishMedicationStockRequest {
                medication_id: med_id_str,
                box_count: 2,
                pills_per_box: 30,
                pill_dosage_mg: 500,
            })
            .unwrap();

        assert_eq!(stock_repo.saved_count(), 1);
        assert_eq!(refill_repo.saved_count(), 1);
    }
}
