use crate::application::dtos::requests::SettingsRequest;
use crate::application::dtos::responses::SettingsResponse;
use crate::application::errors::ApplicationError;

/// Inbound application port for settings-related use-cases.
/// Single execute method taking a Request DTO and returning a Response DTO.
pub trait SettingsPort: Send + Sync {
    fn execute(&self, req: SettingsRequest) -> Result<SettingsResponse, ApplicationError>;
}
