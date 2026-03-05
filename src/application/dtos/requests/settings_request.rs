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
