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

struct SaveSettingsService {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        application::ports::fakes::FakeSettingsRepository,
        domain::value_objects::navigation_mode::NavigationModeVariant,
    };

    #[test]
    fn save_settings_returns_response_with_navigation_mode_on_success() {
        let repo = Arc::new(FakeSettingsRepository::new(AppSettings::new(
            NavigationMode::new(NavigationModeVariant::Vi).unwrap(),
        )));
        let service = SaveSettingsService::new(repo);

        let res = service
            .execute(SaveSettingsRequest::new("vi"))
            .expect("should return settings");

        assert_eq!(res.navigation_mode, "vi");
    }

    #[test]
    fn save_settings_returns_invalid_navigation_mode_error_for_invalid_input() {
        let repo = Arc::new(FakeSettingsRepository::empty());
        let service = SaveSettingsService::new(repo);

        let res = service.execute(SaveSettingsRequest::new("invalid_mode"));

        assert!(matches!(res, Err(ApplicationError::Domain(_))));
    }

    #[test]
    fn save_settings_returns_storage_error_when_repository_fails() {
        let repo = Arc::new(FakeSettingsRepository::failing_save(AppSettings::new(
            NavigationMode::new(NavigationModeVariant::Vi).unwrap(),
        )));
        let service = SaveSettingsService::new(repo);

        let res = service.execute(SaveSettingsRequest::new("vi"));

        assert!(matches!(res, Err(ApplicationError::Storage(_))));
    }
}
