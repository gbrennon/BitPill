use chrono::NaiveDateTime;

/// Data transfer object for a dose record created during a scheduling tick.
#[derive(Clone)]
pub struct DoseRecordDto {
    pub id: String,
    pub medication_id: String,
    pub scheduled_at: NaiveDateTime,
}

pub struct ScheduleDoseResponse {
    /// All dose records created during this tick.
    pub created: Vec<DoseRecordDto>,
}
