use crate::domain::{
    errors::DomainError,
    value_objects::{
        dosage::Dosage, medication_frequency::DoseFrequency, medication_id::MedicationId,
        medication_name::MedicationName, scheduled_time::ScheduledTime,
    },
};

/// An aggregate root representing a medication prescribed to a patient.
///
/// `Medication` groups together everything that defines *what* is to be taken:
/// an identity, a human-readable name, a dosage, and an optional list of
/// scheduled administration times.
///
/// Refill history is tracked separately via [`MedicationRefill`] entities,
/// which carry a `medication_id` foreign key (the same pattern used by
/// [`DoseRecord`]).
///
/// # Invariants
///
/// - `id` is supplied by the caller — use [`MedicationId::create`] to generate
///   a fresh UUID v7, or [`MedicationId::from_uuid`] to reconstitute one from
///   storage.
/// - `name` and `dosage` are validated value objects; they can only hold
///   legal values (non-empty name, non-zero dosage).
/// - `scheduled_time` must satisfy the constraints defined by `dose_frequency`:
///   - For `OnceDaily`, exactly 1 scheduled time is required.
///   - For `TwiceDaily`, exactly 2 scheduled times are required.
///   - For `ThriceDaily`, exactly 3 scheduled times are required.
///   - For `Custom`, at least 4 scheduled times are required.
/// - `scheduled_time` must not contain duplicate times.
///
/// [`MedicationRefill`]: crate::domain::entities::medication_refill::MedicationRefill
/// [`DoseRecord`]: crate::domain::entities::dose_record::DoseRecord
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
///         medication_frequency::DoseFrequency,
///     },
/// };
///
/// let medication = Medication::new(
///     MedicationId::generate(),
///     MedicationName::new("Ibuprofen").unwrap(),
///     Dosage::new(400).unwrap(),
///     vec![ScheduledTime::new(8, 0).unwrap()],
///     DoseFrequency::OnceDaily,
/// )
/// .unwrap();
///
/// assert_eq!(medication.name().value(), "Ibuprofen");
/// assert_eq!(medication.dosage().amount_mg(), 400);
/// assert_eq!(medication.scheduled_time().len(), 1);
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
    pub fn new(
        id: MedicationId,
        name: MedicationName,
        dosage: Dosage,
        scheduled_time: Vec<ScheduledTime>,
        dose_frequency: DoseFrequency,
    ) -> Result<Self, Vec<DomainError>> {
        let scheduled_errors = Self::validate_scheduled_times(&scheduled_time, &dose_frequency);
        if scheduled_errors.is_empty() {
            Ok(Self {
                id,
                name,
                dosage,
                scheduled_time,
                dose_frequency,
            })
        } else {
            Err(scheduled_errors)
        }
    }

    pub fn with_id(
        id: MedicationId,
        name: MedicationName,
        dosage: Dosage,
        scheduled_time: Vec<ScheduledTime>,
        dose_frequency: DoseFrequency,
    ) -> Result<Self, Vec<DomainError>> {
        let scheduled_errors = Self::validate_scheduled_times(&scheduled_time, &dose_frequency);
        if scheduled_errors.is_empty() {
            Ok(Self {
                id,
                name,
                dosage,
                scheduled_time,
                dose_frequency,
            })
        } else {
            Err(scheduled_errors)
        }
    }

    fn validate_scheduled_times(
        scheduled_time: &[ScheduledTime],
        dose_frequency: &DoseFrequency,
    ) -> Vec<DomainError> {
        let mut errors = Vec::new();

        let mut sorted = scheduled_time.to_vec();
        sorted.sort();
        sorted.dedup();
        let has_duplicates = sorted.len() != scheduled_time.len();

        match dose_frequency {
            DoseFrequency::OnceDaily => {
                if scheduled_time.len() != 1 {
                    errors.push(DomainError::InvalidScheduledTimesCount);
                }
            }
            DoseFrequency::TwiceDaily => {
                if scheduled_time.len() != 2 {
                    errors.push(DomainError::InvalidScheduledTimesCount);
                }
            }
            DoseFrequency::ThriceDaily => {
                if scheduled_time.len() != 3 {
                    errors.push(DomainError::InvalidScheduledTimesCount);
                }
            }
            DoseFrequency::Custom(_) => {
                if scheduled_time.len() < 4 {
                    errors.push(DomainError::CustomFrequencyRequiresMinimumFourTimes);
                }
            }
        }

        if has_duplicates {
            errors.push(DomainError::DuplicateScheduledTime);
        }

        errors
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
        .unwrap()
    }

    fn assert_error_contains(errors: Vec<DomainError>, expected: DomainError) {
        assert!(
            errors.contains(&expected),
            "expected errors to contain {:?}, got {:?}",
            expected,
            errors
        );
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
    fn with_id_constructs_medication_with_given_id() {
        let id = MedicationId::generate();
        let name = MedicationName::new("TestMed").unwrap();
        let dosage = Dosage::new(250).unwrap();
        let times = vec![
            ScheduledTime::new(8, 0).unwrap(),
            ScheduledTime::new(12, 0).unwrap(),
            ScheduledTime::new(16, 0).unwrap(),
            ScheduledTime::new(20, 0).unwrap(),
        ];
        let med = Medication::with_id(
            id.clone(),
            name.clone(),
            dosage.clone(),
            times.clone(),
            DoseFrequency::Custom(times),
        )
        .unwrap();

        assert_eq!(med.id(), &id);
        assert_eq!(med.name().value(), name.value());
        assert_eq!(med.dosage().amount_mg(), dosage.amount_mg());
    }

    #[test]
    fn once_daily_accepts_one_scheduled_time() {
        let result = Medication::new(
            MedicationId::generate(),
            MedicationName::new("Aspirin").unwrap(),
            Dosage::new(500).unwrap(),
            vec![ScheduledTime::new(8, 0).unwrap()],
            DoseFrequency::OnceDaily,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn once_daily_rejects_empty_times() {
        let result = Medication::new(
            MedicationId::generate(),
            MedicationName::new("Aspirin").unwrap(),
            Dosage::new(500).unwrap(),
            vec![],
            DoseFrequency::OnceDaily,
        );

        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.contains(&DomainError::InvalidScheduledTimesCount));
    }

    #[test]
    fn once_daily_rejects_multiple_times() {
        let result = Medication::new(
            MedicationId::generate(),
            MedicationName::new("Aspirin").unwrap(),
            Dosage::new(500).unwrap(),
            vec![
                ScheduledTime::new(8, 0).unwrap(),
                ScheduledTime::new(20, 0).unwrap(),
            ],
            DoseFrequency::OnceDaily,
        );

        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.contains(&DomainError::InvalidScheduledTimesCount));
    }

    #[test]
    fn twice_daily_accepts_two_scheduled_times() {
        let result = Medication::new(
            MedicationId::generate(),
            MedicationName::new("Aspirin").unwrap(),
            Dosage::new(500).unwrap(),
            vec![
                ScheduledTime::new(8, 0).unwrap(),
                ScheduledTime::new(20, 0).unwrap(),
            ],
            DoseFrequency::TwiceDaily,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn twice_daily_rejects_one_time() {
        let result = Medication::new(
            MedicationId::generate(),
            MedicationName::new("Aspirin").unwrap(),
            Dosage::new(500).unwrap(),
            vec![ScheduledTime::new(8, 0).unwrap()],
            DoseFrequency::TwiceDaily,
        );

        assert!(result.is_err());
        assert_error_contains(result.unwrap_err(), DomainError::InvalidScheduledTimesCount);
    }

    #[test]
    fn thrice_daily_accepts_three_scheduled_times() {
        let result = Medication::new(
            MedicationId::generate(),
            MedicationName::new("Aspirin").unwrap(),
            Dosage::new(500).unwrap(),
            vec![
                ScheduledTime::new(8, 0).unwrap(),
                ScheduledTime::new(14, 0).unwrap(),
                ScheduledTime::new(20, 0).unwrap(),
            ],
            DoseFrequency::ThriceDaily,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn thrice_daily_rejects_two_times() {
        let result = Medication::new(
            MedicationId::generate(),
            MedicationName::new("Aspirin").unwrap(),
            Dosage::new(500).unwrap(),
            vec![
                ScheduledTime::new(8, 0).unwrap(),
                ScheduledTime::new(20, 0).unwrap(),
            ],
            DoseFrequency::ThriceDaily,
        );

        assert!(result.is_err());
        assert_error_contains(result.unwrap_err(), DomainError::InvalidScheduledTimesCount);
    }

    #[test]
    fn custom_requires_at_least_four_times() {
        let result = Medication::new(
            MedicationId::generate(),
            MedicationName::new("Aspirin").unwrap(),
            Dosage::new(500).unwrap(),
            vec![
                ScheduledTime::new(8, 0).unwrap(),
                ScheduledTime::new(12, 0).unwrap(),
                ScheduledTime::new(16, 0).unwrap(),
                ScheduledTime::new(20, 0).unwrap(),
            ],
            DoseFrequency::Custom(vec![]),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn custom_rejects_fewer_than_four_times() {
        let result = Medication::new(
            MedicationId::generate(),
            MedicationName::new("Aspirin").unwrap(),
            Dosage::new(500).unwrap(),
            vec![],
            DoseFrequency::Custom(vec![]),
        );

        assert!(result.is_err());
        assert_error_contains(
            result.unwrap_err(),
            DomainError::CustomFrequencyRequiresMinimumFourTimes,
        );
    }

    #[test]
    fn custom_accepts_multiple_times() {
        let result = Medication::new(
            MedicationId::generate(),
            MedicationName::new("Aspirin").unwrap(),
            Dosage::new(500).unwrap(),
            vec![
                ScheduledTime::new(8, 0).unwrap(),
                ScheduledTime::new(12, 0).unwrap(),
                ScheduledTime::new(16, 0).unwrap(),
                ScheduledTime::new(20, 0).unwrap(),
            ],
            DoseFrequency::Custom(vec![
                ScheduledTime::new(8, 0).unwrap(),
                ScheduledTime::new(12, 0).unwrap(),
                ScheduledTime::new(16, 0).unwrap(),
                ScheduledTime::new(20, 0).unwrap(),
            ]),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn duplicate_times_are_rejected_for_twice_daily() {
        let result = Medication::new(
            MedicationId::generate(),
            MedicationName::new("Aspirin").unwrap(),
            Dosage::new(500).unwrap(),
            vec![
                ScheduledTime::new(8, 0).unwrap(),
                ScheduledTime::new(8, 0).unwrap(),
            ],
            DoseFrequency::TwiceDaily,
        );

        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.contains(&DomainError::DuplicateScheduledTime));
    }

    #[test]
    fn duplicate_times_are_rejected_for_thrice_daily() {
        let result = Medication::new(
            MedicationId::generate(),
            MedicationName::new("Aspirin").unwrap(),
            Dosage::new(500).unwrap(),
            vec![
                ScheduledTime::new(8, 0).unwrap(),
                ScheduledTime::new(14, 0).unwrap(),
                ScheduledTime::new(8, 0).unwrap(),
            ],
            DoseFrequency::ThriceDaily,
        );

        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.contains(&DomainError::DuplicateScheduledTime));
    }

    #[test]
    fn with_id_rejects_wrong_times_count() {
        let result = Medication::with_id(
            MedicationId::generate(),
            MedicationName::new("Aspirin").unwrap(),
            Dosage::new(500).unwrap(),
            vec![ScheduledTime::new(8, 0).unwrap()],
            DoseFrequency::TwiceDaily,
        );

        assert!(result.is_err());
        assert_error_contains(result.unwrap_err(), DomainError::InvalidScheduledTimesCount);
    }

    #[test]
    fn with_id_rejects_duplicate_times() {
        let result = Medication::with_id(
            MedicationId::generate(),
            MedicationName::new("Aspirin").unwrap(),
            Dosage::new(500).unwrap(),
            vec![
                ScheduledTime::new(8, 0).unwrap(),
                ScheduledTime::new(8, 0).unwrap(),
            ],
            DoseFrequency::TwiceDaily,
        );

        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.contains(&DomainError::DuplicateScheduledTime));
    }

    #[test]
    fn new_with_custom_frequency_valid_times_succeeds() {
        let result = Medication::new(
            MedicationId::generate(),
            MedicationName::new("Test").unwrap(),
            Dosage::new(100).unwrap(),
            vec![
                ScheduledTime::new(8, 0).unwrap(),
                ScheduledTime::new(12, 0).unwrap(),
                ScheduledTime::new(16, 0).unwrap(),
                ScheduledTime::new(20, 0).unwrap(),
            ],
            DoseFrequency::Custom(vec![]),
        );

        assert!(result.is_ok());
    }
}
