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

impl MarkDoseTakenResponse {
    pub fn new(record_id: impl Into<String>) -> Self {
        Self {
            record_id: record_id.into(),
        }
    }
}

pub trait MarkDoseTakenPort: Send + Sync {
    fn execute(
        &self,
        request: MarkDoseTakenRequest,
    ) -> Result<MarkDoseTakenResponse, ApplicationError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn request_and_response_new_and_fields() {
        let dt = NaiveDate::from_ymd_opt(2026, 3, 5).unwrap().and_hms_opt(12, 0, 0).unwrap();
        let req = MarkDoseTakenRequest::new("some-id", dt);
        assert_eq!(req.record_id, "some-id");
        assert_eq!(req.taken_at, dt);
        let res = MarkDoseTakenResponse::new("rid");
        assert_eq!(res.record_id, "rid");
    }
}
