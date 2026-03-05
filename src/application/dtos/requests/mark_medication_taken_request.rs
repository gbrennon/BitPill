use chrono::NaiveDateTime;

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
