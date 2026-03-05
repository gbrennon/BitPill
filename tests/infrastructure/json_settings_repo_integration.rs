use tempfile::tempdir;
use bitpill::infrastructure::container::Container;
use bitpill::application::dtos::requests::{SettingsOperation, SettingsRequest};
use serde_json::json;

#[test]
fn settings_persisted_across_containers() {
    let dir = tempdir().unwrap();
    let meds = dir.path().join("meds.json");
    let doses = dir.path().join("doses.json");
    let settings = dir.path().join("settings.json");

    let mut c1 = Container::new_with_paths(meds.clone(), doses.clone(), settings.clone());
    let req = SettingsRequest { op: SettingsOperation::Update { settings: json!({"k":"v"}) } };
    c1.get_settings_service().execute(req).expect("save");

    let c2 = Container::new_with_paths(meds, doses, settings);
    let get = SettingsRequest { op: SettingsOperation::Get };
    let resp = c2.get_settings_service().execute(get).expect("load");
    assert_eq!(resp.settings["k"], "v");
}
