use chrono::NaiveDateTime;

use crate::domain::{
    errors::DomainError,
    value_objects::{
        medication_id::MedicationId, medication_stock_id::MedicationStockId,
        stock_quantity::StockQuantity,
    },
};

/// Maintains the current stock level for a medication in pills.
///
/// A `MedicationStock` tracks how many pills/capsules remain for a given
/// medication. It is immutable — methods return new instances rather than
/// mutating in place.
///
/// # Invariants
///
/// - `id` is a time-sortable UUID v7 — unique per instance.
/// - `medication_id` references the owning [`Medication`] aggregate.
/// - `quantity` represents the number of pills in stock.
/// - `last_replenished_at` records when stock was last added.
///
/// [`Medication`]: crate::domain::entities::medication::Medication
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct MedicationStock {
    id: MedicationStockId,
    medication_id: MedicationId,
    quantity: StockQuantity,
    last_replenished_at: Option<NaiveDateTime>,
}

impl MedicationStock {
    /// Creates a new stock entry with auto-generated ID.
    ///
    /// - `medication_id` — the medication this stock belongs to.
    /// - `initial_pill_count` — the starting number of pills in stock.
    pub fn new(medication_id: MedicationId, initial_pill_count: u16) -> Self {
        Self {
            id: MedicationStockId::generate(),
            medication_id,
            quantity: StockQuantity::new(initial_pill_count),
            last_replenished_at: None,
        }
    }

    /// Reconstitutes a stock entry from storage.
    pub fn with_id(
        id: MedicationStockId,
        medication_id: MedicationId,
        quantity: StockQuantity,
        last_replenished_at: Option<NaiveDateTime>,
    ) -> Self {
        Self {
            id,
            medication_id,
            quantity,
            last_replenished_at,
        }
    }

    /// Consumes the specified number of pills from stock.
    ///
    /// Returns a new `MedicationStock` with the reduced pill count.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::QuantityCannotBeNegative`] if the requested
    /// amount exceeds the available stock.
    pub fn consume(&self, pill_count: u16) -> Result<Self, DomainError> {
        let new_quantity = self.quantity.consume(pill_count)?;
        Ok(Self {
            id: self.id.clone(),
            medication_id: self.medication_id.clone(),
            quantity: new_quantity,
            last_replenished_at: self.last_replenished_at,
        })
    }

    /// Adds the specified number of pills to stock.
    ///
    /// Returns a new `MedicationStock` with the increased pill count
    /// and updates `last_replenished_at` to the current time.
    pub fn replenish(&self, pill_count: u16) -> Self {
        Self {
            id: self.id.clone(),
            medication_id: self.medication_id.clone(),
            quantity: self.quantity.replenish(pill_count),
            last_replenished_at: Some(chrono::Utc::now().naive_utc()),
        }
    }

    /// Returns true if there is at least one pill in stock.
    pub fn has_stock(&self) -> bool {
        self.quantity.has_stock()
    }

    /// Returns true if the stock is empty (zero pills).
    pub fn is_empty(&self) -> bool {
        self.quantity.is_zero()
    }

    /// Returns the unique identifier of this stock entry.
    pub fn id(&self) -> &MedicationStockId {
        &self.id
    }

    /// Returns the medication ID this stock belongs to.
    pub fn medication_id(&self) -> &MedicationId {
        &self.medication_id
    }

    /// Returns the current quantity of pills in stock.
    pub fn quantity(&self) -> &StockQuantity {
        &self.quantity
    }

    /// Returns the timestamp when stock was last replenished, if applicable.
    pub fn last_replenished_at(&self) -> Option<NaiveDateTime> {
        self.last_replenished_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_stock() -> MedicationStock {
        MedicationStock::new(MedicationId::generate(), 100)
    }

    #[test]
    fn new_creates_stock_with_auto_generated_id() {
        let stock = make_stock();
        assert!(!stock.id().to_string().is_empty());
    }

    #[test]
    fn new_stores_medication_id() {
        let med_id = MedicationId::generate();
        let stock = MedicationStock::new(med_id.clone(), 50);
        assert_eq!(stock.medication_id(), &med_id);
    }

    #[test]
    fn new_stores_initial_pill_count() {
        let stock = MedicationStock::new(MedicationId::generate(), 75);
        assert_eq!(stock.quantity().amount(), 75);
    }

    #[test]
    fn has_stock_returns_true_when_positive() {
        let stock = MedicationStock::new(MedicationId::generate(), 1);
        assert!(stock.has_stock());
    }

    #[test]
    fn has_stock_returns_false_when_zero() {
        let stock = MedicationStock::new(MedicationId::generate(), 0);
        assert!(!stock.has_stock());
    }

    #[test]
    fn is_empty_returns_true_when_zero() {
        let stock = MedicationStock::new(MedicationId::generate(), 0);
        assert!(stock.is_empty());
    }

    #[test]
    fn is_empty_returns_false_when_positive() {
        let stock = MedicationStock::new(MedicationId::generate(), 10);
        assert!(!stock.is_empty());
    }

    #[test]
    fn consume_reduces_pill_count() {
        let stock = make_stock();
        let remaining = stock.consume(10).unwrap();
        assert_eq!(remaining.quantity().amount(), 90);
    }

    #[test]
    fn consume_fails_when_insufficient_stock() {
        let stock = MedicationStock::new(MedicationId::generate(), 5);
        let result = stock.consume(10);
        assert!(matches!(result, Err(DomainError::QuantityCannotBeNegative)));
    }

    #[test]
    fn consume_preserves_medication_id() {
        let med_id = MedicationId::generate();
        let stock = MedicationStock::new(med_id.clone(), 100);
        let consumed = stock.consume(50).unwrap();
        assert_eq!(consumed.medication_id(), &med_id);
    }

    #[test]
    fn consume_preserves_id() {
        let stock = make_stock();
        let consumed = stock.consume(10).unwrap();
        assert_eq!(consumed.id(), stock.id());
    }

    #[test]
    fn consume_preserves_last_replenished_at() {
        let stock = make_stock();
        let consumed = stock.consume(10).unwrap();
        assert_eq!(consumed.last_replenished_at(), stock.last_replenished_at());
    }

    #[test]
    fn replenish_increases_pill_count() {
        let stock = make_stock();
        let replenished = stock.replenish(50);
        assert_eq!(replenished.quantity().amount(), 150);
    }

    #[test]
    fn replenish_updates_last_replenished_at() {
        let stock = make_stock();
        let replenished = stock.replenish(10);
        assert!(replenished.last_replenished_at().is_some());
    }

    #[test]
    fn with_id_uses_provided_id() {
        let id = MedicationStockId::generate();
        let med_id = MedicationId::generate();
        let stock =
            MedicationStock::with_id(id.clone(), med_id.clone(), StockQuantity::new(30), None);
        assert_eq!(stock.id(), &id);
        assert_eq!(stock.medication_id(), &med_id);
    }

    #[test]
    fn with_id_preserves_last_replenished_at() {
        let med_id = MedicationId::generate();
        let existing_time = chrono::NaiveDate::from_ymd_opt(2025, 1, 1)
            .unwrap()
            .and_hms_opt(10, 0, 0);
        let stock = MedicationStock::with_id(
            MedicationStockId::generate(),
            med_id,
            StockQuantity::new(50),
            existing_time,
        );
        assert_eq!(stock.last_replenished_at(), existing_time);
    }

    #[test]
    fn consume_returns_new_instance() {
        let original = make_stock();
        let consumed = original.consume(5).unwrap();
        assert_eq!(original.quantity().amount(), 100);
        assert_eq!(consumed.quantity().amount(), 95);
    }

    #[test]
    fn replenish_returns_new_instance() {
        let original = make_stock();
        let replenished = original.replenish(5);
        assert_eq!(original.quantity().amount(), 100);
        assert_eq!(replenished.quantity().amount(), 105);
    }
}
