use bitpill::infrastructure::persistence::json_settings_repository::JsonSettingsRepository;
use serde_json::json;
use std::env;
use std::fs;

#[test]
fn save_and_load_settings_roundtrip() {
    // create a unique path in the system temp dir
    let mut path = env::temp_dir();
    let file_name = format!("bitpill_settings_test_{}.json", std::process::id());
    path.push(file_name);

    // ensure no leftover file
    let _ = fs::remove_file(&path);

    let repo = JsonSettingsRepository::new(path.clone());
    let v = json!({"theme":"dark","volume":5});

    repo.save(&v).expect("save failed");
    let loaded = repo.load().expect("load failed");
    assert_eq!(loaded, v);

    // cleanup
    let _ = fs::remove_file(&path);
}
