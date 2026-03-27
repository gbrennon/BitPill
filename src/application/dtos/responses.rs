use chrono::NaiveDateTime;
use serde_json::Value;

#[derive(Debug, PartialEq)]
pub struct CreateDoseRecordResponse {
    pub id: String,
}

impl CreateDoseRecordResponse {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_return_self() {
        let id = "id1234";

        let actual_response = CreateDoseRecordResponse::new(id);

        let expected_response = CreateDoseRecordResponse { id: id.into() };
        assert_eq!(expected_response, actual_response);
    }
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

/// Data transfer object representing a medication returned from the list query.
pub struct MedicationDto {
    pub id: String,
    pub name: String,
    pub amount_mg: u32,
    pub scheduled_time: Vec<(u32, u32)>,
    pub dose_frequency: String,
    pub taken_today: usize,
    pub scheduled_today: usize,
}

pub struct ListAllMedicationsResponse {
    pub medications: Vec<MedicationDto>,
}

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

#[cfg(test)]
mod mark_dose_taken_test {
    use super::*;

    #[test]
    fn response_new_and_fields() {
        let raw_id = "some-id";

        let response = MarkDoseTakenResponse::new("some-id");

        assert_eq!(response.record_id, raw_id);
    }
}

/// Data transfer object for a dose record created during a scheduling tick.
#[derive(Clone)]
pub struct ScheduledDoseRecordDto {
    pub id: String,
    pub medication_id: String,
    pub scheduled_at: NaiveDateTime,
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
