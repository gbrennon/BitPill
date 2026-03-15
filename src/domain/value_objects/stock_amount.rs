use crate::domain::errors::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StockAmount{
    value: u8,
}

impl StockAmount {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn increase(&self, amount: u8) -> Result<StockAmount, DomainError> {
        let new_stock_amount = self
            .value
            .checked_add(amount)
            .ok_or(DomainError::StockAmountOverflow)?;

        Ok(Self::new(new_stock_amount))
    }

    pub fn decrease(&self, amount: u8) -> Result<StockAmount, DomainError> {
        let value = self
            .value
            .checked_sub(amount)
            .ok_or(DomainError::StockAmountNotEnough)?;

        Ok(Self { value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_when_value_then_assign_parameter() {
        let value = 128;

        let stock_amount = StockAmount::new(value);

        let expected_stock_amount = StockAmount { value };
        assert_eq!(stock_amount, expected_stock_amount);
    }

    #[test]
    fn value_when_called_then_return_assigned() {
        let value =128;

        let stock_amount = StockAmount { value: value };

        let expected_value = 128;
        assert_eq!(stock_amount.value(), expected_value)
    }

    #[test]
    fn increase_when_result_bigger_than_255_then_error() {
        let value = 255;
        let incrementer = 1;
        let stock_amount = StockAmount { value: value };

        let result = stock_amount.increase(incrementer);

        let expected_result = DomainError::StockAmountOverflow;
        assert_eq!(expected_result, result.unwrap_err());
    }

    #[test]
    fn decrease_when_amount_biggger_than_available_then_not_enough() {
        let value = 0;
        let decrementer = 1;
        let stock_amount = StockAmount { value: value };

        let result = stock_amount.decrease(decrementer);

        let expected_result = DomainError::StockAmountNotEnough;
        assert_eq!(expected_result, result.unwrap_err());
    }
}
