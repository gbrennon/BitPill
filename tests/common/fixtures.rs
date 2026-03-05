use bitpill::domain::{
    entities::medication::Medication,
    value_objects::{
        dosage::Dosage,
        medication_frequency::DoseFrequency,
        medication_id::MedicationId,
        medication_name::MedicationName,
        scheduled_time::ScheduledTime,
    },
};

/// Builds a valid `Medication` with the given name and `amount_mg`, scheduled once at 08:00.
pub fn medication(name: &str, amount_mg: u32) -> Medication {
    Medication::new(
        MedicationId::generate(),
        MedicationName::new(name).unwrap(),
        Dosage::new(amount_mg).unwrap(),
        vec![ScheduledTime::new(8, 0).unwrap()],
        DoseFrequency::OnceDaily,
    )
}

/// Builds a `Medication` scheduled at the given hour and minute.
#[allow(dead_code)]
pub fn medication_at(name: &str, hour: u32, minute: u32) -> Medication {
    Medication::new(
        MedicationId::generate(),
        MedicationName::new(name).unwrap(),
        Dosage::new(500).unwrap(),
        vec![ScheduledTime::new(hour, minute).unwrap()],
        DoseFrequency::OnceDaily,
    )
}
