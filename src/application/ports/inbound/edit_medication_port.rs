use crate::application::dtos::requests::EditMedicationRequest;
use crate::application::dtos::responses::EditMedicationResponse;
use crate::application::errors::ApplicationError;

pub trait EditMedicationPort: Send + Sync {
    fn execute(
        &self,
        request: EditMedicationRequest,
    ) -> Result<EditMedicationResponse, ApplicationError>;
}
