use chrono::NaiveDateTime;

#[derive(Clone)]
pub struct DoseRecordDto {
    pub id: String,
    pub medication_id: String,
    pub scheduled_at: NaiveDateTime,
    pub taken_at: Option<NaiveDateTime>,
}

pub struct ListDoseRecordsResponse {
    pub records: Vec<DoseRecordDto>,
}
