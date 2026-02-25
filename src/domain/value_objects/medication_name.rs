use crate::domain::errors::DomainError;

/// A validated, trimmed display name for a medication (e.g. `"Aspirin"`).
///
/// `MedicationName` is a value object — two instances with the same string are
/// considered equal. It enforces that the name is never empty or whitespace-only,
/// and stores the value with surrounding whitespace stripped.
///
/// # Invariants
///
/// - The underlying string is never empty after trimming.
/// - Surrounding whitespace is removed on construction.
///
/// # Examples
///
/// ```rust
/// use bitpill::domain::{value_objects::medication_name::MedicationName, errors::DomainError};
///
/// let name = MedicationName::new("  Ibuprofen  ").unwrap();
/// assert_eq!(name.value(), "Ibuprofen");
///
/// assert!(matches!(MedicationName::new(""), Err(DomainError::EmptyMedicationName)));
/// assert!(matches!(MedicationName::new("   "), Err(DomainError::EmptyMedicationName)));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MedicationName(String);

impl MedicationName {
    /// Creates a new `MedicationName` from any string-like value.
    ///
    /// Whitespace is trimmed before validation and storage.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError::EmptyMedicationName`] when the trimmed string is empty.
    pub fn new(name: impl Into<String>) -> Result<Self, DomainError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(DomainError::EmptyMedicationName);
        }
        Ok(Self(name.trim().to_string()))
    }

    /// Returns the medication name as a string slice.
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for MedicationName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_with_valid_name_returns_medication_name() {
        let result = MedicationName::new("Aspirin");

        assert!(result.is_ok());
        assert_eq!(result.unwrap().value(), "Aspirin");
    }

    #[test]
    fn new_with_empty_string_returns_error() {
        let result = MedicationName::new("");

        assert!(matches!(result, Err(DomainError::EmptyMedicationName)));
    }

    #[test]
    fn new_with_whitespace_only_returns_error() {
        let result = MedicationName::new("   ");

        assert!(matches!(result, Err(DomainError::EmptyMedicationName)));
    }

    #[test]
    fn new_trims_surrounding_whitespace() {
        let result = MedicationName::new("  Ibuprofen  ");

        assert_eq!(result.unwrap().value(), "Ibuprofen");
    }

    #[test]
    fn display_formats_as_medication_name_string() {
        let name = MedicationName::new("Paracetamol").unwrap();

        assert_eq!(name.to_string(), "Paracetamol");
    }
}
