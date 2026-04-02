use bitpill::{
    application::ports::outbound::settings_repository_port::SettingsRepositoryPort,
    domain::{
        entities::app_settings::AppSettings,
        value_objects::navigation_mode::{NavigationMode, NavigationModeVariant},
    },
    infrastructure::persistence::json_settings_repository::JsonSettingsRepository,
};
use tempfile::tempdir;

#[test]
fn save_and_load_settings_roundtrip() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("settings.json");

    let repo = JsonSettingsRepository::new(path.clone());
    let settings = AppSettings::new(NavigationMode::new(NavigationModeVariant::Vi).unwrap());

    repo.save(&settings).expect("save failed");
    let loaded = repo.load().expect("load failed");

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
