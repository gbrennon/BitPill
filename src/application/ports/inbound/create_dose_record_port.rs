use crate::application::{
    dtos::{requests::CreateDoseRecordRequest, responses::CreateDoseRecordResponse},
    errors::ApplicationError,
};

pub trait CreateDoseRecordPort: Send + Sync {
    fn execute(
        &self,
        request: CreateDoseRecordRequest,
    ) -> Result<CreateDoseRecordResponse, ApplicationError>;
}
