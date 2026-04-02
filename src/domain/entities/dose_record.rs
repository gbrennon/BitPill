use chrono::NaiveDateTime;

use crate::domain::{
    errors::DomainError,
    value_objects::{dose_record_id::DoseRecordId, medication_id::MedicationId},
};

/// Records whether a single scheduled dose of a medication was taken.
///
/// A `DoseRecord` is created when a dose is *scheduled* (`taken_at = None`)
/// and transitions to *taken* exactly once via [`mark_taken`].
/// The transition is irreversible — calling [`mark_taken`] on an already-taken
/// record returns [`DomainError::DoseAlreadyTaken`].
///
/// # Invariants
///
/// - `id` is a time-sortable UUID v7 — unique per instance.
/// - `taken_at` starts as `None` and can only be set once.
/// - `medication_id` must reference an existing [`Medication`] aggregate root
///   (the domain does not enforce referential integrity; that is the
///   repository's responsibility).
///
/// [`mark_taken`]: DoseRecord::mark_taken
/// [`Medication`]: crate::domain::entities::medication::Medication
///
/// # Examples
///
/// ```rust
/// use bitpill::domain::{
///     entities::dose_record::DoseRecord,
///     value_objects::medication_id::MedicationId,
///     errors::DomainError,
/// };
/// use chrono::NaiveDate;
///
/// let medication_id  = MedicationId::generate();
/// let scheduled_at   = NaiveDate::from_ymd_opt(2025, 6, 1)
///     .unwrap()
///     .and_hms_opt(8, 0, 0)
///     .unwrap();
///
/// let mut record = DoseRecord::new(medication_id, scheduled_at);
/// assert!(!record.is_taken());
///
/// // Mark the dose as taken.
/// record.mark_taken(scheduled_at).unwrap();
/// assert!(record.is_taken());
///
/// // Attempting to mark it taken again is an error.
/// assert!(matches!(
///     record.mark_taken(scheduled_at),
///     Err(DomainError::DoseAlreadyTaken)
/// ));
/// ```
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct DoseRecord {
    id: DoseRecordId,
    medication_id: MedicationId,
    scheduled_at: NaiveDateTime,
    taken_at: Option<NaiveDateTime>,
}

impl DoseRecord {
    /// Creates a new, untaken dose record.
    ///
    /// - `medication_id` — the medication this record belongs to.
    /// - `scheduled_at` — the datetime at which the dose was due.
    ///
    /// `taken_at` is initialised to `None`; the record is not yet taken.
    pub fn new(medication_id: MedicationId, scheduled_at: NaiveDateTime) -> Self {
        Self {
            id: DoseRecordId::generate(),
            medication_id,
            scheduled_at,
            taken_at: None,
        }
    }

    pub fn with_id(
        id: DoseRecordId,
        medication_id: MedicationId,
        scheduled_at: NaiveDateTime,
        taken_at: NaiveDateTime,
    ) -> Self {
        Self {
            id,
            medication_id,
            scheduled_at,
            taken_at: Some(taken_at),
        }
    }

    /// Marks this dose as taken at the given datetime.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::DoseAlreadyTaken`] if the dose was already marked
    /// taken. This enforces the invariant that a dose can only be taken once.
    pub fn mark_taken(&mut self, at: NaiveDateTime) -> Result<(), DomainError> {
        if self.taken_at.is_some() {
            return Err(DomainError::DoseAlreadyTaken);
        }
        self.taken_at = Some(at);
        Ok(())
    }

    /// Returns `true` if the dose has been marked as taken.
    pub fn is_taken(&self) -> bool {
        self.taken_at.is_some()
    }

    /// Returns the unique identifier of this dose record.
    pub fn id(&self) -> &DoseRecordId {
        &self.id
    }

    /// Returns the identifier of the medication this record belongs to.
    pub fn medication_id(&self) -> &MedicationId {
        &self.medication_id
    }

    /// Returns the datetime at which the dose was scheduled.
    pub fn scheduled_at(&self) -> NaiveDateTime {
        self.scheduled_at
    }

    /// Returns the datetime at which the dose was taken, or `None` if not yet taken.
    pub fn taken_at(&self) -> Option<NaiveDateTime> {
        self.taken_at
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::*;

    fn make_datetime(h: u32, m: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2025, 1, 1)
            .unwrap()
            .and_hms_opt(h, m, 0)
            .unwrap()
    }

    fn make_dose_record() -> DoseRecord {
        DoseRecord::new(MedicationId::generate(), make_datetime(8, 0))
    }

    #[test]
    fn new_creates_record_as_not_taken() {
        let record = make_dose_record();

        assert!(!record.is_taken());
        assert!(record.taken_at().is_none());
    }

    #[test]
    fn new_assigns_a_unique_id() {
        let record_a = make_dose_record();
        let record_b = make_dose_record();

        assert_ne!(record_a.id(), record_b.id());
    }

    #[test]
    fn mark_taken_sets_taken_at_and_marks_as_taken() {
        let mut record = make_dose_record();
        let taken_at = make_datetime(8, 5);

        record.mark_taken(taken_at).unwrap();

        assert!(record.is_taken());
        assert_eq!(record.taken_at(), Some(taken_at));
    }

    #[test]
    fn mark_taken_twice_returns_error() {
        let mut record = make_dose_record();
        record.mark_taken(make_datetime(8, 5)).unwrap();

        let result = record.mark_taken(make_datetime(8, 10));

        assert!(matches!(result, Err(DomainError::DoseAlreadyTaken)));
    }

    #[test]
    fn scheduled_at_returns_the_datetime_passed_to_constructor() {
        let scheduled_at = make_datetime(20, 0);
        let record = DoseRecord::new(MedicationId::generate(), scheduled_at);

        assert_eq!(record.scheduled_at(), scheduled_at);
    }

    #[test]
    fn with_id_allows_constructing_taken_record() {
        let id = DoseRecordId::generate();
        let med_id = MedicationId::generate();
        let scheduled_at = make_datetime(9, 0);
        let taken_at = make_datetime(9, 5);
        let record = DoseRecord::with_id(id.clone(), med_id.clone(), scheduled_at, taken_at);

        assert_eq!(record.id(), &id);
        assert_eq!(record.medication_id(), &med_id);
        assert_eq!(record.scheduled_at(), scheduled_at);
        assert_eq!(record.taken_at(), Some(taken_at));
        assert!(record.is_taken());
    }
}
