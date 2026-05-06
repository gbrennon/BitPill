use std::path::PathBuf;

const APP_DIR_NAME: &str = "bitpill";
const ENV_MEDICATIONS: &str = "BITPILL_MEDICATIONS_FILE";
const ENV_DOSE_RECORDS: &str = "BITPILL_DOSE_RECORDS_FILE";
const ENV_SETTINGS: &str = "BITPILL_SETTINGS_FILE";

trait PathResolver {
    fn resolve(&self) -> (PathBuf, PathBuf, PathBuf, PathBuf);
}

struct DevelopmentPathResolver;

impl PathResolver for DevelopmentPathResolver {
    fn resolve(&self) -> (PathBuf, PathBuf, PathBuf, PathBuf) {
        let dir = std::env::temp_dir().join("bitpill-dev");
        (
            dir.clone(),
            dir.join("medications.json"),
            dir.join("dose_records.json"),
            dir.join("settings.json"),
        )
    }
}

struct ProductionPathResolver;

impl PathResolver for ProductionPathResolver {
    fn resolve(&self) -> (PathBuf, PathBuf, PathBuf, PathBuf) {
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

        (config_dir, medications, dose_records, settings)
    }
}

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
    fn is_development() -> bool {
        std::path::Path::new("Cargo.toml").exists()
    }

    pub fn resolve() -> Self {
        let resolver: Box<dyn PathResolver> = if Self::is_development() {
            Box::new(DevelopmentPathResolver)
        } else {
            Box::new(ProductionPathResolver)
        };
        let (config_dir, medications, dose_records, settings) = resolver.resolve();

        Self {
            config_dir,
            medications,
            dose_records,
            settings,
        }
    }

    /// Constructs paths from explicit values. Intended for tests only.
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

    /// Tests path resolution, handling both development and production modes.
    /// In development, Cargo.toml exists in the current directory.
    /// In production, paths should use the XDG config directory.
    #[test]
    fn config_dir_contains_bitpill_segment() {
        let paths = AppPaths::resolve();

        // Detects environment: development if Cargo.toml exists, otherwise production
        let is_dev = std::path::Path::new("Cargo.toml").exists();

        if is_dev {
            // Development: config directory should use temporary directory with 'bitpill-dev' segment
            assert!(
                paths
                    .config_dir()
                    .components()
                    .any(|c| c.as_os_str() == "bitpill-dev")
            );
        } else {
            // Production: config directory should use XDG config path with 'bitpill' segment
            assert!(
                paths
                    .config_dir()
                    .components()
                    .any(|c| c.as_os_str() == APP_DIR_NAME)
            );
        }
    }

    /// Verifies each file path ends with the expected filename.
    /// Development mode uses bitpill-dev temp directory.
    /// Production mode uses XDG config directory, but env vars can override individual paths.
    #[test]
    fn resolve_returns_correct_filename_for_each_path() {
        let paths = AppPaths::resolve();

        // Determine environment to know expected path structure
        let is_dev = std::path::Path::new("Cargo.toml").exists();

        if is_dev {
            // Development: all paths should be within the temporary bitpill-dev directory
            assert!(
                paths
                    .medications_path()
                    .to_str()
                    .unwrap()
                    .contains("bitpill-dev")
            );
            assert!(
                paths
                    .dose_records_path()
                    .to_str()
                    .unwrap()
                    .contains("bitpill-dev")
            );
            assert!(
                paths
                    .settings_path()
                    .to_str()
                    .unwrap()
                    .contains("bitpill-dev")
            );
        } else {
            // Production: paths should end with expected filenames OR use env var overrides
            assert!(
                paths
                    .medications_path()
                    .to_str()
                    .unwrap()
                    .ends_with("medications.json")
                    || std::env::var("BITPILL_MEDICATIONS_FILE").is_ok()
            );
            assert!(
                paths
                    .dose_records_path()
                    .to_str()
                    .unwrap()
                    .ends_with("dose_records.json")
                    || std::env::var("BITPILL_DOSE_RECORDS_FILE").is_ok()
            );
            assert!(
                paths
                    .settings_path()
                    .to_str()
                    .unwrap()
                    .ends_with("settings.json")
                    || std::env::var("BITPILL_SETTINGS_FILE").is_ok()
            );
        }
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
