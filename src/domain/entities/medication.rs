use crate::domain::value_objects::{
    dosage::Dosage, medication_id::MedicationId, medication_name::MedicationName,
    scheduled_time::ScheduledTime,
};

/// An aggregate root representing a medication prescribed to a patient.
///
/// `Medication` groups together everything that defines *what* is to be taken:
/// an identity, a human-readable name, a dosage, and an optional list of
/// scheduled administration times.
///
/// # Invariants
///
/// - `id` is supplied by the caller — use [`MedicationId::create`] to generate
///   a fresh UUID v7, or [`MedicationId::from_uuid`] to reconstitute one from
///   storage.
/// - `name` and `dosage` are validated value objects; they can only hold
///   legal values (non-empty name, non-zero dosage).
/// - `scheduled_times` may be empty (unscheduled medication is allowed).
///
/// # Examples
///
/// ```rust
/// use bitpill::domain::{
///     entities::medication::Medication,
///     value_objects::{
///         dosage::Dosage,
///         medication_id::MedicationId,
///         medication_name::MedicationName,
///         scheduled_time::ScheduledTime,
///     },
/// };
///
/// let medication = Medication::new(
///     MedicationId::create(),
///     MedicationName::new("Ibuprofen").unwrap(),
///     Dosage::new(400).unwrap(),
///     vec![ScheduledTime::new(8, 0).unwrap(), ScheduledTime::new(20, 0).unwrap()],
/// );
///
/// assert_eq!(medication.name().value(), "Ibuprofen");
/// assert_eq!(medication.dosage().amount_mg(), 400);
/// assert_eq!(medication.scheduled_times().len(), 2);
/// ```
#[derive(Debug, Clone)]
pub struct Medication {
    id: MedicationId,
    name: MedicationName,
    dosage: Dosage,
    scheduled_times: Vec<ScheduledTime>,
}

impl Medication {
    /// Creates a new `Medication` with the supplied `id`.
    ///
    /// Use [`MedicationId::create`] to generate a fresh identifier, or
    /// [`MedicationId::from_uuid`] to reconstitute one from storage.
    /// `name` and `dosage` are pre-validated value objects.
    /// `scheduled_times` may be an empty `Vec` for unscheduled medications.
    pub fn new(
        id: MedicationId,
        name: MedicationName,
        dosage: Dosage,
        scheduled_times: Vec<ScheduledTime>,
    ) -> Self {
        Self {
            id,
            name,
            dosage,
            scheduled_times,
        }
    }

    /// Returns the unique identifier of this medication.
    pub fn id(&self) -> &MedicationId {
        &self.id
    }

    /// Returns the medication's name.
    pub fn name(&self) -> &MedicationName {
        &self.name
    }

    /// Returns the prescribed dosage.
    pub fn dosage(&self) -> &Dosage {
        &self.dosage
    }

    /// Returns the list of scheduled administration times.
    ///
    /// An empty slice means the medication has no fixed schedule.
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
            MedicationId::create(),
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
            MedicationId::create(),
            MedicationName::new("Aspirin").unwrap(),
            Dosage::new(500).unwrap(),
            vec![],
        );

        assert!(med.scheduled_times().is_empty());
    }
}
