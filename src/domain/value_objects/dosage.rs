use crate::domain::errors::DomainError;

/// The prescribed amount of a medication in milligrams.
///
/// `Dosage` is a value object — two instances with the same `amount_mg` are
/// considered equal regardless of identity.
///
/// # Invariants
///
/// - `amount_mg` must be greater than zero. A zero dosage is meaningless and
///   rejected by the constructor.
///
/// # Examples
///
/// ```rust
/// use bitpill::domain::{value_objects::dosage::Dosage, errors::DomainError};
///
/// let dosage = Dosage::new(500).unwrap();
/// assert_eq!(dosage.amount_mg(), 500);
/// assert_eq!(dosage.to_string(), "500mg");
///
/// assert!(matches!(Dosage::new(0), Err(DomainError::InvalidDosage)));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Dosage {
    amount_mg: u32,
}

impl Dosage {
    /// Creates a new `Dosage`.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::InvalidDosage`] when `amount_mg` is `0`.
    pub fn new(amount_mg: u32) -> Result<Self, DomainError> {
        if amount_mg == 0 {
            return Err(DomainError::InvalidDosage);
        }
        Ok(Self { amount_mg })
    }

    /// Returns the dosage amount in milligrams.
    pub fn amount_mg(&self) -> u32 {
        self.amount_mg
    }
}

impl std::fmt::Display for Dosage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}mg", self.amount_mg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_with_valid_amount_returns_dosage() {
        let result = Dosage::new(500);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().amount_mg(), 500);
    }

    #[test]
    fn new_with_zero_amount_returns_error() {
        let result = Dosage::new(0);

        assert!(matches!(result, Err(DomainError::InvalidDosage)));
    }

    #[test]
    fn amount_mg_returns_value_passed_to_constructor() {
        let dosage = Dosage::new(250).unwrap();

        assert_eq!(dosage.amount_mg(), 250);
    }

    #[test]
    fn display_formats_with_mg_suffix() {
        let dosage = Dosage::new(100).unwrap();

        assert_eq!(dosage.to_string(), "100mg");
    }
}
