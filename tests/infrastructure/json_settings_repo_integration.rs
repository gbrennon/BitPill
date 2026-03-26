use bitpill::{
    application::dtos::requests::{SettingsOperation, SettingsRequest},
    infrastructure::container::Container,
};
use serde_json::json;
use tempfile::tempdir;

#[test]
fn settings_persisted_across_containers() {
    let dir = tempdir().unwrap();
    let meds = dir.path().join("meds.json");
    let doses = dir.path().join("doses.json");
    let settings = dir.path().join("settings.json");

    let c1 = Container::new(meds.clone(), doses.clone(), settings.clone());
    let req = SettingsRequest {
        op: SettingsOperation::Update {
            settings: json!({"k":"v"}),
        },
    };
    c1.settings_service.execute(req).expect("save");

    let c2 = Container::new(meds, doses, settings);
    let get = SettingsRequest {
        op: SettingsOperation::Get,
    };
    let resp = c2.settings_service.execute(get).expect("load");
    assert_eq!(resp.settings["k"], "v");
}
