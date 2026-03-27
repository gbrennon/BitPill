use crate::application::{
    dtos::{requests::GetMedicationRequest, responses::GetMedicationResponse},
    errors::ApplicationError,
};

pub trait GetMedicationPort: Send + Sync {
    fn execute(
        &self,
        request: GetMedicationRequest,
    ) -> Result<GetMedicationResponse, ApplicationError>;
}
