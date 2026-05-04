use std::sync::Arc;

use bitpill::{
    application::{
        dtos::requests::SaveSettingsRequest,
        errors::ApplicationError,
        ports::{
            save_settings_port::SaveSettingsPort, settings_repository_port::SettingsRepositoryPort,
        },
        services::save_settings_service::SaveSettingsService,
    },
    domain::{
        entities::app_settings::AppSettings,
        value_objects::navigation_mode::{NavigationMode, NavigationModeVariant},
    },
};

use crate::fakes::FakeSettingsRepository;

#[cfg(test)]
mod tests {
    use super::*;

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
