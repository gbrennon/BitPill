use chrono::{Local, Timelike};

use crate::{
    application::ports::scheduled_time_supplier_port::ScheduledTimeSupplier,
    domain::value_objects::scheduled_time::ScheduledTime,
};

/// Derives the current [`ScheduledTime`] from the system clock via [`chrono::Local`].
pub struct SystemScheduledTimeSupplier;

impl ScheduledTimeSupplier for SystemScheduledTimeSupplier {
    fn current(&self) -> ScheduledTime {
        let now = Local::now();
        ScheduledTime::new(now.hour(), now.minute())
            .expect("system clock always returns a valid hour (0–23) and minute (0–59)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_returns_valid_hour_and_minute() {
        let sup = SystemScheduledTimeSupplier;
        let st = sup.current();
        assert!(st.hour() <= 23);
        assert!(st.minute() <= 59);
    }
}
