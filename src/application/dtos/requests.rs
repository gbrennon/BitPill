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
    /// UUID string identifying the [`DoseRecord`] to mark as taken.
    ///
    /// [`DoseRecord`]: crate::domain::entities::dose_record::DoseRecord
    pub record_id: String,
    /// Optional scheduled time - used when creating a new record from a scheduled slot.
    /// When provided, the new record will be created with this scheduled_at time.
    pub scheduled_at: Option<NaiveDateTime>,
}

impl MarkDoseTakenRequest {
    pub fn new(record_id: impl Into<String>) -> Self {
        Self {
            record_id: record_id.into(),
            scheduled_at: None,
        }
    }

    pub fn new_with_schedule(record_id: impl Into<String>, scheduled_at: NaiveDateTime) -> Self {
        Self {
            record_id: record_id.into(),
            scheduled_at: Some(scheduled_at),
        }
    }
}

/// A scheduling tick request. Contains no parameters — the service uses its
/// injected [`ClockPort`] to determine the current time.
///
/// [`ClockPort`]: crate::application::ports::clock_port::ClockPort
pub struct ScheduleDoseRequest;

/// Request DTO for settings operations.
pub struct SettingsRequest {
    pub op: SettingsOperation,
}

/// Supported operations for the Settings inbound port.
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

pub struct GetSettingsRequest {}

pub struct SaveSettingsRequest {
    pub navigation_mode: String,
}

impl SaveSettingsRequest {
    pub fn new(navigation_mode: impl Into<String>) -> Self {
        Self {
            navigation_mode: navigation_mode.into(),
        }
    }
}

pub struct ReplenishMedicationStockRequest {
    pub medication_id: String,
    pub box_count: u16,
    pub pills_per_box: u16,
    pub pill_dosage_mg: u16,
}

pub struct RegisterMedicationBoxRequest {
    pub medication_id: String,
    pub name: String,
    pub pills_per_box: u16,
    pub dosage_mg: u16,
}
