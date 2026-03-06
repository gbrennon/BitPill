/// Domain layer — pure business logic with no I/O or infrastructure dependencies.
///
/// # Structure
///
/// - [`entities`] — Aggregate roots that carry identity and behaviour.
/// - [`value_objects`] — Immutable types defined by their attributes, not identity.
/// - [`errors`] — Domain-level error variants shared across this layer.
///
/// # Usage example
///
/// ```rust
/// use bitpill::domain::{
///     entities::{medication::Medication, dose_record::DoseRecord},
///     value_objects::{
///         dosage::Dosage,
///         medication_id::MedicationId,
///         medication_name::MedicationName,
///         scheduled_time::ScheduledTime,
///         medication_frequency::DoseFrequency,
///     },
/// };
/// use chrono::NaiveDate;
///
/// // Build value objects (validation happens here).
/// let name     = MedicationName::new("Aspirin").unwrap();
/// let dosage   = Dosage::new(500).unwrap();
/// let morning  = ScheduledTime::new(8, 0).unwrap();
///
/// // Create a Medication aggregate root.
/// let medication = Medication::new(MedicationId::generate(), name, dosage, vec![morning], DoseFrequency::OnceDaily);
///
/// // Create a DoseRecord and mark it as taken.
/// let scheduled_at = NaiveDate::from_ymd_opt(2025, 6, 1)
///     .unwrap()
///     .and_hms_opt(8, 0, 0)
///     .unwrap();
/// let mut record = DoseRecord::new(medication.id().clone(), scheduled_at);
/// record.mark_taken(scheduled_at).unwrap();
/// assert!(record.is_taken());
/// ```
pub mod entities;
pub mod errors;
pub mod ports;
pub mod value_objects;
