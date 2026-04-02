use bitpill::{
    application::ports::notification_port::NotificationPort,
    domain::{
        entities::{dose_record::DoseRecord, medication::Medication},
        value_objects::{
            dosage::Dosage, medication_id::MedicationId, medication_name::MedicationName,
            scheduled_time::ScheduledTime,
        },
    },
    infrastructure::notifications::console_notification_adapter::ConsoleNotificationAdapter,
};
use chrono::NaiveDate;

fn make_medication() -> Medication {
    Medication::new(
        MedicationId::generate(),
        MedicationName::new("Aspirin").unwrap(),
        Dosage::new(500).unwrap(),
        vec![ScheduledTime::new(8, 0).unwrap()],
        bitpill::domain::value_objects::medication_frequency::DoseFrequency::OnceDaily,
    )
}

fn make_dose_record(medication_id: &MedicationId) -> DoseRecord {
    let scheduled_at = NaiveDate::from_ymd_opt(2025, 6, 1)
        .unwrap()
        .and_hms_opt(8, 0, 0)
        .unwrap();
    DoseRecord::new(medication_id.clone(), scheduled_at)
}

#[test]
fn notify_dose_due_returns_ok() {
    let adapter = ConsoleNotificationAdapter;
    let medication = make_medication();
    let record = make_dose_record(medication.id());

    let result = adapter.notify_dose_due(&medication, &record);

    assert!(result.is_ok());
}
