use crate::domain::errors::DomainError;

/// Immutable value object representing a quantity of pills in stock.
/// Avoids primitive obsession by encapsulating raw integers.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StockQuantity(u16);

impl StockQuantity {
    /// Creates a new StockQuantity with the given number of pills.
    pub fn new(amount: u16) -> Self {
        Self(amount)
    }

    /// Returns the number of pills in stock.
    pub fn amount(&self) -> u16 {
        self.0
    }

    /// Returns true if there are no pills in stock.
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    /// Returns true if there is at least one pill in stock.
    pub fn has_stock(&self) -> bool {
        self.0 > 0
    }

    /// Returns a new StockQuantity with the added pill count.
    /// Caps at u16::MAX to prevent overflow.
    pub fn replenish(&self, amount: u16) -> Self {
        let new_amount = self.0.saturating_add(amount);
        Self(new_amount)
    }

    /// Returns a new StockQuantity with the subtracted pill count.
    /// Returns Err if attempting to consume more than available.
    pub fn consume(&self, amount: u16) -> Result<Self, DomainError> {
        if amount > self.0 {
            return Err(DomainError::QuantityCannotBeNegative);
        }
        Ok(Self(self.0 - amount))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_quantity_with_given_amount() {
        let qty = StockQuantity::new(50);
        assert_eq!(qty.amount(), 50);
    }

    #[test]
    fn is_zero_returns_true_when_zero() {
        assert!(StockQuantity::new(0).is_zero());
        assert!(!StockQuantity::new(1).is_zero());
    }

    #[test]
    fn has_stock_returns_true_when_positive() {
        assert!(StockQuantity::new(1).has_stock());
        assert!(!StockQuantity::new(0).has_stock());
    }

    #[test]
    fn replenish_adds_to_existing_quantity() {
        let qty = StockQuantity::new(10).replenish(5);
        assert_eq!(qty.amount(), 15);
    }

    #[test]
    fn replenish_caps_at_max() {
        let qty = StockQuantity::new(u16::MAX).replenish(1);
        assert_eq!(qty.amount(), u16::MAX);
    }

    #[test]
    fn consume_reduces_quantity() {
        let qty = StockQuantity::new(10).consume(3).unwrap();
        assert_eq!(qty.amount(), 7);
    }

    #[test]
    fn consume_fails_when_insufficient_stock() {
        let result = StockQuantity::new(5).consume(10);
        assert!(matches!(result, Err(DomainError::QuantityCannotBeNegative)));
    }

    #[test]
    fn consume_returns_zero_when_exact_amount() {
        let qty = StockQuantity::new(5).consume(5).unwrap();
        assert_eq!(qty.amount(), 0);
        assert!(qty.is_zero());
    }
}
