use crate::application::dtos::requests::ScheduleDoseRequest;
use crate::application::dtos::responses::schedule_dose_response::ScheduleDoseResponse;
use crate::application::errors::ApplicationError;

/// A scheduling tick port. The service uses its injected [`ClockPort`] to determine
/// the current time.
///
/// [`ClockPort`]: crate::application::ports::clock_port::ClockPort
pub trait ScheduleDosePort: Send + Sync {
    fn execute(
        &self,
        request: ScheduleDoseRequest,
    ) -> Result<ScheduleDoseResponse, ApplicationError>;
}
