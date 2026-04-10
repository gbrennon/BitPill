use std::sync::Arc;

use chrono::Local;

use crate::{
    application::{
        dtos::{
            requests::ReplenishMedicationStockRequest, responses::ReplenishMedicationStockResponse,
        },
        errors::{ApplicationError, NotFoundError},
        ports::{
            inbound::replenish_medication_stock_port::ReplenishMedicationStockPort,
            outbound::{
                MedicationRefillRepositoryPort, MedicationRepository, MedicationStockRepositoryPort,
            },
        },
    },
    domain::{
        entities::{medication_refill::MedicationRefill, medication_stock::MedicationStock},
        value_objects::{dosage::Dosage, medication_id::MedicationId},
    },
};

pub struct ReplenishMedicationStockService {
    medication_stock_repository: Arc<dyn MedicationStockRepositoryPort>,
    medication_refill_repository: Arc<dyn MedicationRefillRepositoryPort>,
    medication_repository: Arc<dyn MedicationRepository>,
}

impl ReplenishMedicationStockService {
    pub fn new(
        medication_stock_repository: Arc<dyn MedicationStockRepositoryPort>,
        medication_refill_repository: Arc<dyn MedicationRefillRepositoryPort>,
        medication_repository: Arc<dyn MedicationRepository>,
    ) -> Self {
        Self {
            medication_stock_repository,
            medication_refill_repository,
            medication_repository,
        }
    }
}

impl ReplenishMedicationStockPort for ReplenishMedicationStockService {
    fn execute(
        &self,
        request: ReplenishMedicationStockRequest,
    ) -> Result<ReplenishMedicationStockResponse, ApplicationError> {
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
        let current_stock = self
            .medication_stock_repository
            .find_by_medication_id(&medication_id)?;

        let new_total = if let Some(stock) = current_stock {
            stock.replenish(request.box_count * request.pills_per_box)
        } else {
            MedicationStock::new(medication_id, request.box_count * request.pills_per_box)
        };

        self.medication_stock_repository.save(&new_total)?;

        let pill_dosage = Dosage::new(request.pill_dosage_mg.into())
            .map_err(|e| ApplicationError::InvalidInput(format!("invalid pill dosage: {}", e)))?;

        let refill = MedicationRefill::new(
            new_total.medication_id().clone(),
            pill_dosage,
            request.pills_per_box,
            request.box_count,
            Local::now().naive_local(),
        );
        self.medication_refill_repository.save(&refill)?;

        Ok(ReplenishMedicationStockResponse {
            medication_id: request.medication_id,
            total_pills: u32::from(new_total.quantity().amount()),
            pill_dosage_mg: request.pill_dosage_mg,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::{
        application::{
            dtos::requests::ReplenishMedicationStockRequest,
            ports::fakes::{
                FakeMedicationRefillRepository, FakeMedicationRepository,
                FakeMedicationStockRepository,
            },
        },
        domain::{
            entities::medication::Medication, value_objects::medication_frequency::DoseFrequency,
        },
    };

    fn make_valid_medication_id() -> String {
        "018f8a2e-0000-0000-0000-000000000001".to_string()
    }

    fn make_request(
        medication_id: &str,
        box_count: u16,
        pills_per_box: u16,
        pill_dosage_mg: u16,
    ) -> ReplenishMedicationStockRequest {
        ReplenishMedicationStockRequest {
            medication_id: medication_id.to_string(),
            box_count,
            pills_per_box,
            pill_dosage_mg,
        }
    }

    fn make_service(
        medication_repository: Arc<FakeMedicationRepository>,
    ) -> ReplenishMedicationStockService {
        ReplenishMedicationStockService::new(
            Arc::new(FakeMedicationStockRepository::new()),
            Arc::new(FakeMedicationRefillRepository::new()),
            medication_repository,
        )
    }

    #[test]
    fn execute_with_valid_inputs_returns_response() {
        use crate::domain::value_objects::{
            medication_name::MedicationName, scheduled_time::ScheduledTime,
        };

        let medication_id = make_valid_medication_id();
        let med_id = MedicationId::from(uuid::Uuid::parse_str(&medication_id).unwrap());
        let mut med_repo = FakeMedicationRepository::new();
        med_repo.set_find_by_id_result(Some(
            Medication::new(
                med_id,
                MedicationName::new("TestMed").unwrap(),
                Dosage::new(500).unwrap(),
                vec![ScheduledTime::new(8, 0).unwrap()],
                DoseFrequency::OnceDaily,
            )
            .unwrap(),
        ));

        let service = make_service(Arc::new(med_repo));
        let result = service.execute(make_request(&medication_id, 2, 30, 500));

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.medication_id, medication_id);
        assert_eq!(response.total_pills, 60);
        assert_eq!(response.pill_dosage_mg, 500);
    }

    #[test]
    fn execute_with_invalid_medication_id_returns_error() {
        let service = make_service(Arc::new(FakeMedicationRepository::new()));
        let result = service.execute(make_request("not-a-valid-uuid", 2, 30, 500));
        assert!(matches!(result, Err(ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn execute_when_medication_not_found_returns_not_found_error() {
        let medication_id = make_valid_medication_id();
        let service = make_service(Arc::new(FakeMedicationRepository::new()));
        let result = service.execute(make_request(&medication_id, 2, 30, 500));
        assert!(matches!(result, Err(ApplicationError::NotFound(_))));
    }

    #[test]
    fn execute_with_invalid_pill_dosage_returns_error() {
        use crate::domain::value_objects::{
            medication_name::MedicationName, scheduled_time::ScheduledTime,
        };

        let medication_id = make_valid_medication_id();
        let med_id = MedicationId::from(uuid::Uuid::parse_str(&medication_id).unwrap());
        let mut med_repo = FakeMedicationRepository::new();
        med_repo.set_find_by_id_result(Some(
            Medication::new(
                med_id,
                MedicationName::new("TestMed").unwrap(),
                Dosage::new(500).unwrap(),
                vec![ScheduledTime::new(8, 0).unwrap()],
                DoseFrequency::OnceDaily,
            )
            .unwrap(),
        ));

        let service = make_service(Arc::new(med_repo));
        let result = service.execute(make_request(&medication_id, 2, 30, 0));
        assert!(matches!(result, Err(ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn execute_saves_stock_to_repository() {
        use crate::domain::value_objects::{
            medication_name::MedicationName, scheduled_time::ScheduledTime,
        };

        let medication_id = make_valid_medication_id();
        let med_id = MedicationId::from(uuid::Uuid::parse_str(&medication_id).unwrap());
        let mut med_repo = FakeMedicationRepository::new();
        med_repo.set_find_by_id_result(Some(
            Medication::new(
                med_id.clone(),
                MedicationName::new("TestMed").unwrap(),
                Dosage::new(500).unwrap(),
                vec![ScheduledTime::new(8, 0).unwrap()],
                DoseFrequency::OnceDaily,
            )
            .unwrap(),
        ));

        let stock_repo = Arc::new(FakeMedicationStockRepository::new());
        let service = ReplenishMedicationStockService::new(
            stock_repo.clone(),
            Arc::new(FakeMedicationRefillRepository::new()),
            Arc::new(med_repo),
        );

        service
            .execute(make_request(&medication_id, 2, 30, 500))
            .unwrap();

        assert_eq!(stock_repo.saved_count(), 1);
    }

    #[test]
    fn execute_saves_refill_to_repository() {
        use crate::domain::value_objects::{
            medication_name::MedicationName, scheduled_time::ScheduledTime,
        };

        let medication_id = make_valid_medication_id();
        let med_id = MedicationId::from(uuid::Uuid::parse_str(&medication_id).unwrap());
        let mut med_repo = FakeMedicationRepository::new();
        med_repo.set_find_by_id_result(Some(
            Medication::new(
                med_id,
                MedicationName::new("TestMed").unwrap(),
                Dosage::new(500).unwrap(),
                vec![ScheduledTime::new(8, 0).unwrap()],
                DoseFrequency::OnceDaily,
            )
            .unwrap(),
        ));

        let refill_repo = Arc::new(FakeMedicationRefillRepository::new());
        let service = ReplenishMedicationStockService::new(
            Arc::new(FakeMedicationStockRepository::new()),
            refill_repo.clone(),
            Arc::new(med_repo),
        );

        service
            .execute(make_request(&medication_id, 2, 30, 500))
            .unwrap();

        assert_eq!(refill_repo.saved_count(), 1);
    }

    #[test]
    fn execute_when_stock_save_fails_returns_storage_error() {
        use crate::domain::value_objects::{
            medication_name::MedicationName, scheduled_time::ScheduledTime,
        };

        let medication_id = make_valid_medication_id();
        let med_id = MedicationId::from(uuid::Uuid::parse_str(&medication_id).unwrap());
        let mut med_repo = FakeMedicationRepository::new();
        med_repo.set_find_by_id_result(Some(
            Medication::new(
                med_id,
                MedicationName::new("TestMed").unwrap(),
                Dosage::new(500).unwrap(),
                vec![ScheduledTime::new(8, 0).unwrap()],
                DoseFrequency::OnceDaily,
            )
            .unwrap(),
        ));

        let service = ReplenishMedicationStockService::new(
            Arc::new(FakeMedicationStockRepository::failing()),
            Arc::new(FakeMedicationRefillRepository::new()),
            Arc::new(med_repo),
        );

        let result = service.execute(make_request(&medication_id, 2, 30, 500));
        assert!(matches!(result, Err(ApplicationError::Storage(_))));
    }

    #[test]
    fn execute_when_refill_save_fails_returns_storage_error() {
        use crate::domain::value_objects::{
            medication_name::MedicationName, scheduled_time::ScheduledTime,
        };

        let medication_id = make_valid_medication_id();
        let med_id = MedicationId::from(uuid::Uuid::parse_str(&medication_id).unwrap());
        let mut med_repo = FakeMedicationRepository::new();
        med_repo.set_find_by_id_result(Some(
            Medication::new(
                med_id,
                MedicationName::new("TestMed").unwrap(),
                Dosage::new(500).unwrap(),
                vec![ScheduledTime::new(8, 0).unwrap()],
                DoseFrequency::OnceDaily,
            )
            .unwrap(),
        ));

        let service = ReplenishMedicationStockService::new(
            Arc::new(FakeMedicationStockRepository::new()),
            Arc::new(FakeMedicationRefillRepository::failing()),
            Arc::new(med_repo),
        );

        let result = service.execute(make_request(&medication_id, 2, 30, 500));
        assert!(matches!(result, Err(ApplicationError::Storage(_))));
    }

    #[test]
    fn execute_with_existing_stock_replenishes_adds_pills() {
        use crate::domain::value_objects::{
            medication_name::MedicationName, scheduled_time::ScheduledTime,
        };

        let medication_id = make_valid_medication_id();
        let med_id = MedicationId::from(uuid::Uuid::parse_str(&medication_id).unwrap());
        let existing_stock = MedicationStock::new(med_id.clone(), 50);

        let mut med_repo = FakeMedicationRepository::new();
        med_repo.set_find_by_id_result(Some(
            Medication::new(
                med_id,
                MedicationName::new("TestMed").unwrap(),
                Dosage::new(500).unwrap(),
                vec![ScheduledTime::new(8, 0).unwrap()],
                DoseFrequency::OnceDaily,
            )
            .unwrap(),
        ));

        let stock_repo = Arc::new(FakeMedicationStockRepository::with(existing_stock));
        let service = ReplenishMedicationStockService::new(
            stock_repo.clone(),
            Arc::new(FakeMedicationRefillRepository::new()),
            Arc::new(med_repo),
        );

        let result = service.execute(make_request(&medication_id, 2, 30, 500));

        assert!(result.is_ok());
        assert_eq!(result.unwrap().total_pills, 110);
    }
}
