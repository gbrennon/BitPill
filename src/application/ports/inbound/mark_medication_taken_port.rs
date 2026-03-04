use chrono::NaiveDateTime;

use crate::application::errors::ApplicationError;

/// Request to mark a medication dose as taken by creating a DoseRecord immediately.
pub struct MarkMedicationTakenRequest {
    pub medication_id: String,
    pub taken_at: NaiveDateTime,
}

impl MarkMedicationTakenRequest {
    pub fn new(medication_id: impl Into<String>, taken_at: NaiveDateTime) -> Self {
        Self {
            medication_id: medication_id.into(),
            taken_at,
        }
    }
}

pub struct MarkMedicationTakenResponse {
    pub id: String,
}

impl MarkMedicationTakenResponse {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

pub trait MarkMedicationTakenPort: Send + Sync {
    fn execute(
        &self,
        request: MarkMedicationTakenRequest,
    ) -> Result<MarkMedicationTakenResponse, ApplicationError>;
}
