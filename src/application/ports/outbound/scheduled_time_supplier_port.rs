use crate::domain::value_objects::scheduled_time::ScheduledTime;

/// Supplies the current scheduled time for a medication tick.
///
/// Inject via `Arc<dyn ScheduledTimeSupplier>` — never derive a `ScheduledTime`
/// directly inside application services.
pub trait ScheduledTimeSupplier: Send + Sync {
    /// Returns the current [`ScheduledTime`].
    fn current(&self) -> ScheduledTime;
}
