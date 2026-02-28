use chrono::{Local, Timelike};

use crate::application::ports::clock_port::ClockPort;

/// Provides the real system time via [`chrono::Local`].
///
/// Seconds and nanoseconds are zeroed so the returned datetime represents
/// the current minute rather than the precise instant.
pub struct SystemClock;

impl ClockPort for SystemClock {
    fn now(&self) -> chrono::NaiveDateTime {
        let now = Local::now().naive_local();
        now.with_second(0)
            .and_then(|dt| dt.with_nanosecond(0))
            .expect("zeroing seconds on a valid NaiveDateTime always succeeds")
    }
}
