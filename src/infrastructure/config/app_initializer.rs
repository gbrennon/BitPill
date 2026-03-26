use std::fs;
use std::io;
use std::path::Path;

use serde_json::{Map, Value};

use crate::infrastructure::config::app_paths::AppPaths;

const DEFAULT_MEDICATIONS: &str = "[]";
const DEFAULT_DOSE_RECORDS: &str = "[]";

fn default_settings() -> Value {
    let mut map = Map::new();
    map.insert("vim_enabled".to_string(), Value::Bool(false));
    Value::Object(map)
}

/// Bootstrap service that ensures `~/.config/bitpill/` and its data files
/// exist before repositories are constructed.
///
/// Rules:
/// - Creates the config directory if it does not exist.
/// - Creates `medications.json` and `dose_records.json` with empty arrays
///   only when the files are absent (never overwrites).
/// - Creates `settings.json` with defaults when absent.
///   When present, additively merges any missing default keys without
///   touching existing user values.
pub struct AppInitializer;

impl AppInitializer {
    pub fn initialize(paths: &AppPaths) -> io::Result<()> {
        fs::create_dir_all(paths.config_dir())?;

        Self::init_data_file(paths.medications_path(), DEFAULT_MEDICATIONS)?;
        Self::init_data_file(paths.dose_records_path(), DEFAULT_DOSE_RECORDS)?;
        Self::init_settings_file(paths.settings_path())?;

        Ok(())
    }

    /// Initialize from explicit paths instead of AppPaths.
    pub fn initialize_from_paths(
        medications_path: &Path,
        dose_records_path: &Path,
        settings_path: &Path,
    ) -> io::Result<()> {
        if let Some(parent) = medications_path.parent() {
            fs::create_dir_all(parent)?;
        }

        Self::init_data_file(medications_path, DEFAULT_MEDICATIONS)?;
        Self::init_data_file(dose_records_path, DEFAULT_DOSE_RECORDS)?;
        Self::init_settings_file(settings_path)?;

        Ok(())
    }

    fn init_data_file(path: &Path, default_content: &str) -> io::Result<()> {
        if !path.exists() {
            fs::write(path, default_content)?;
        }
        Ok(())
    }

    fn init_settings_file(path: &Path) -> io::Result<()> {
        if !path.exists() {
            let serialized = serde_json::to_string_pretty(&default_settings())
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            return fs::write(path, serialized);
        }

        Self::merge_default_settings(path)
    }

    fn merge_default_settings(path: &Path) -> io::Result<()> {
        let raw = fs::read_to_string(path)?;
        let mut existing: Value = serde_json::from_str(&raw)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let defaults = default_settings();
        let mut changed = false;

        if let (Some(user_map), Value::Object(default_map)) = (existing.as_object_mut(), defaults) {
            for (key, value) in default_map {
                if !user_map.contains_key(&key) {
                    user_map.insert(key, value);
                    changed = true;
                }
            }
        }

        if changed {
            let serialized = serde_json::to_string_pretty(&existing)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            fs::write(path, serialized)?;
        }

        Ok(())
    }
}
