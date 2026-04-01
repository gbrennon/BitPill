use std::sync::Mutex;

use bitpill::application::errors::StorageError;
use bitpill::application::ports::outbound::settings_repository_port::SettingsRepositoryPort;
use bitpill::domain::entities::app_settings::AppSettings;
use bitpill::domain::value_objects::navigation_mode::{NavigationMode, NavigationModeVariant};

pub struct FakeSettingsRepository {
    to_return: Option<AppSettings>,
    saved: Mutex<Option<AppSettings>>,
    fail_on_save: bool,
    fail_on_load: bool,
}

impl FakeSettingsRepository {
    pub fn new(app_settings: AppSettings) -> Self {
        Self {
            to_return: Some(app_settings),
            saved: Mutex::new(None),
            fail_on_save: false,
            fail_on_load: false,
        }
    }

    pub fn empty() -> Self {
        Self {
            to_return: None,
            saved: Mutex::new(None),
            fail_on_save: false,
            fail_on_load: false,
        }
    }

    pub fn failing_load() -> Self {
        Self {
            to_return: None,
            saved: Mutex::new(None),
            fail_on_save: false,
            fail_on_load: true,
        }
    }

    pub fn failing_save(app_settings: AppSettings) -> Self {
        Self {
            to_return: Some(app_settings),
            saved: Mutex::new(None),
            fail_on_save: true,
            fail_on_load: false,
        }
    }

    pub fn last_saved(&self) -> Option<AppSettings> {
        self.saved.lock().unwrap().clone()
    }
}

impl SettingsRepositoryPort for FakeSettingsRepository {
    fn load(&self) -> Result<Option<AppSettings>, StorageError> {
        if self.fail_on_load {
            return Err(StorageError("load fail".into()));
        }
        Ok(self.to_return.clone())
    }

    fn save(&self, settings: &AppSettings) -> Result<(), StorageError> {
        if self.fail_on_save {
            return Err(StorageError("save fail".into()));
        }
        *self.saved.lock().unwrap() = Some(settings.clone());
        Ok(())
    }
}
