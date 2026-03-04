use crate::application::errors::ApplicationError;
use serde_json::Value;

/// Request DTO for settings operations.
pub struct SettingsRequest {
    pub op: SettingsOperation,
}

/// Supported operations for the Settings inbound port.
pub enum SettingsOperation {
    Get,
    Update { settings: Value },
}

/// Response DTO for settings operations.
pub struct SettingsResponse {
    pub settings: Value,
}

/// Inbound application port for settings-related use-cases.
/// Single execute method taking a Request DTO and returning a Response DTO.
pub trait SettingsPort: Send + Sync {
    fn execute(&self, req: SettingsRequest) -> Result<SettingsResponse, ApplicationError>;
}
