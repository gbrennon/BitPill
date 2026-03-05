use crate::application::errors::ApplicationError;

pub struct DeleteMedicationRequest {
    pub id: String,
}

pub struct DeleteMedicationResponse {}

pub trait DeleteMedicationPort: Send + Sync {
    fn execute(
        &self,
        request: DeleteMedicationRequest,
    ) -> Result<DeleteMedicationResponse, ApplicationError>;
}
