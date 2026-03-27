use crate::application::{
    dtos::{requests::ListDoseRecordsRequest, responses::ListDoseRecordsResponse},
    errors::ApplicationError,
};

pub trait ListDoseRecordsPort: Send + Sync {
    fn execute(
        &self,
        request: ListDoseRecordsRequest,
    ) -> Result<ListDoseRecordsResponse, ApplicationError>;
}
