use crate::application::{
    dtos::{requests::ScheduleDoseRequest, responses::ScheduleDoseResponse},
    errors::ApplicationError,
};

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
