use std::sync::Mutex;

use chrono::NaiveDateTime;

use crate::application::errors::{DeliveryError, StorageError};
use crate::application::ports::{
    clock_port::ClockPort,
    dose_record_repository_port::DoseRecordRepository,
    medication_repository_port::MedicationRepository,
    notification_port::NotificationPort,
};
use crate::domain::{
    entities::{dose_record::DoseRecord, medication::Medication},
    value_objects::{dose_record_id::DoseRecordId, medication_id::MedicationId},
};

// ── FakeClock ─────────────────────────────────────────────────────────────────

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

// ── FakeMedicationRepository ──────────────────────────────────────────────────

pub struct FakeMedicationRepository {
    medications: Mutex<Vec<Medication>>,
    fail_on_save: bool,
}

impl FakeMedicationRepository {
    pub fn new() -> Self {
        Self {
            medications: Mutex::new(Vec::new()),
            fail_on_save: false,
        }
    }

    pub fn with(medications: Vec<Medication>) -> Self {
        Self {
            medications: Mutex::new(medications),
            fail_on_save: false,
        }
    }

    pub fn failing() -> Self {
        Self {
            medications: Mutex::new(Vec::new()),
            fail_on_save: true,
        }
    }

    pub fn saved_count(&self) -> usize {
        self.medications.lock().unwrap().len()
    }
}

impl MedicationRepository for FakeMedicationRepository {
    fn save(&self, medication: &Medication) -> Result<(), StorageError> {
        if self.fail_on_save {
            return Err(StorageError("forced failure".into()));
        }
        self.medications.lock().unwrap().push(medication.clone());
        Ok(())
    }

    fn find_by_id(&self, id: &MedicationId) -> Result<Option<Medication>, StorageError> {
        Ok(self
            .medications
            .lock()
            .unwrap()
            .iter()
            .find(|m| m.id() == id)
            .cloned())
    }

    fn find_all(&self) -> Result<Vec<Medication>, StorageError> {
        Ok(self.medications.lock().unwrap().clone())
    }

    fn delete(&self, _id: &MedicationId) -> Result<(), StorageError> {
        Ok(())
    }
}

// ── FakeDoseRecordRepository ──────────────────────────────────────────────────

pub struct FakeDoseRecordRepository {
    records: Mutex<Vec<DoseRecord>>,
}

impl FakeDoseRecordRepository {
    pub fn new() -> Self {
        Self {
            records: Mutex::new(Vec::new()),
        }
    }

    pub fn with(record: DoseRecord) -> Self {
        Self {
            records: Mutex::new(vec![record]),
        }
    }

    pub fn saved_count(&self) -> usize {
        self.records.lock().unwrap().len()
    }
}

impl DoseRecordRepository for FakeDoseRecordRepository {
    fn save(&self, record: &DoseRecord) -> Result<(), StorageError> {
        let mut records = self.records.lock().unwrap();
        if let Some(existing) = records.iter_mut().find(|r| r.id() == record.id()) {
            *existing = record.clone();
        } else {
            records.push(record.clone());
        }
        Ok(())
    }

    fn find_by_id(&self, id: &DoseRecordId) -> Result<Option<DoseRecord>, StorageError> {
        Ok(self
            .records
            .lock()
            .unwrap()
            .iter()
            .find(|r| r.id() == id)
            .cloned())
    }

    fn find_all_by_medication(
        &self,
        medication_id: &MedicationId,
    ) -> Result<Vec<DoseRecord>, StorageError> {
        Ok(self
            .records
            .lock()
            .unwrap()
            .iter()
            .filter(|r| r.medication_id() == medication_id)
            .cloned()
            .collect())
    }

    fn delete(&self, _id: &DoseRecordId) -> Result<(), StorageError> {
        Ok(())
    }
}

// ── FakeNotificationPort ──────────────────────────────────────────────────────

pub struct FakeNotificationPort {
    calls: Mutex<Vec<String>>,
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
