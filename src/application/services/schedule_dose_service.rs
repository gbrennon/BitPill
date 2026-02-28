use std::sync::Arc;

use chrono::Timelike;

use crate::application::errors::ApplicationError;
use crate::application::ports::{
    clock_port::ClockPort,
    dose_record_repository_port::DoseRecordRepository,
    medication_repository_port::MedicationRepository,
    notification_port::NotificationPort,
    schedule_dose_port::{
        DoseRecordDto, ScheduleDosePort, ScheduleDoseRequest, ScheduleDoseResponse,
    },
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
                .scheduled_times()
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
    use crate::application::ports::fakes::{
        FakeClock, FakeDoseRecordRepository, FakeMedicationRepository, FakeNotificationPort,
    };
    use crate::domain::{
        entities::medication::Medication,
        value_objects::{
            dosage::Dosage, medication_id::MedicationId, medication_name::MedicationName,
            scheduled_time::ScheduledTime,
        },
    };

    fn make_medication(name: &str, hour: u32, minute: u32) -> Medication {
        Medication::new(
            MedicationId::generate(),
            MedicationName::new(name).unwrap(),
            Dosage::new(500).unwrap(),
            vec![ScheduledTime::new(hour, minute).unwrap()],
        )
    }

    fn make_service(
        medications: Vec<Medication>,
        clock: FakeClock,
    ) -> (
        ScheduleDoseService,
        Arc<FakeDoseRecordRepository>,
        Arc<FakeNotificationPort>,
    ) {
        let dose_repo = Arc::new(FakeDoseRecordRepository::new());
        let notif = Arc::new(FakeNotificationPort::new());
        let service = ScheduleDoseService::new(
            Arc::new(FakeMedicationRepository::with(medications)),
            dose_repo.clone(),
            notif.clone(),
            Arc::new(clock),
        );
        (service, dose_repo, notif)
    }

    // ── Tests ─────────────────────────────────────────────────────────────

    #[test]
    fn execute_with_no_medications_returns_empty_vec() {
        let (service, _, _) = make_service(vec![], FakeClock::at(8, 0));

        let result = service.execute().unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn execute_with_matching_time_creates_dose_record_and_notifies() {
        let medication = make_medication("Aspirin", 8, 0);
        let (service, dose_repo, notif) = make_service(vec![medication], FakeClock::at(8, 0));

        let result = service.execute().unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(dose_repo.saved_count(), 1);
        assert_eq!(notif.call_count(), 1);
    }

    #[test]
    fn execute_with_non_matching_time_creates_no_records() {
        let medication = make_medication("Aspirin", 8, 0);
        let (service, dose_repo, notif) = make_service(vec![medication], FakeClock::at(20, 0));

        let result = service.execute().unwrap();

        assert!(result.is_empty());
        assert_eq!(dose_repo.saved_count(), 0);
        assert_eq!(notif.call_count(), 0);
    }

    #[test]
    fn execute_notifies_only_medications_due_at_current_time() {
        let aspirin = make_medication("Aspirin", 8, 0);
        let ibuprofen = make_medication("Ibuprofen", 20, 0);
        let (service, dose_repo, notif) =
            make_service(vec![aspirin, ibuprofen], FakeClock::at(8, 0));

        let result = service.execute().unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(dose_repo.saved_count(), 1);
        assert_eq!(notif.call_count(), 1);
    }

    #[test]
    fn execute_notifies_all_medications_due_at_same_time() {
        let med_a = make_medication("Aspirin", 8, 0);
        let med_b = make_medication("Ibuprofen", 8, 0);
        let (service, dose_repo, notif) = make_service(vec![med_a, med_b], FakeClock::at(8, 0));

        let result = service.execute().unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(dose_repo.saved_count(), 2);
        assert_eq!(notif.call_count(), 2);
    }

    #[test]
    fn execute_created_record_links_to_correct_medication() {
        let medication = make_medication("Aspirin", 8, 0);
        let medication_id = medication.id().clone();
        let (service, _, _) = make_service(vec![medication], FakeClock::at(8, 0));

        let records = service.execute().unwrap();

        assert_eq!(records[0].medication_id(), &medication_id);
    }

    #[test]
    fn execute_created_record_scheduled_at_matches_clock_now() {
        let medication = make_medication("Aspirin", 8, 0);
        let clock = FakeClock::at(8, 0);
        let expected_now = clock.datetime;
        let (service, _, _) = make_service(vec![medication], clock);

        let records = service.execute().unwrap();

        assert_eq!(records[0].scheduled_at(), expected_now);
    }

    #[test]
    fn execute_medication_with_no_scheduled_times_is_ignored() {
        let medication = Medication::new(
            MedicationId::generate(),
            MedicationName::new("On-demand").unwrap(),
            Dosage::new(100).unwrap(),
            vec![],
        );
        let (service, dose_repo, notif) = make_service(vec![medication], FakeClock::at(8, 0));

        let result = service.execute().unwrap();

        assert!(result.is_empty());
        assert_eq!(dose_repo.saved_count(), 0);
        assert_eq!(notif.call_count(), 0);
    }
}
