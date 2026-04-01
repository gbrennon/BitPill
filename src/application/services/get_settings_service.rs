use std::sync::Arc;

use crate::application::{
    dtos::{requests::GetSettingsRequest, responses::GetSettingsResponse},
    errors::{ApplicationError, NotFoundError},
    ports::{
        inbound::get_settings_port::GetSettingsPort,
        outbound::settings_repository_port::SettingsRepositoryPort,
    },
};

pub struct GetSettingsService {
    repository: Arc<dyn SettingsRepositoryPort>,
}

impl GetSettingsService {
    pub fn new(repository: Arc<dyn SettingsRepositoryPort>) -> Self {
        Self { repository }
    }
}

impl GetSettingsPort for GetSettingsService {
    fn execute(
        &self,
        _request: GetSettingsRequest,
    ) -> Result<GetSettingsResponse, ApplicationError> {
        match self.repository.load()? {
            Some(s) => Ok(GetSettingsResponse {
                navigation_mode: s.navigation_mode().as_str().to_string(),
            }),
            None => Err(ApplicationError::NotFound(NotFoundError)),
        }
    }
}
