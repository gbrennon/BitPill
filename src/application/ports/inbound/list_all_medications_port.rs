use crate::application::dtos::requests::ListAllMedicationsRequest;
use crate::application::dtos::responses::ListAllMedicationsResponse;
use crate::application::errors::ApplicationError;

pub trait ListAllMedicationsPort: Send + Sync {
    fn execute(
        &self,
        request: ListAllMedicationsRequest,
    ) -> Result<ListAllMedicationsResponse, ApplicationError>;
}
