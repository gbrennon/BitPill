use crate::application::{
    dtos::{requests::GetSettingsRequest, responses::GetSettingsResponse},
    errors::ApplicationError,
};

pub trait GetSettingsPort: Send + Sync {
    fn execute(&self, request: GetSettingsRequest)
    -> Result<GetSettingsResponse, ApplicationError>;
}
