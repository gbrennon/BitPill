use std::sync::Mutex;

use bitpill::application::errors::StorageError;
use bitpill::application::ports::outbound::settings_repository_port::SettingsRepositoryPort;
use serde_json::Value;

pub struct FakeSettingsRepository {
    to_return: Value,
    saved: Mutex<Option<Value>>,
    fail_on_save: bool,
    fail_on_load: bool,
}

impl FakeSettingsRepository {
    pub fn new(initial: Value) -> Self {
        Self {
            to_return: initial,
            saved: Mutex::new(None),
            fail_on_save: false,
            fail_on_load: false,
        }
    }

    pub fn failing_load() -> Self {
        Self {
            to_return: Value::Null,
            saved: Mutex::new(None),
            fail_on_save: false,
            fail_on_load: true,
        }
    }

    pub fn failing_save(initial: Value) -> Self {
        Self {
            to_return: initial,
            saved: Mutex::new(None),
            fail_on_save: true,
            fail_on_load: false,
        }
    }

    pub fn last_saved(&self) -> Option<Value> {
        self.saved.lock().unwrap().clone()
    }
}

impl SettingsRepositoryPort for FakeSettingsRepository {
    fn load(&self) -> Result<Value, StorageError> {
        if self.fail_on_load {
            return Err(StorageError("load fail".into()));
        }
        Ok(self.to_return.clone())
    }

    fn save(&self, settings: &Value) -> Result<(), StorageError> {
        if self.fail_on_save {
            return Err(StorageError("save fail".into()));
        }
        *self.saved.lock().unwrap() = Some(settings.clone());
        Ok(())
    }
}
