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
