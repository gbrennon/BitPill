use chrono::NaiveDateTime;

use crate::application::errors::ApplicationError;

pub struct MarkDoseTakenRequest {
    /// UUID string identifying the [`DoseRecord`] to mark as taken.
    ///
    /// [`DoseRecord`]: crate::domain::entities::dose_record::DoseRecord
    pub record_id: String,
    pub taken_at: NaiveDateTime,
}

impl MarkDoseTakenRequest {
    pub fn new(record_id: impl Into<String>, taken_at: NaiveDateTime) -> Self {
        Self {
            record_id: record_id.into(),
            taken_at,
        }
    }
}

pub struct MarkDoseTakenResponse {
    /// UUID string of the dose record that was marked as taken.
    pub record_id: String,
}

pub trait MarkDoseTakenPort: Send + Sync {
    fn execute(
        &self,
        request: MarkDoseTakenRequest,
    ) -> Result<MarkDoseTakenResponse, ApplicationError>;
}
