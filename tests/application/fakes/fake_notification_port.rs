use std::sync::Mutex;

use crate::{
    application::{errors::DeliveryError, ports::notification_port::NotificationPort},
    domain::entities::{dose_record::DoseRecord, medication::Medication},
};

pub struct FakeNotificationPort {
    calls: Mutex<Vec<String>>,
    should_fail: bool,
}

impl Default for FakeNotificationPort {
    fn default() -> Self {
        Self::new()
    }
}

impl FakeNotificationPort {
    pub fn new() -> Self {
        Self {
            calls: Mutex::new(Vec::new()),
            should_fail: false,
        }
    }

    pub fn failing() -> Self {
        Self {
            calls: Mutex::new(Vec::new()),
            should_fail: true,
        }
    }

    pub fn call_count(&self) -> usize {
        self.calls.lock().unwrap().len()
    }
}

impl NotificationPort for FakeNotificationPort {
    fn notify_dose_due(
        &self,
        medication: &Medication,
        _record: &DoseRecord,
    ) -> Result<(), DeliveryError> {
        if self.should_fail {
            return Err(DeliveryError("simulated failure".into()));
        }
        self.calls
            .lock()
            .unwrap()
            .push(medication.name().to_string());
        Ok(())
    }
}
