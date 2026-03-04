use crate::application::errors::StorageError;
use crate::application::ports::settings_repository_port::SettingsRepositoryPort;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct JsonSettingsRepository {
    path: PathBuf,
    // reuse same Mutex pattern as other json repos for thread-safety
    inner_lock: Mutex<()>,
}

impl JsonSettingsRepository {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            inner_lock: Mutex::new(()),
        }
    }
}

impl SettingsRepositoryPort for JsonSettingsRepository {
    fn load(&self) -> Result<Value, StorageError> {
        let _guard = self
            .inner_lock
            .lock()
            .map_err(|e| StorageError(format!("lock error: {e}")))?;
        let data =
            fs::read_to_string(&self.path).map_err(|e| StorageError(format!("read error: {e}")))?;
        let v: Value =
            serde_json::from_str(&data).map_err(|e| StorageError(format!("parse error: {e}")))?;
        Ok(v)
    }

    fn save(&self, settings: &Value) -> Result<(), StorageError> {
        // Serialize while holding lock, write file outside lock to avoid blocking others
        let serialized = serde_json::to_string_pretty(settings)
            .map_err(|e| StorageError(format!("serialize error: {e}")))?;
        drop(
            self.inner_lock
                .lock()
                .map_err(|e| StorageError(format!("lock error: {e}")))?,
        );
        fs::write(&self.path, serialized).map_err(|e| StorageError(format!("write error: {e}")))?;
        Ok(())
    }
}
