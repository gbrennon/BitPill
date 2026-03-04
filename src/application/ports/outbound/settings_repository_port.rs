use crate::application::errors::StorageError;
use serde_json::Value;

pub trait SettingsRepositoryPort: Send + Sync {
    fn load(&self) -> Result<Value, StorageError>;
    fn save(&self, settings: &Value) -> Result<(), StorageError>;
}

// Boxed trait alias for simpler wiring in the container
pub type SettingsRepositoryPortBox = dyn SettingsRepositoryPort + Send + Sync;
