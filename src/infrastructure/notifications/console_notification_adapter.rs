use crate::{
    application::{errors::DeliveryError, ports::notification_port::NotificationPort},
    domain::entities::{dose_record::DoseRecord, medication::Medication},
};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        entities::{dose_record::DoseRecord, medication::Medication},
        value_objects::{
            dosage::Dosage, medication_frequency::DoseFrequency, medication_id::MedicationId,
            medication_name::MedicationName, scheduled_time::ScheduledTime,
        },
    };

    #[test]
    fn notify_dose_due_returns_ok() {
        let med = Medication::new(
            MedicationId::generate(),
            MedicationName::new("TestMed").unwrap(),
            Dosage::new(10).unwrap(),
            vec![ScheduledTime::new(8, 0).unwrap()],
            DoseFrequency::OnceDaily,
        );
        let scheduled_at = chrono::NaiveDate::from_ymd_opt(2025, 1, 1)
            .unwrap()
            .and_hms_opt(8, 0, 0)
            .unwrap();
        let record = DoseRecord::new(med.id().clone(), scheduled_at);
        let adapter = ConsoleNotificationAdapter;
        let res = adapter.notify_dose_due(&med, &record);
        assert!(res.is_ok());
    }
}
