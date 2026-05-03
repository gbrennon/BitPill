use std::sync::Arc;

use bitpill::{
    application::{
        dtos::requests::GetSettingsRequest,
        errors::ApplicationError,
        ports::{fakes::FakeSettingsRepository, inbound::get_settings_port::GetSettingsPort},
        services::get_settings_service::GetSettingsService,
    },
    domain::{
        entities::app_settings::AppSettings,
        value_objects::navigation_mode::{NavigationMode, NavigationModeVariant},
    },
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_settings_returns_settings_response_when_found() {
        let settings = AppSettings::new(NavigationMode::new(NavigationModeVariant::Vi).unwrap());
        let repo = Arc::new(FakeSettingsRepository::new(settings));
        let service = GetSettingsService::new(repo);

        let res = service
            .execute(GetSettingsRequest {})
            .expect("should return settings");

        assert_eq!(res.navigation_mode, "vi");
    }

    #[test]
    fn get_settings_returns_not_found_when_empty() {
        let repo = Arc::new(FakeSettingsRepository::empty());
        let service = GetSettingsService::new(repo);

        let res = service.execute(GetSettingsRequest {});

        assert!(matches!(res, Err(ApplicationError::NotFound(_))));
    }

    #[test]
    fn get_settings_returns_error_when_repository_fails() {
        let repo = Arc::new(FakeSettingsRepository::failing_load());
        let service = GetSettingsService::new(repo);

        let res = service.execute(GetSettingsRequest {});

        assert!(matches!(res, Err(ApplicationError::Storage(_))));
    }
}
