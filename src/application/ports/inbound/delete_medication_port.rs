use crate::application::{
    dtos::{requests::DeleteMedicationRequest, responses::DeleteMedicationResponse},
    errors::ApplicationError,
};

pub trait DeleteMedicationPort: Send + Sync {
    fn execute(
        &self,
        request: DeleteMedicationRequest,
    ) -> Result<DeleteMedicationResponse, ApplicationError>;
}
