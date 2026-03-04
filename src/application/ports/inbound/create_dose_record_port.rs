use chrono::NaiveDateTime;

use crate::application::errors::ApplicationError;

pub struct CreateDoseRecordRequest {
    pub medication_id: String,
    pub scheduled_at: NaiveDateTime,
}

impl CreateDoseRecordRequest {
    pub fn new(medication_id: impl Into<String>, scheduled_at: NaiveDateTime) -> Self {
        Self {
            medication_id: medication_id.into(),
            scheduled_at,
        }
    }
}

pub struct CreateDoseRecordResponse {
    pub id: String,
}

pub trait CreateDoseRecordPort: Send + Sync {
    fn execute(&self, request: CreateDoseRecordRequest) -> Result<CreateDoseRecordResponse, ApplicationError>;
}
