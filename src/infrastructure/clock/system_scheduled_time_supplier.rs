use chrono::{Local, Timelike};

use crate::application::ports::scheduled_time_supplier_port::ScheduledTimeSupplier;
use crate::domain::value_objects::scheduled_time::ScheduledTime;

/// Derives the current [`ScheduledTime`] from the system clock via [`chrono::Local`].
pub struct SystemScheduledTimeSupplier;

impl ScheduledTimeSupplier for SystemScheduledTimeSupplier {
    fn current(&self) -> ScheduledTime {
        let now = Local::now();
        ScheduledTime::new(now.hour(), now.minute())
            .expect("system clock always returns a valid hour (0–23) and minute (0–59)")
    }
}
