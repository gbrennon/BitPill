use chrono::NaiveDateTime;

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
