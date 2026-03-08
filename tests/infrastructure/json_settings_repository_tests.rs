use bitpill::{
    application::ports::outbound::settings_repository_port::SettingsRepositoryPort,
    infrastructure::persistence::json_settings_repository::JsonSettingsRepository,
};
use serde_json::json;
use tempfile::tempdir;

#[test]
fn save_and_load_settings_roundtrip() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("settings.json");

    let repo = JsonSettingsRepository::new(path.clone());
    let v = json!({"theme":"dark","volume":5});

    repo.save(&v).expect("save failed");
    let loaded = repo.load().expect("load failed");

    assert_eq!(loaded, v);
}
