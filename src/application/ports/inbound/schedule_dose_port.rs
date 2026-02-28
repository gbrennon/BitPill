use chrono::NaiveDateTime;

use crate::application::errors::ApplicationError;

/// Data transfer object for a dose record created during a scheduling tick.
pub struct DoseRecordDto {
    pub id: String,
    pub medication_id: String,
    pub scheduled_at: NaiveDateTime,
}

/// A scheduling tick request. Contains no parameters — the service uses its
/// injected [`ClockPort`] to determine the current time.
///
/// [`ClockPort`]: crate::application::ports::clock_port::ClockPort
pub struct ScheduleDoseRequest;

pub struct ScheduleDoseResponse {
    /// All dose records created during this tick.
    pub created: Vec<DoseRecordDto>,
}

pub trait ScheduleDosePort: Send + Sync {
    fn execute(
        &self,
        request: ScheduleDoseRequest,
    ) -> Result<ScheduleDoseResponse, ApplicationError>;
}
