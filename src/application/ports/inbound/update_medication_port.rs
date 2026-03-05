use crate::application::dtos::requests::UpdateMedicationRequest;
use crate::application::dtos::responses::UpdateMedicationResponse;
use crate::application::errors::ApplicationError;

pub trait UpdateMedicationPort: Send + Sync {
    fn execute(
        &self,
        request: UpdateMedicationRequest,
    ) -> Result<UpdateMedicationResponse, ApplicationError>;
}
