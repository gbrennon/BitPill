use chrono::NaiveDateTime;

pub struct MarkDoseTakenRequest {
    /// UUID string identifying the [`DoseRecord`] to mark as taken.
    ///
    /// [`DoseRecord`]: crate::domain::entities::dose_record::DoseRecord
    pub record_id: String,
    /// Optional scheduled time - used when creating a new record from a scheduled slot.
    /// When provided, the new record will be created with this scheduled_at time.
    pub scheduled_at: Option<NaiveDateTime>,
}

impl MarkDoseTakenRequest {
    pub fn new(record_id: impl Into<String>) -> Self {
        Self {
            record_id: record_id.into(),
            scheduled_at: None,
        }
    }

    pub fn new_with_schedule(record_id: impl Into<String>, scheduled_at: NaiveDateTime) -> Self {
        Self {
            record_id: record_id.into(),
            scheduled_at: Some(scheduled_at),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn new() {
        let id = "foo-id";

        let request = MarkDoseTakenRequest::new(id);

        let expected_id = "foo-id";
        assert_eq!(expected_id, request.record_id);
        assert!(request.scheduled_at.is_none());
    }

    #[test]
    fn new_with_schedule() {
        let id = "foo-id";
        let scheduled = NaiveDate::from_ymd_opt(2025, 1, 1)
            .unwrap()
            .and_hms_opt(8, 0, 0)
            .unwrap();

        let request = MarkDoseTakenRequest::new_with_schedule(id, scheduled);

        assert_eq!(request.record_id, "foo-id");
        assert_eq!(request.scheduled_at, Some(scheduled));
    }
}
