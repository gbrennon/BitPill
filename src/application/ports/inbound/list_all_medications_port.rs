use crate::application::{
    dtos::{requests::ListAllMedicationsRequest, responses::ListAllMedicationsResponse},
    errors::ApplicationError,
};

pub trait ListAllMedicationsPort: Send + Sync {
    fn execute(
        &self,
        request: ListAllMedicationsRequest,
    ) -> Result<ListAllMedicationsResponse, ApplicationError>;
}
