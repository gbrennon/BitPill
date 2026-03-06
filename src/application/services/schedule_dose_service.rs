use std::sync::Arc;

use chrono::Timelike;

use crate::application::dtos::requests::ScheduleDoseRequest;
use crate::application::dtos::responses::schedule_dose_response::{
    DoseRecordDto, ScheduleDoseResponse,
};
use crate::application::errors::ApplicationError;
use crate::application::ports::{
    clock_port::ClockPort, dose_record_repository_port::DoseRecordRepository,
    medication_repository_port::MedicationRepository, notification_port::NotificationPort,
    schedule_dose_port::ScheduleDosePort,
};
use crate::domain::entities::dose_record::DoseRecord;

/// Checks every registered medication against the current time and, for each
/// one whose [`ScheduledTime`] matches, creates a [`DoseRecord`] and fires a
/// notification.
///
/// Call `execute()` once per minute (e.g. from a recurring timer) to drive
/// the reminder flow.
///
/// # Example
///
/// ```no_run
/// // In tests, inject fakes for full isolation (see the #[cfg(test)] module below).
/// // In production, wire the service via the composition root.
/// ```
pub struct ScheduleDoseService {
    medication_repository: Arc<dyn MedicationRepository>,
    dose_record_repository: Arc<dyn DoseRecordRepository>,
    notification_port: Arc<dyn NotificationPort>,
    clock: Arc<dyn ClockPort>,
}

impl ScheduleDoseService {
    pub fn new(
        medication_repository: Arc<dyn MedicationRepository>,
        dose_record_repository: Arc<dyn DoseRecordRepository>,
        notification_port: Arc<dyn NotificationPort>,
        clock: Arc<dyn ClockPort>,
    ) -> Self {
        Self {
            medication_repository,
            dose_record_repository,
            notification_port,
            clock,
        }
    }

    /// Runs one scheduling tick.
    ///
    /// Returns the [`DoseRecord`]s created during this tick (one per medication
    /// whose scheduled time matched the current minute).
    pub fn execute(&self) -> Result<Vec<DoseRecord>, ApplicationError> {
        let now = self.clock.now();

        let medications = self.medication_repository.find_all()?;
        let mut created = Vec::new();

        for medication in &medications {
            let is_due = medication
                .scheduled_time()
                .iter()
                .any(|t| t.hour() == now.hour() && t.minute() == now.minute());

            if is_due {
                let record = DoseRecord::new(medication.id().clone(), now);
                self.dose_record_repository.save(&record)?;
                self.notification_port
                    .notify_dose_due(medication, &record)?;
                created.push(record);
            }
        }

        Ok(created)
    }
}

impl ScheduleDosePort for ScheduleDoseService {
    fn execute(
        &self,
        _request: ScheduleDoseRequest,
    ) -> Result<ScheduleDoseResponse, ApplicationError> {
        let records = self.execute()?;
        let created = records
            .into_iter()
            .map(|r| DoseRecordDto {
                id: r.id().to_string(),
                medication_id: r.medication_id().to_string(),
                scheduled_at: r.scheduled_at(),
            })
            .collect();
        Ok(ScheduleDoseResponse { created })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::fakes::{FakeClock, FakeDoseRecordRepository, FakeMedicationRepository, FakeNotificationPort};
    use crate::domain::entities::medication::Medication;
    use crate::domain::value_objects::{medication_id::MedicationId, medication_name::MedicationName, dosage::Dosage, scheduled_time::ScheduledTime, medication_frequency::DoseFrequency};

    fn make_service(
        med_repo: std::sync::Arc<FakeMedicationRepository>,
        dose_repo: std::sync::Arc<FakeDoseRecordRepository>,
        notification: std::sync::Arc<FakeNotificationPort>,
        clock: std::sync::Arc<FakeClock>,
    ) -> ScheduleDoseService {
        ScheduleDoseService::new(med_repo, dose_repo, notification, clock)
    }

    #[test]
    fn execute_creates_records_and_sends_notifications_when_due() {
        let med = Medication::new(
            MedicationId::generate(),
            MedicationName::new("Test").unwrap(),
            Dosage::new(100).unwrap(),
            vec![ScheduledTime::new(8,0).unwrap()],
            DoseFrequency::OnceDaily,
        );
        let med_repo = std::sync::Arc::new(FakeMedicationRepository::with(vec![med.clone()]));
        let dose_repo = std::sync::Arc::new(FakeDoseRecordRepository::new());
        let notification = std::sync::Arc::new(FakeNotificationPort::new());
        let clock = std::sync::Arc::new(FakeClock::at(8,0));

        let service = make_service(med_repo.clone(), dose_repo.clone(), notification.clone(), clock.clone());
        let created = service.execute().unwrap();
        assert_eq!(created.len(), 1);
        assert_eq!(notification.call_count(), 1);
        assert_eq!(dose_repo.saved_count(), 1);
    }

    #[test]
    fn execute_returns_empty_when_not_due() {
        let med = Medication::new(
            MedicationId::generate(),
            MedicationName::new("Test").unwrap(),
            Dosage::new(100).unwrap(),
            vec![ScheduledTime::new(9,0).unwrap()],
            DoseFrequency::OnceDaily,
        );
        let med_repo = std::sync::Arc::new(FakeMedicationRepository::with(vec![med]));
        let dose_repo = std::sync::Arc::new(FakeDoseRecordRepository::new());
        let notification = std::sync::Arc::new(FakeNotificationPort::new());
        let clock = std::sync::Arc::new(FakeClock::at(8,0));

        let service = make_service(med_repo, dose_repo, notification, clock);
        let created = service.execute().unwrap();
        assert!(created.is_empty());
    }
}
