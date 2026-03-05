use crate::application::errors::ApplicationError;
use chrono::NaiveDateTime;

#[derive(Clone)]
pub struct DoseRecordDto {
    pub id: String,
    pub medication_id: String,
    pub scheduled_at: NaiveDateTime,
    pub taken_at: Option<NaiveDateTime>,
}

pub struct ListDoseRecordsRequest {
    pub medication_id: String,
}

pub struct ListDoseRecordsResponse {
    pub records: Vec<DoseRecordDto>,
}

pub trait ListDoseRecordsPort: Send + Sync {
    fn execute(
        &self,
        request: ListDoseRecordsRequest,
    ) -> Result<ListDoseRecordsResponse, ApplicationError>;
}
