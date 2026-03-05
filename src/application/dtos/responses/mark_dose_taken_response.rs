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
