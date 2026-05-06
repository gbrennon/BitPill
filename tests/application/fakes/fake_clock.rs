use chrono::NaiveDateTime;

use crate::application::ports::clock_port::ClockPort;

pub struct FakeClock {
    pub datetime: NaiveDateTime,
}

impl FakeClock {
    pub fn at(hour: u32, minute: u32) -> Self {
        use chrono::NaiveDate;
        Self {
            datetime: NaiveDate::from_ymd_opt(2025, 6, 1)
                .unwrap()
                .and_hms_opt(hour, minute, 0)
                .unwrap(),
        }
    }
}

impl ClockPort for FakeClock {
    fn now(&self) -> NaiveDateTime {
        self.datetime
    }
}
