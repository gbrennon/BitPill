use crate::application::{
    dtos::{
        requests::ReplenishMedicationStockRequest, responses::ReplenishMedicationStockResponse,
    },
    errors::ApplicationError,
};

pub trait ReplenishMedicationStockPort: Send + Sync {
    fn execute(
        &self,
        request: ReplenishMedicationStockRequest,
    ) -> Result<ReplenishMedicationStockResponse, ApplicationError>;
}
