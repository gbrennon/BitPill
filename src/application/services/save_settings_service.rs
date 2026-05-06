use std::sync::Arc;

use crate::{
    application::{
        dtos::{requests::SaveSettingsRequest, responses::SaveSettingsResponse},
        errors::ApplicationError,
        ports::{
            save_settings_port::SaveSettingsPort, settings_repository_port::SettingsRepositoryPort,
        },
    },
    domain::{entities::app_settings::AppSettings, value_objects::navigation_mode::NavigationMode},
};

pub struct SaveSettingsService {
    repository: Arc<dyn SettingsRepositoryPort>,
}

impl SaveSettingsService {
    pub fn new(repository: Arc<dyn SettingsRepositoryPort>) -> Self {
        Self { repository }
    }
}

impl SaveSettingsPort for SaveSettingsService {
    fn execute(
        &self,
        request: SaveSettingsRequest,
    ) -> Result<SaveSettingsResponse, ApplicationError> {
        let navigation_mode = NavigationMode::try_from(request.navigation_mode.as_str())?;
        let settings = AppSettings::new(navigation_mode);
        self.repository.save(&settings)?;

        Ok(SaveSettingsResponse {
            navigation_mode: settings.navigation_mode().as_str().to_string(),
        })
    }
}
