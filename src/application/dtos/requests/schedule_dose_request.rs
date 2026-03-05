/// A scheduling tick request. Contains no parameters — the service uses its
/// injected [`ClockPort`] to determine the current time.
///
/// [`ClockPort`]: crate::application::ports::clock_port::ClockPort
pub struct ScheduleDoseRequest;
