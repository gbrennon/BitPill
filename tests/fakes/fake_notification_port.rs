use std::sync::Mutex;

use bitpill::application::errors::DeliveryError;
use bitpill::application::ports::outbound::notification_port::NotificationPort;
use bitpill::domain::entities::{dose_record::DoseRecord, medication::Medication};

pub struct FakeNotificationPort {
    calls: Mutex<Vec<String>>,
    fail: bool,
}

impl FakeNotificationPort {
    pub fn new() -> Self {
        Self {
            calls: Mutex::new(Vec::new()),
            fail: false,
        }
    }

    pub fn failing() -> Self {
        Self {
            calls: Mutex::new(Vec::new()),
            fail: true,
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
        if self.fail {
            return Err(DeliveryError("forced notification failure".into()));
        }
        self.calls
            .lock()
            .unwrap()
            .push(medication.name().to_string());
        Ok(())
    }
}
