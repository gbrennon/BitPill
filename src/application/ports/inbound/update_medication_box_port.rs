use crate::application::{
    dtos::{requests::UpdateMedicationBoxRequest, responses::UpdateMedicationBoxResponse},
    errors::ApplicationError,
};

pub trait UpdateMedicationBoxPort: Send + Sync {
    fn execute(
        &self,
        request: UpdateMedicationBoxRequest,
    ) -> Result<UpdateMedicationBoxResponse, ApplicationError>;
}
