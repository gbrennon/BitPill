use chrono::NaiveDateTime;
use serde_json::Value;

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

pub struct CreateMedicationRequest {
    pub name: String,
    pub amount_mg: u32,
    pub scheduled_time: Vec<(u32, u32)>,
    pub dose_frequency: String,
}

impl CreateMedicationRequest {
    pub fn new(
        name: impl Into<String>,
        amount_mg: u32,
        scheduled_time: Vec<(u32, u32)>,
        dose_frequency: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            amount_mg,
            scheduled_time,
            dose_frequency: dose_frequency.into(),
        }
    }
}

pub struct DeleteMedicationRequest {
    pub id: String,
}

pub struct EditMedicationRequest {
    pub id: String,
    pub name: String,
    pub amount_mg: u32,
    pub scheduled_time: Vec<(u32, u32)>,
    pub dose_frequency: String,
}

impl EditMedicationRequest {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        amount_mg: u32,
        scheduled_time: Vec<(u32, u32)>,
        dose_frequency: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            amount_mg,
            scheduled_time,
            dose_frequency: dose_frequency.into(),
        }
    }
}

pub struct GetMedicationRequest {
    pub id: String,
}

pub struct ListAllMedicationsRequest;

pub struct ListDoseRecordsRequest {
    pub medication_id: String,
}

pub struct MarkDoseTakenRequest {
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

pub struct ScheduleDoseRequest;

pub struct SettingsRequest {
    pub op: SettingsOperation,
}

pub enum SettingsOperation {
    Get,
    Update { settings: Value },
}

pub struct UpdateMedicationRequest {
    pub id: String,
    pub name: String,
    pub amount_mg: u32,
    pub scheduled_time: Vec<(u32, u32)>,
    pub dose_frequency: String,
}

impl UpdateMedicationRequest {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        amount_mg: u32,
        scheduled_time: Vec<(u32, u32)>,
        dose_frequency: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            amount_mg,
            scheduled_time,
            dose_frequency: dose_frequency.into(),
        }
    }
}
