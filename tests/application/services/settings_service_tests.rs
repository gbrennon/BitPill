use crate::fakes::FakeSettingsRepository;
use bitpill::application::dtos::requests::{SettingsOperation, SettingsRequest};
use bitpill::application::errors::ApplicationError;
use bitpill::application::ports::inbound::settings_port::SettingsPort;
use bitpill::application::services::settings_service::SettingsService;
use serde_json::json;
use std::sync::Arc;

#[test]
fn execute_get_returns_settings() {
    let initial = json!({"vim_navigation": true});
    let repo = Arc::new(FakeSettingsRepository::new(initial.clone()));
    let svc = SettingsService::new(repo);

    let req = SettingsRequest {
        op: SettingsOperation::Get,
    };
    let resp = svc.execute(req).expect("execute failed");

    assert_eq!(resp.settings, initial);
}

#[test]
fn execute_update_persists_settings() {
    let initial = json!({"vim_navigation": false});
    let repo = Arc::new(FakeSettingsRepository::new(initial.clone()));
    let svc = SettingsService::new(repo.clone());

    let new_settings = json!({"vim_navigation": true});
    let req = SettingsRequest {
        op: SettingsOperation::Update {
            settings: new_settings.clone(),
        },
    };
    let resp = svc.execute(req).expect("execute failed");

    assert_eq!(resp.settings, new_settings);
    let last = repo.last_saved().expect("no saved value");
    assert_eq!(last, new_settings);
}

#[test]
fn execute_get_load_error_returns_storage() {
    let repo = Arc::new(FakeSettingsRepository::failing_load());
    let svc = SettingsService::new(repo);
    let req = SettingsRequest {
        op: SettingsOperation::Get,
    };

    let res = svc.execute(req);

    assert!(matches!(res, Err(ApplicationError::Storage(_))));
}

#[test]
fn execute_update_save_error_returns_storage() {
    let initial = json!({});
    let repo = Arc::new(FakeSettingsRepository::failing_save(initial));
    let svc = SettingsService::new(repo);
    let req = SettingsRequest {
        op: SettingsOperation::Update {
            settings: json!({"k": "v"}),
        },
    };

    let res = svc.execute(req);

    assert!(matches!(res, Err(ApplicationError::Storage(_))));
}
