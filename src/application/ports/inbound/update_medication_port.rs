use crate::application::{
    dtos::{requests::UpdateMedicationRequest, responses::UpdateMedicationResponse},
    errors::ApplicationError,
};

pub trait UpdateMedicationPort: Send + Sync {
    fn execute(
        &self,
        request: UpdateMedicationRequest,
    ) -> Result<UpdateMedicationResponse, ApplicationError>;
}
