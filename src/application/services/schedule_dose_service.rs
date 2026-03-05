use std::sync::Arc;

use chrono::Timelike;

use crate::application::dtos::requests::ScheduleDoseRequest;
use crate::application::dtos::responses::schedule_dose_response::{DoseRecordDto, ScheduleDoseResponse};
use crate::application::errors::ApplicationError;
use crate::application::ports::{
    clock_port::ClockPort,
    dose_record_repository_port::DoseRecordRepository,
    medication_repository_port::MedicationRepository,
    notification_port::NotificationPort,
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
