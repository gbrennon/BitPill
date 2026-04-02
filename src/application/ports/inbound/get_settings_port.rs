use crate::application::{
    dtos::{requests::GetSettingsRequest, responses::GetSettingsResponse},
    errors::ApplicationError,
};

pub trait GetSettingsPort {
    fn execute(&self, request: GetSettingsRequest)
    -> Result<GetSettingsResponse, ApplicationError>;
}
