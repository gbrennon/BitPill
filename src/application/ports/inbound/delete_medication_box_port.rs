use crate::application::{
    dtos::{requests::DeleteMedicationBoxRequest, responses::DeleteMedicationBoxResponse},
    errors::ApplicationError,
};

pub trait DeleteMedicationBoxPort: Send + Sync {
    fn execute(
        &self,
        request: DeleteMedicationBoxRequest,
    ) -> Result<DeleteMedicationBoxResponse, ApplicationError>;
}
