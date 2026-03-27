use crate::application::{
    dtos::{requests::EditMedicationRequest, responses::EditMedicationResponse},
    errors::ApplicationError,
};

pub trait EditMedicationPort: Send + Sync {
    fn execute(
        &self,
        request: EditMedicationRequest,
    ) -> Result<EditMedicationResponse, ApplicationError>;
}
