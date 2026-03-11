use std::sync::Mutex;

use crate::application::errors::DeliveryError;
use crate::application::ports::notification_port::NotificationPort;
use crate::domain::entities::{dose_record::DoseRecord, medication::Medication};

pub struct FakeNotificationPort {
    calls: Mutex<Vec<String>>,
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
        self.calls
            .lock()
            .unwrap()
            .push(medication.name().to_string());
        Ok(())
    }
}
