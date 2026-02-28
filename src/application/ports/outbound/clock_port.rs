use chrono::NaiveDateTime;

/// Abstracts the system clock so services can be tested with a controlled time.
///
/// Inject via `Arc<dyn ClockPort>` — never call `chrono::Local::now()` directly
/// inside application services.
pub trait ClockPort: Send + Sync {
    /// Returns the current local datetime.
    fn now(&self) -> NaiveDateTime;
}
