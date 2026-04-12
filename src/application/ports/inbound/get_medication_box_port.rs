use crate::application::{
    dtos::{requests::GetMedicationBoxRequest, responses::GetMedicationBoxResponse},
    errors::ApplicationError,
};

pub trait GetMedicationBoxPort: Send + Sync {
    fn execute(
        &self,
        request: GetMedicationBoxRequest,
    ) -> Result<GetMedicationBoxResponse, ApplicationError>;
}
