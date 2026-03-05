use std::sync::Arc;

use crate::application::errors::ApplicationError;
use crate::application::ports::inbound::settings_port::{
    SettingsOperation, SettingsPort, SettingsRequest, SettingsResponse,
};
use crate::application::ports::settings_repository_port::SettingsRepositoryPort;

pub struct SettingsService {
    repository: Arc<dyn SettingsRepositoryPort>,
}

impl SettingsService {
    pub fn new(repository: Arc<dyn SettingsRepositoryPort>) -> Self {
        Self { repository }
    }
}

// Test imports at top
#[cfg(test)]
use serde_json::json;
#[cfg(test)]
use serde_json::Value;

impl SettingsPort for SettingsService {
    fn execute(&self, req: SettingsRequest) -> Result<SettingsResponse, ApplicationError> {
        match req.op {
            SettingsOperation::Get => {
                let v = self.repository.load()?;
                Ok(SettingsResponse { settings: v })
            }
            SettingsOperation::Update { settings } => {
                // validate? just persist as-is
                self.repository.save(&settings)?;
                Ok(SettingsResponse { settings })
            }
        }
    }
}

// Unit tests located in the same file as the implementation
#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn execute_get_load_error_returns_storage() {
        struct FailingRepo;
        impl crate::application::ports::outbound::settings_repository_port::SettingsRepositoryPort for FailingRepo {
            fn load(&self) -> Result<serde_json::Value, crate::application::errors::StorageError> {
                Err(crate::application::errors::StorageError("load fail".into()))
            }
            fn save(&self, _settings: &serde_json::Value) -> Result<(), crate::application::errors::StorageError> {
                Ok(())
            }
        }
        let repo = Arc::new(FailingRepo);
        let svc = SettingsService::new(repo);
        let req = SettingsRequest { op: SettingsOperation::Get };
        let res = svc.execute(req);
        assert!(matches!(res, Err(crate::application::errors::ApplicationError::Storage(_))));
    }

    #[test]
    fn execute_update_save_error_returns_storage() {
        struct FailingRepo;
        impl crate::application::ports::outbound::settings_repository_port::SettingsRepositoryPort for FailingRepo {
            fn load(&self) -> Result<serde_json::Value, crate::application::errors::StorageError> {
                Ok(serde_json::json!({}))
            }
            fn save(&self, _settings: &serde_json::Value) -> Result<(), crate::application::errors::StorageError> {
                Err(crate::application::errors::StorageError("save fail".into()))
            }
        }
        let repo = Arc::new(FailingRepo);
        let svc = SettingsService::new(repo);
        let new_settings = serde_json::json!({"k": "v"});
        let req = SettingsRequest { op: SettingsOperation::Update { settings: new_settings } };
        let res = svc.execute(req);
        assert!(matches!(res, Err(crate::application::errors::ApplicationError::Storage(_))));
    }
}
