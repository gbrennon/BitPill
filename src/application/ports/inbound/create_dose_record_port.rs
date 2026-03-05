use crate::application::dtos::requests::CreateDoseRecordRequest;
use crate::application::dtos::responses::CreateDoseRecordResponse;
use crate::application::errors::ApplicationError;

pub trait CreateDoseRecordPort: Send + Sync {
    fn execute(
        &self,
        request: CreateDoseRecordRequest,
    ) -> Result<CreateDoseRecordResponse, ApplicationError>;
}
