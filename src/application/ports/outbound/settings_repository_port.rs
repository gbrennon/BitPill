use crate::application::errors::StorageError;
use crate::domain::entities::app_settings::AppSettings;

pub trait SettingsRepositoryPort: Send + Sync {
    fn load(&self) -> Result<Option<AppSettings>, StorageError>;
    fn save(&self, settings: &AppSettings) -> Result<(), StorageError>;
}
