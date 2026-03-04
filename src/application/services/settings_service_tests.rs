#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::settings_repository_port::SettingsRepositoryPort;
    use crate::application::ports::inbound::settings_port::{SettingsRequest, SettingsOperation};
    use crate::application::services::SettingsService;
    use serde_json::json;
    use serde_json::Value;
    use std::sync::{Arc, Mutex};

    struct FakeSettingsRepository {
        to_return: Value,
        saved: Mutex<Option<Value>>,
    }

    impl FakeSettingsRepository {
        fn new(initial: Value) -> Self {
            Self { to_return: initial, saved: Mutex::new(None) }
        }

        fn last_saved(&self) -> Option<Value> {
            self.saved.lock().unwrap().clone()
        }
    }

    impl SettingsRepositoryPort for FakeSettingsRepository {
        fn load(&self) -> Result<Value, crate::application::errors::StorageError> {
            Ok(self.to_return.clone())
        }

        fn save(&self, settings: &Value) -> Result<(), crate::application::errors::StorageError> {
            let mut guard = self.saved.lock().unwrap();
            *guard = Some(settings.clone());
            Ok(())
        }
    }

    #[test]
    fn execute_get_returns_settings() {
        let initial = json!({"vim_navigation": true});
        let repo = Arc::new(FakeSettingsRepository::new(initial.clone()));
        let svc = SettingsService::new(repo);

        let req = SettingsRequest { op: SettingsOperation::Get };
        let resp = svc.execute(req).expect("execute failed");
        assert_eq!(resp.settings, initial);
    }

    #[test]
    fn execute_update_persists_settings() {
        let initial = json!({"vim_navigation": false});
        let repo = Arc::new(FakeSettingsRepository::new(initial.clone()));
        let svc = SettingsService::new(repo.clone());

        let new_settings = json!({"vim_navigation": true});
        let req = SettingsRequest { op: SettingsOperation::Update { settings: new_settings.clone() } };
        let resp = svc.execute(req).expect("execute failed");
        assert_eq!(resp.settings, new_settings);

        // ensure repository saved the settings
        let last = repo.last_saved().expect("no saved value");
        assert_eq!(last, new_settings);
    }
}
