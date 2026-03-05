use crate::application::dtos::requests::GetMedicationRequest;
use crate::application::dtos::responses::GetMedicationResponse;
use crate::application::errors::ApplicationError;

pub trait GetMedicationPort: Send + Sync {
    fn execute(
        &self,
        request: GetMedicationRequest,
    ) -> Result<GetMedicationResponse, ApplicationError>;
}
