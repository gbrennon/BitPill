use std::sync::Arc;

use crate::application::errors::ApplicationError;
use crate::application::ports::inbound::settings_port::{
    SettingsOperation, SettingsPort, SettingsRequest, SettingsResponse,
};
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
                // validate? just persist as-is
                self.repository.save(&settings)?;
                Ok(SettingsResponse { settings })
            }
        }
    }
}
