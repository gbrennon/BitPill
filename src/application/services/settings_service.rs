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
