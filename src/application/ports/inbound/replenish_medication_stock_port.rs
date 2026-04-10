use crate::application::{
    dtos::{
        requests::ReplenishMedicationStockRequest, responses::ReplenishMedicationStockResponse,
    },
    errors::ApplicationError,
};

pub trait ReplenishMedicationStockPort {
    fn execute(
        request: ReplenishMedicationStockRequest,
    ) -> Result<ReplenishMedicationStockResponse, ApplicationError>;
}
