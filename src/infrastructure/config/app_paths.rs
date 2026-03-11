use std::path::PathBuf;

const APP_DIR_NAME: &str = "bitpill";
const ENV_MEDICATIONS: &str = "BITPILL_MEDICATIONS_FILE";
const ENV_DOSE_RECORDS: &str = "BITPILL_DOSE_RECORDS_FILE";
const ENV_SETTINGS: &str = "BITPILL_SETTINGS_FILE";

/// Resolves all file system paths for BitPill's persistent data.
///
/// Default resolution (no env overrides):
///   - config dir   → `~/.config/bitpill/`
///   - medications  → `~/.config/bitpill/medications.json`
///   - dose records → `~/.config/bitpill/dose_records.json`
///   - settings     → `~/.config/bitpill/settings.json`
///
/// Each path can be overridden via its env var, which is useful for
/// integration tests and custom deployments.
pub struct AppPaths {
    config_dir: PathBuf,
    medications: PathBuf,
    dose_records: PathBuf,
    settings: PathBuf,
}

impl AppPaths {
    /// Builds paths from env var overrides or XDG-standard defaults.
    pub fn resolve() -> Self {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from(".config"))
            .join(APP_DIR_NAME);

        let medications = std::env::var(ENV_MEDICATIONS)
            .map(PathBuf::from)
            .unwrap_or_else(|_| config_dir.join("medications.json"));

        let dose_records = std::env::var(ENV_DOSE_RECORDS)
            .map(PathBuf::from)
            .unwrap_or_else(|_| config_dir.join("dose_records.json"));

        let settings = std::env::var(ENV_SETTINGS)
            .map(PathBuf::from)
            .unwrap_or_else(|_| config_dir.join("settings.json"));

        Self {
            config_dir,
            medications,
            dose_records,
            settings,
        }
    }

    /// Constructs paths from explicit values. Intended for tests only.
    #[cfg(any(test, feature = "test-helpers"))]
    pub fn with_paths(
        config_dir: PathBuf,
        medications: PathBuf,
        dose_records: PathBuf,
        settings: PathBuf,
    ) -> Self {
        Self {
            config_dir,
            medications,
            dose_records,
            settings,
        }
    }

    pub fn config_dir(&self) -> &PathBuf {
        &self.config_dir
    }

    pub fn medications_path(&self) -> &PathBuf {
        &self.medications
    }

    pub fn dose_records_path(&self) -> &PathBuf {
        &self.dose_records
    }

    pub fn settings_path(&self) -> &PathBuf {
        &self.settings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_returns_paths_ending_with_expected_filenames() {
        let paths = AppPaths::resolve();

        assert!(
            paths
                .medications_path()
                .to_str()
                .unwrap()
                .ends_with("medications.json")
        );
        assert!(
            paths
                .dose_records_path()
                .to_str()
                .unwrap()
                .ends_with("dose_records.json")
        );
        assert!(
            paths
                .settings_path()
                .to_str()
                .unwrap()
                .ends_with("settings.json")
        );
    }

    #[test]
    fn config_dir_contains_bitpill_segment() {
        let paths = AppPaths::resolve();

        assert!(
            paths
                .config_dir()
                .components()
                .any(|c| c.as_os_str() == APP_DIR_NAME)
        );
    }

    #[test]
    fn with_paths_stores_provided_paths() {
        let paths = AppPaths::with_paths(
            PathBuf::from("/tmp/cfg"),
            PathBuf::from("/tmp/meds.json"),
            PathBuf::from("/tmp/doses.json"),
            PathBuf::from("/tmp/settings.json"),
        );

        assert_eq!(paths.config_dir(), &PathBuf::from("/tmp/cfg"));
        assert_eq!(paths.medications_path(), &PathBuf::from("/tmp/meds.json"));
        assert_eq!(paths.dose_records_path(), &PathBuf::from("/tmp/doses.json"));
        assert_eq!(paths.settings_path(), &PathBuf::from("/tmp/settings.json"));
    }
}
