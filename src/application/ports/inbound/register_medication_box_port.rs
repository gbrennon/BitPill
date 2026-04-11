use crate::application::{
    dtos::{requests::RegisterMedicationBoxRequest, responses::RegisterMedicationBoxResponse},
    errors::ApplicationError,
};

pub trait RegisterMedicationBoxPort: Send + Sync {
    fn execute(
        &self,
        request: RegisterMedicationBoxRequest,
    ) -> Result<RegisterMedicationBoxResponse, ApplicationError>;
}
