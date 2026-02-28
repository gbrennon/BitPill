use crate::application::errors::DeliveryError;
use crate::application::ports::notification_port::NotificationPort;
use crate::domain::entities::{dose_record::DoseRecord, medication::Medication};

/// Delivers dose-due notifications by printing to stdout.
pub struct ConsoleNotificationAdapter;

impl NotificationPort for ConsoleNotificationAdapter {
    fn notify_dose_due(
        &self,
        medication: &Medication,
        record: &DoseRecord,
    ) -> Result<(), DeliveryError> {
        println!(
            "[DOSE DUE] {} — {} mg scheduled at {}",
            medication.name(),
            medication.dosage().amount_mg(),
            record.scheduled_at(),
        );
        Ok(())
    }
}
