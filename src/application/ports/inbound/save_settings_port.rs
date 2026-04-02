use crate::application::{
    dtos::{requests::SaveSettingsRequest, responses::SaveSettingsResponse},
    errors::ApplicationError,
};

pub trait SaveSettingsPort {
    fn execute(
        &self,
        request: SaveSettingsRequest,
    ) -> Result<SaveSettingsResponse, ApplicationError>;
}
