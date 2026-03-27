use crate::application::{
    dtos::{requests::CreateMedicationRequest, responses::CreateMedicationResponse},
    errors::ApplicationError,
};

pub trait CreateMedicationPort: Send + Sync {
    fn execute(
        &self,
        request: CreateMedicationRequest,
    ) -> Result<CreateMedicationResponse, ApplicationError>;
}
