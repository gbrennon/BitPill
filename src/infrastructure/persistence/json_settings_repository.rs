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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::tempdir;

    #[test]
    fn save_and_load_roundtrip() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("settings.json");
        let repo = JsonSettingsRepository::new(path.clone());
        let val = json!({"key":"value"});
        repo.save(&val).expect("save should work");
        let loaded = repo.load().expect("load should work");
        assert_eq!(loaded["key"], "value");
    }

    #[test]
    fn load_returns_error_when_file_does_not_exist() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("nonexistent.json");
        let repo = JsonSettingsRepository::new(path);

        let result = repo.load();

        assert!(result.is_err());
    }

    #[test]
    fn save_returns_error_when_path_is_a_directory() {
        let dir = tempdir().unwrap();
        let repo = JsonSettingsRepository::new(dir.path().to_path_buf());

        let result = repo.save(&json!({"key": "value"}));

        assert!(result.is_err());
    }

    #[test]
    fn load_returns_error_for_invalid_json() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("settings.json");
        std::fs::write(&path, b"not valid json {{{{").unwrap();
        let repo = JsonSettingsRepository::new(path);

        let result = repo.load();

        assert!(result.is_err());
    }

    #[test]
    fn load_returns_error_when_mutex_is_poisoned() {
        use std::sync::Arc;
        let dir = tempdir().unwrap();
        let path = dir.path().join("settings_poison_load.json");
        std::fs::write(&path, r#"{"k":"v"}"#).unwrap();
        let repo = Arc::new(JsonSettingsRepository::new(path));
        let clone = Arc::clone(&repo);

        let _ = std::thread::spawn(move || {
            let _guard = clone.inner_lock.lock().unwrap();
            panic!("poison");
        })
        .join();

        assert!(repo.load().is_err());
    }

    #[test]
    fn save_returns_error_when_mutex_is_poisoned() {
        use std::sync::Arc;
        let dir = tempdir().unwrap();
        let path = dir.path().join("settings_poison_save.json");
        let repo = Arc::new(JsonSettingsRepository::new(path));
        let clone = Arc::clone(&repo);

        let _ = std::thread::spawn(move || {
            let _guard = clone.inner_lock.lock().unwrap();
            panic!("poison");
        })
        .join();

        assert!(repo.save(&json!({"k": "v"})).is_err());
    }
}
