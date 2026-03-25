use crate::{
    application::{
        dtos::{
            requests::SettingsRequest,
            responses::SettingsResponse,
        },
        errors::ApplicationError
    }
};

/// Inbound application port for settings-related use-cases.
/// Single execute method taking a Request DTO and returning a Response DTO.
pub trait UpdateSettingsPort: Send + Sync {
    fn execute(&self, req: SettingsRequest) -> Result<SettingsResponse, ApplicationError>;
}
