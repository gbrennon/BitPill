use std::fs;

use bitpill::infrastructure::config::{app_initializer::AppInitializer, app_paths::AppPaths};
use serde_json::Value;
use tempfile::{TempDir, tempdir};

struct Fixture {
    _dir: TempDir,
    pub paths: AppPaths,
}

impl Fixture {
    fn new() -> Self {
        let dir = tempdir().unwrap();
        let paths = AppPaths::with_paths(
            dir.path().to_path_buf(),
            dir.path().join("medications.json"),
            dir.path().join("dose_records.json"),
            dir.path().join("settings.json"),
        );
        Self { _dir: dir, paths }
    }
}

#[test]
fn initialize_creates_all_files_on_first_run() {
    let fixture = Fixture::new();

    AppInitializer::initialize(&fixture.paths).unwrap();

    assert!(fixture.paths.medications_path().exists());
    assert!(fixture.paths.dose_records_path().exists());
    assert!(fixture.paths.settings_path().exists());
}

#[test]
fn initialize_does_not_overwrite_existing_data_files() {
    let fixture = Fixture::new();
    let custom = r#"[{"id":"abc"}]"#;
    fs::write(fixture.paths.medications_path(), custom).unwrap();
    fs::write(fixture.paths.dose_records_path(), custom).unwrap();

    AppInitializer::initialize(&fixture.paths).unwrap();

    assert_eq!(fs::read_to_string(fixture.paths.medications_path()).unwrap(), custom);
    assert_eq!(fs::read_to_string(fixture.paths.dose_records_path()).unwrap(), custom);
}

#[test]
fn initialize_settings_created_with_defaults_when_absent() {
    let fixture = Fixture::new();

    AppInitializer::initialize(&fixture.paths).unwrap();

    let content = fs::read_to_string(fixture.paths.settings_path()).unwrap();
    let v: Value = serde_json::from_str(&content).unwrap();
    assert_eq!(v["vim_enabled"], Value::Bool(false));
}

#[test]
fn initialize_settings_does_not_overwrite_existing_user_value() {
    let fixture = Fixture::new();
    fs::write(fixture.paths.settings_path(), r#"{"vim_enabled": true}"#).unwrap();

    AppInitializer::initialize(&fixture.paths).unwrap();

    let content = fs::read_to_string(fixture.paths.settings_path()).unwrap();
    let v: Value = serde_json::from_str(&content).unwrap();
    assert_eq!(v["vim_enabled"], Value::Bool(true), "user value must be preserved");
}

#[test]
fn initialize_settings_adds_new_default_keys_to_existing_file() {
    let fixture = Fixture::new();
    fs::write(
        fixture.paths.settings_path(),
        r#"{"some_other_key": "user_value"}"#,
    )
    .unwrap();

    AppInitializer::initialize(&fixture.paths).unwrap();

    let content = fs::read_to_string(fixture.paths.settings_path()).unwrap();
    let v: Value = serde_json::from_str(&content).unwrap();
    assert_eq!(v["some_other_key"], "user_value", "user keys must be preserved");
    assert_eq!(v["vim_enabled"], Value::Bool(false), "missing default must be added");
}

#[test]
fn initialize_creates_config_dir_when_missing() {
    let base = tempdir().unwrap();
    let nested = base.path().join("deep").join("nested");
    let paths = AppPaths::with_paths(
        nested.clone(),
        nested.join("medications.json"),
        nested.join("dose_records.json"),
        nested.join("settings.json"),
    );

    AppInitializer::initialize(&paths).unwrap();

    assert!(nested.exists());
}
