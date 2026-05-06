use std::path::PathBuf;

use bitpill::infrastructure::config::app_paths::AppPaths;

const APP_DIR_NAME: &str = "bitpill";

#[test]
fn with_paths_config_dir_returns_the_config_directory() {
    let paths = AppPaths::with_paths(
        PathBuf::from("/tmp/cfg"),
        PathBuf::from("/tmp/meds.json"),
        PathBuf::from("/tmp/doses.json"),
        PathBuf::from("/tmp/settings.json"),
    );

    assert_eq!(paths.config_dir(), &PathBuf::from("/tmp/cfg"));
}

#[test]
fn with_paths_medications_path_returns_the_medications_file() {
    let paths = AppPaths::with_paths(
        PathBuf::from("/tmp/cfg"),
        PathBuf::from("/tmp/meds.json"),
        PathBuf::from("/tmp/doses.json"),
        PathBuf::from("/tmp/settings.json"),
    );

    assert_eq!(paths.medications_path(), &PathBuf::from("/tmp/meds.json"));
}

#[test]
fn with_paths_dose_records_path_returns_the_dose_records_file() {
    let paths = AppPaths::with_paths(
        PathBuf::from("/tmp/cfg"),
        PathBuf::from("/tmp/meds.json"),
        PathBuf::from("/tmp/doses.json"),
        PathBuf::from("/tmp/settings.json"),
    );

    assert_eq!(paths.dose_records_path(), &PathBuf::from("/tmp/doses.json"));
}

#[test]
fn with_paths_settings_path_returns_the_settings_file() {
    let paths = AppPaths::with_paths(
        PathBuf::from("/tmp/cfg"),
        PathBuf::from("/tmp/meds.json"),
        PathBuf::from("/tmp/doses.json"),
        PathBuf::from("/tmp/settings.json"),
    );

    assert_eq!(paths.settings_path(), &PathBuf::from("/tmp/settings.json"));
}

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

#[test]
fn with_paths_handles_absolute_paths() {
    let paths = AppPaths::with_paths(
        PathBuf::from("/custom/absolute/path"),
        PathBuf::from("/custom/absolute/meds.json"),
        PathBuf::from("/custom/absolute/doses.json"),
        PathBuf::from("/custom/absolute/settings.json"),
    );

    assert_eq!(paths.config_dir(), &PathBuf::from("/custom/absolute/path"));
    assert!(paths.medications_path().is_absolute());
    assert!(paths.dose_records_path().is_absolute());
    assert!(paths.settings_path().is_absolute());
}
