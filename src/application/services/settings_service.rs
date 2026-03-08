use std::sync::Arc;

use crate::application::dtos::requests::{SettingsOperation, SettingsRequest};
use crate::application::dtos::responses::SettingsResponse;
use crate::application::errors::ApplicationError;
use crate::application::ports::inbound::settings_port::SettingsPort;
use crate::application::ports::settings_repository_port::SettingsRepositoryPort;

pub struct SettingsService {
    repository: Arc<dyn SettingsRepositoryPort>,
}

impl SettingsService {
    pub fn new(repository: Arc<dyn SettingsRepositoryPort>) -> Self {
        Self { repository }
    }
}

impl SettingsPort for SettingsService {
    fn execute(&self, req: SettingsRequest) -> Result<SettingsResponse, ApplicationError> {
        match req.op {
            SettingsOperation::Get => {
                let v = self.repository.load()?;
                Ok(SettingsResponse { settings: v })
            }
            SettingsOperation::Update { settings } => {
                self.repository.save(&settings)?;
                Ok(SettingsResponse { settings })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    struct InMemorySettingsRepo {
        value: std::sync::Mutex<serde_json::Value>,
    }

    impl InMemorySettingsRepo {
        fn new(v: serde_json::Value) -> Self {
            Self {
                value: std::sync::Mutex::new(v),
            }
        }
    }

    impl crate::application::ports::outbound::settings_repository_port::SettingsRepositoryPort
        for InMemorySettingsRepo
    {
        fn load(&self) -> Result<serde_json::Value, crate::application::errors::StorageError> {
            Ok(self.value.lock().unwrap().clone())
        }

        fn save(
            &self,
            settings: &serde_json::Value,
        ) -> Result<(), crate::application::errors::StorageError> {
            *self.value.lock().unwrap() = settings.clone();
            Ok(())
        }
    }

    fn make_service(repo: std::sync::Arc<InMemorySettingsRepo>) -> SettingsService {
        SettingsService::new(repo)
    }

    #[test]
    fn execute_get_returns_saved_settings() {
        let repo = std::sync::Arc::new(InMemorySettingsRepo::new(json!({"k": "v"})));
        let service = make_service(repo);
        let req = SettingsRequest {
            op: SettingsOperation::Get,
        };

        let res = service.execute(req).unwrap();
        assert_eq!(res.settings, json!({"k":"v"}));
    }

    #[test]
    fn execute_update_saves_and_returns_settings() {
        let repo = std::sync::Arc::new(InMemorySettingsRepo::new(json!({})));
        let service = make_service(repo.clone());
        let new_settings = json!({"a":1});
        let req = SettingsRequest {
            op: SettingsOperation::Update {
                settings: new_settings.clone(),
            },
        };

        let res = service.execute(req).unwrap();
        assert_eq!(res.settings, new_settings);
        assert_eq!(*repo.value.lock().unwrap(), new_settings);
    }
}
