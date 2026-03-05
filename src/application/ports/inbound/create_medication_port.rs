use crate::application::dtos::requests::CreateMedicationRequest;
use crate::application::dtos::responses::CreateMedicationResponse;
use crate::application::errors::ApplicationError;

pub trait CreateMedicationPort: Send + Sync {
    fn execute(
        &self,
        request: CreateMedicationRequest,
    ) -> Result<CreateMedicationResponse, ApplicationError>;
}
