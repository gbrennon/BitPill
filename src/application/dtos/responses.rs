use chrono::NaiveDateTime;
use serde_json::Value;

#[derive(Clone)]
pub struct DoseRecordDto {
    pub id: String,
    pub medication_id: String,
    pub scheduled_at: NaiveDateTime,
    pub taken_at: Option<NaiveDateTime>,
}

#[derive(Clone)]
pub struct ScheduledDoseRecordDto {
    pub id: String,
    pub medication_id: String,
    pub scheduled_at: NaiveDateTime,
}

pub struct MedicationDto {
    pub id: String,
    pub name: String,
    pub amount_mg: u32,
    pub scheduled_time: Vec<(u32, u32)>,
    pub dose_frequency: String,
}

pub struct CreateDoseRecordResponse {
    pub id: String,
}

pub struct CreateMedicationResponse {
    pub id: String,
}

pub struct DeleteMedicationResponse {}

pub struct EditMedicationResponse {
    pub id: String,
}

pub struct GetMedicationResponse {
    pub medication: MedicationDto,
}

pub struct GetSettingsResponse {
    pub navigation_mode: String,
}

pub struct ListAllMedicationsResponse {
    pub medications: Vec<MedicationDto>,
}

pub struct ListDoseRecordsResponse {
    pub records: Vec<DoseRecordDto>,
}

pub struct MarkDoseTakenResponse {
    pub record_id: String,
}

impl MarkDoseTakenResponse {
    pub fn new(record_id: impl Into<String>) -> Self {
        Self {
            record_id: record_id.into(),
        }
    }
}

pub struct MarkMedicationTakenResponse {
    pub id: String,
}

impl MarkMedicationTakenResponse {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

pub struct ScheduleDoseResponse {
    pub created: Vec<ScheduledDoseRecordDto>,
}

pub struct SettingsResponse {
    pub settings: Value,
}

pub struct UpdateMedicationResponse {
    pub id: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn response_new_and_fields() {
        let raw_id = "some-id";
        let response = MarkDoseTakenResponse::new("some-id");
        assert_eq!(response.record_id, raw_id);
    }
}
