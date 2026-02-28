use chrono::Timelike;

use bitpill::application::ports::clock_port::ClockPort;
use bitpill::application::ports::scheduled_time_supplier_port::ScheduledTimeSupplier;
use bitpill::infrastructure::clock::{
    system_clock::SystemClock,
    system_scheduled_time_supplier::SystemScheduledTimeSupplier,
};

#[test]
fn system_clock_now_returns_datetime_with_zero_seconds() {
    let clock = SystemClock;

    let now = clock.now();

    assert_eq!(now.second(), 0);
    assert_eq!(now.nanosecond(), 0);
}

#[test]
fn system_scheduled_time_supplier_current_returns_valid_scheduled_time() {
    let supplier = SystemScheduledTimeSupplier;

    let time = supplier.current();

    // hour() and minute() are within valid ranges — just verify it formats
    assert!(time.to_string().contains(':'));
}
