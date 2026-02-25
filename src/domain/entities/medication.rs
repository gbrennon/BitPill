use crate::domain::value_objects::{
    dosage::Dosage, medication_id::MedicationId, medication_name::MedicationName,
    scheduled_time::ScheduledTime,
};

#[derive(Debug, Clone)]
pub struct Medication {
    id: MedicationId,
    name: MedicationName,
    dosage: Dosage,
    scheduled_times: Vec<ScheduledTime>,
}

impl Medication {
    pub fn new(name: MedicationName, dosage: Dosage, scheduled_times: Vec<ScheduledTime>) -> Self {
        Self {
            id: MedicationId::new(),
            name,
            dosage,
            scheduled_times,
        }
    }

    pub fn id(&self) -> &MedicationId {
        &self.id
    }

    pub fn name(&self) -> &MedicationName {
        &self.name
    }

    pub fn dosage(&self) -> &Dosage {
        &self.dosage
    }

    pub fn scheduled_times(&self) -> &[ScheduledTime] {
        &self.scheduled_times
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::{
        dosage::Dosage, medication_name::MedicationName, scheduled_time::ScheduledTime,
    };

    fn make_medication() -> Medication {
        Medication::new(
            MedicationName::new("Aspirin").unwrap(),
            Dosage::new(500).unwrap(),
            vec![
                ScheduledTime::new(8, 0).unwrap(),
                ScheduledTime::new(20, 0).unwrap(),
            ],
        )
    }

    #[test]
    fn new_assigns_a_unique_id() {
        let med_a = make_medication();
        let med_b = make_medication();

        assert_ne!(med_a.id(), med_b.id());
    }

    #[test]
    fn name_returns_the_name_passed_to_constructor() {
        let med = make_medication();

        assert_eq!(med.name().value(), "Aspirin");
    }

    #[test]
    fn dosage_returns_the_dosage_passed_to_constructor() {
        let med = make_medication();

        assert_eq!(med.dosage().amount_mg(), 500);
    }

    #[test]
    fn scheduled_times_returns_all_times_passed_to_constructor() {
        let med = make_medication();

        assert_eq!(med.scheduled_times().len(), 2);
        assert_eq!(med.scheduled_times()[0].to_string(), "08:00");
        assert_eq!(med.scheduled_times()[1].to_string(), "20:00");
    }

    #[test]
    fn new_with_no_scheduled_times_is_allowed() {
        let med = Medication::new(
            MedicationName::new("Aspirin").unwrap(),
            Dosage::new(500).unwrap(),
            vec![],
        );

        assert!(med.scheduled_times().is_empty());
    }
}
