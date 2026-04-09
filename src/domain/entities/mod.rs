/// Domain entities — aggregate roots that carry identity and behavior.
///
/// Entities are types with distinct identity: two entities with the same
/// attributes but different IDs are not equal. Each entity has a unique
/// identifier and encapsulates the business rules governing its lifecycle.
///
/// # Entities
///
/// - [`medication::Medication`] — Represents a medication prescription.
/// - [`dose_record::DoseRecord`] — Records whether a scheduled dose was taken.
/// - [`medication_refill::MedicationRefill`] — Records a medication purchase.
/// - [`medication_stock::MedicationStock`] — Maintains current stock level of a medication.
/// - [`app_settings::AppSettings`] — Application configuration settings.
pub mod app_settings;
pub mod dose_record;
pub mod medication;
pub mod medication_refill;
pub mod medication_stock;
