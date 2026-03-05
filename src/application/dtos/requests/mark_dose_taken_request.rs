use chrono::NaiveDateTime;

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
