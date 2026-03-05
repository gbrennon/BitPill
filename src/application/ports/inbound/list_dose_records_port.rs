use crate::application::dtos::requests::ListDoseRecordsRequest;
use crate::application::dtos::responses::ListDoseRecordsResponse;
use crate::application::errors::ApplicationError;

pub trait ListDoseRecordsPort: Send + Sync {
    fn execute(
        &self,
        request: ListDoseRecordsRequest,
    ) -> Result<ListDoseRecordsResponse, ApplicationError>;
}
