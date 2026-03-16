use chrono::NaiveDateTime;

use crate::domain::value_objects::{
    dosage::Dosage,
    medication_id::MedicationId,
    medication_refill_id::MedicationRefillId,
};

/// Records a physical purchase of a medication package.
///
/// A `MedicationRefill` is created when the user buys a new supply of a
/// medication. It tracks which medication was purchased, its dosage strength,
/// the package size, how many packages were bought, and when the purchase
/// was made.
///
/// `medication_id` is a foreign-key reference to the owning [`Medication`]
/// aggregate root (the same pattern used by [`DoseRecord`]).
///
/// [`Medication`]: crate::domain::entities::medication::Medication
/// [`DoseRecord`]: crate::domain::entities::dose_record::DoseRecord
///
/// # Invariants
///
/// - `id` is a time-sortable UUID v7 — unique per instance.
/// - `medication_id` must reference an existing [`Medication`] (referential
///   integrity is enforced by the repository, not the domain).
/// - `pill_strength` must be non-zero (validated by [`Dosage::new`]).
/// - `pills_per_package` and `packages_purchased` are raw counts; callers
///   are responsible for supplying positive values.
///
/// # Examples
///
/// ```rust
/// use bitpill::domain::{
///     entities::medication_refill::MedicationRefill,
///     value_objects::{
///         dosage::Dosage,
///         medication_id::MedicationId,
///     },
/// };
/// use chrono::NaiveDate;
///
/// let medication_id = MedicationId::generate();
/// let purchased_at  = NaiveDate::from_ymd_opt(2025, 6, 1)
///     .unwrap()
///     .and_hms_opt(10, 30, 0)
///     .unwrap();
///
/// let refill = MedicationRefill::new(
///     medication_id,
///     Dosage::new(500).unwrap(),
///     30,
///     2,
///     purchased_at,
/// );
///
/// assert_eq!(refill.pills_per_package(), 30);
/// assert_eq!(refill.packages_purchased(), 2);
/// ```
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct MedicationRefill {
    id: MedicationRefillId,
    medication_id: MedicationId,
    pill_strength: Dosage,
    pills_per_package: u16,
    packages_purchased: u16,
    purchased_at: NaiveDateTime,
}

impl MedicationRefill {
    /// Creates a new `MedicationRefill`, auto-generating a fresh UUID v7 identifier.
    ///
    /// - `medication_id` — the medication this refill belongs to.
    /// - `pill_strength` — dosage per tablet/capsule.
    /// - `pills_per_package` — number of pills in one package.
    /// - `packages_purchased` — how many packages were bought.
    /// - `purchased_at` — timestamp of the purchase.
    pub fn new(
        medication_id: MedicationId,
        pill_strength: Dosage,
        pills_per_package: u16,
        packages_purchased: u16,
        purchased_at: NaiveDateTime,
    ) -> Self {
        Self {
            id: MedicationRefillId::generate(),
            medication_id,
            pill_strength,
            pills_per_package,
            packages_purchased,
            purchased_at,
        }
    }

    /// Reconstitutes a `MedicationRefill` from a known `id` (e.g. loaded from storage).
    pub fn with_id(
        id: MedicationRefillId,
        medication_id: MedicationId,
        pill_strength: Dosage,
        pills_per_package: u16,
        packages_purchased: u16,
        purchased_at: NaiveDateTime,
    ) -> Self {
        Self {
            id,
            medication_id,
            pill_strength,
            pills_per_package,
            packages_purchased,
            purchased_at,
        }
    }

    /// Returns the unique identifier of this refill.
    pub fn id(&self) -> &MedicationRefillId {
        &self.id
    }

    /// Returns the identifier of the medication this refill belongs to.
    pub fn medication_id(&self) -> &MedicationId {
        &self.medication_id
    }

    /// Returns the dosage strength per pill.
    pub fn pill_strength(&self) -> &Dosage {
        &self.pill_strength
    }

    /// Returns the number of pills in one package.
    pub fn pills_per_package(&self) -> u16 {
        self.pills_per_package
    }

    /// Returns how many packages were purchased.
    pub fn packages_purchased(&self) -> u16 {
        self.packages_purchased
    }

    /// Returns the datetime the purchase was made.
    pub fn purchased_at(&self) -> NaiveDateTime {
        self.purchased_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn make_datetime() -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2025, 1, 1)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap()
    }

    fn make_refill() -> MedicationRefill {
        MedicationRefill::new(
            MedicationId::generate(),
            Dosage::new(500).unwrap(),
            30,
            1,
            make_datetime(),
        )
    }

    #[test]
    fn new_assigns_a_unique_id() {
        let a = make_refill();
        let b = make_refill();

        assert_ne!(a.id(), b.id());
    }

    #[test]
    fn new_stores_medication_id() {
        let med_id = MedicationId::generate();

        let refill = MedicationRefill::new(
            med_id.clone(),
            Dosage::new(500).unwrap(),
            30,
            1,
            make_datetime(),
        );

        assert_eq!(refill.medication_id(), &med_id);
    }

    #[test]
    fn new_stores_pill_strength() {
        let refill = MedicationRefill::new(
            MedicationId::generate(),
            Dosage::new(250).unwrap(),
            30,
            1,
            make_datetime(),
        );

        assert_eq!(refill.pill_strength().amount_mg(), 250);
    }

    #[test]
    fn new_stores_pills_per_package() {
        let refill = MedicationRefill::new(
            MedicationId::generate(),
            Dosage::new(500).unwrap(),
            60,
            1,
            make_datetime(),
        );

        assert_eq!(refill.pills_per_package(), 60);
    }

    #[test]
    fn new_stores_packages_purchased() {
        let refill = MedicationRefill::new(
            MedicationId::generate(),
            Dosage::new(500).unwrap(),
            30,
            3,
            make_datetime(),
        );

        assert_eq!(refill.packages_purchased(), 3);
    }

    #[test]
    fn new_stores_purchased_at() {
        let purchased_at = make_datetime();

        let refill = MedicationRefill::new(
            MedicationId::generate(),
            Dosage::new(500).unwrap(),
            30,
            1,
            purchased_at,
        );

        assert_eq!(refill.purchased_at(), purchased_at);
    }

    #[test]
    fn with_id_uses_the_supplied_id() {
        let id = MedicationRefillId::generate();
        let med_id = MedicationId::generate();

        let refill = MedicationRefill::with_id(
            id.clone(),
            med_id,
            Dosage::new(100).unwrap(),
            28,
            2,
            make_datetime(),
        );

        assert_eq!(refill.id(), &id);
    }
}
