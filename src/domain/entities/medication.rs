use crate::domain::value_objects::medication_frequency::DoseFrequency;
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
/// - `scheduled_time` may be empty (unscheduled medication is allowed).
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
///     MedicationId::generate(),
///     MedicationName::new("Ibuprofen").unwrap(),
///     Dosage::new(400).unwrap(),
///     vec![ScheduledTime::new(8, 0).unwrap(), ScheduledTime::new(20, 0).unwrap()],
/// );
///
/// assert_eq!(medication.name().value(), "Ibuprofen");
/// assert_eq!(medication.dosage().amount_mg(), 400);
/// assert_eq!(medication.scheduled_time().len(), 2);
/// ```
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Medication {
    id: MedicationId,
    name: MedicationName,
    dosage: Dosage,
    scheduled_time: Vec<ScheduledTime>,
    dose_frequency: DoseFrequency,
}

impl Medication {
    /// Creates a new `Medication` with the supplied `id`.
    ///
    /// Use [`MedicationId::generate`] to generate a fresh identifier, or
    /// `MedicationId::from(uuid)` to reconstitute one from storage.
    /// `name` and `dosage` are pre-validated value objects.
    /// `scheduled_time` may be an empty `Vec` for unscheduled medications.
    pub fn new(
        id: MedicationId,
        name: MedicationName,
        dosage: Dosage,
        scheduled_time: Vec<ScheduledTime>,
        dose_frequency: DoseFrequency,
    ) -> Self {
        Self {
            id,
            name,
            dosage,
            scheduled_time,
            dose_frequency,
        }
    }

    /// Reconstitutes a `Medication` from a known `id` (e.g. loaded from storage).
    ///
    /// Identical to [`new`](Self::new) — prefer `new` when building fresh
    /// instances and `with_id` when the intent is explicit reconstitution.
    pub fn with_id(
        id: MedicationId,
        name: MedicationName,
        dosage: Dosage,
        scheduled_time: Vec<ScheduledTime>,
        dose_frequency: DoseFrequency,
    ) -> Self {
        Self {
            id,
            name,
            dosage,
            scheduled_time,
            dose_frequency,
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
    pub fn scheduled_time(&self) -> &[ScheduledTime] {
        &self.scheduled_time
    }

    pub fn dose_frequency(&self) -> &DoseFrequency {
        &self.dose_frequency
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
            MedicationId::generate(),
            MedicationName::new("Aspirin").unwrap(),
            Dosage::new(500).unwrap(),
            vec![
                ScheduledTime::new(8, 0).unwrap(),
                ScheduledTime::new(20, 0).unwrap(),
            ],
            DoseFrequency::TwiceDaily,
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
    fn scheduled_time_returns_all_times_passed_to_constructor() {
        let med = make_medication();

        assert_eq!(med.scheduled_time().len(), 2);
        assert_eq!(med.scheduled_time()[0].to_string(), "08:00");
        assert_eq!(med.scheduled_time()[1].to_string(), "20:00");
    }

    #[test]
    fn new_with_no_scheduled_time_is_allowed() {
        let med = Medication::new(
            MedicationId::generate(),
            MedicationName::new("Aspirin").unwrap(),
            Dosage::new(500).unwrap(),
            vec![],
            DoseFrequency::OnceDaily,
        );

        assert!(med.scheduled_time().is_empty());
    }
}
