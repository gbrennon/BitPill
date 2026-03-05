use serde_json::Value;

/// Response DTO for settings operations.
pub struct SettingsResponse {
    pub settings: Value,
}
