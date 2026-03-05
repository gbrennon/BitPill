use crate::application::dtos::requests::DeleteMedicationRequest;
use crate::application::dtos::responses::DeleteMedicationResponse;
use crate::application::errors::ApplicationError;

pub trait DeleteMedicationPort: Send + Sync {
    fn execute(
        &self,
        request: DeleteMedicationRequest,
    ) -> Result<DeleteMedicationResponse, ApplicationError>;
}
