use crate::application::errors::StorageError;
use crate::application::ports::settings_repository_port::SettingsRepositoryPort;
use crate::domain::entities::app_settings::AppSettings;
use crate::domain::value_objects::navigation_mode::NavigationMode;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct JsonSettingsRepository {
    path: PathBuf,
    inner_lock: Mutex<()>,
}

impl JsonSettingsRepository {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            inner_lock: Mutex::new(()),
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

impl SettingsRepositoryPort for JsonSettingsRepository {
    fn load(&self) -> Result<Option<AppSettings>, StorageError> {
        let _guard = self
            .inner_lock
            .lock()
            .map_err(|e| StorageError(format!("lock error: {e}")))?;

        if !self.path.exists() {
            return Ok(None);
        }

        let data =
            fs::read_to_string(&self.path).map_err(|e| StorageError(format!("read error: {e}")))?;

        if data.trim().is_empty() {
            return Ok(None);
        }

        let v: serde_json::Value =
            serde_json::from_str(&data).map_err(|e| StorageError(format!("parse error: {e}")))?;

        let navigation_mode = match v.get("navigation_mode").and_then(|v| v.as_str()) {
            Some(mode_str) => NavigationMode::try_from(mode_str)
                .map_err(|e| StorageError(format!("invalid navigation mode: {e}")))?,
            None => NavigationMode::new(
                crate::domain::value_objects::navigation_mode::NavigationModeVariant::Vi,
            )
            .map_err(|e| StorageError(format!("default navigation mode error: {e}")))?,
        };

        Ok(Some(AppSettings::new(navigation_mode)))
    }

    fn save(&self, settings: &AppSettings) -> Result<(), StorageError> {
        let v = serde_json::json!({
            "navigation_mode": settings.navigation_mode().as_str()
        });
        let serialized = serde_json::to_string_pretty(&v)
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
    use crate::domain::value_objects::navigation_mode::NavigationModeVariant;
    use tempfile::tempdir;

    #[test]
    fn save_and_load_roundtrip() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("settings.json");
        let repo = JsonSettingsRepository::new(path.clone());
        let settings = AppSettings::new(NavigationMode::new(NavigationModeVariant::Vi).unwrap());
        repo.save(&settings).expect("save should work");
        let loaded = repo.load().expect("load should work");
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().navigation_mode().as_str(), "vi");
    }

    #[test]
    fn load_returns_none_when_file_does_not_exist() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("nonexistent.json");
        let repo = JsonSettingsRepository::new(path);

        let result = repo.load();

        assert!(matches!(result, Ok(None)));
    }

    #[test]
    fn save_returns_error_when_path_is_a_directory() {
        let dir = tempdir().unwrap();
        let repo = JsonSettingsRepository::new(dir.path().to_path_buf());
        let settings = AppSettings::new(NavigationMode::new(NavigationModeVariant::Vi).unwrap());

        let result = repo.save(&settings);

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
        std::fs::write(&path, r#"{"navigation_mode":"vi"}"#).unwrap();
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
        let settings = AppSettings::new(NavigationMode::new(NavigationModeVariant::Vi).unwrap());

        let _ = std::thread::spawn(move || {
            let _guard = clone.inner_lock.lock().unwrap();
            panic!("poison");
        })
        .join();

        assert!(repo.save(&settings).is_err());
    }
}
