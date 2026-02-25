use crate::domain::errors::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PillName(String);

impl PillName {
    pub fn new(name: impl Into<String>) -> Result<Self, DomainError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(DomainError::EmptyPillName);
        }
        Ok(Self(name.trim().to_string()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for PillName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_with_valid_name_returns_pill_name() {
        let result = PillName::new("Aspirin");

        assert!(result.is_ok());
        assert_eq!(result.unwrap().value(), "Aspirin");
    }

    #[test]
    fn new_with_empty_string_returns_error() {
        let result = PillName::new("");

        assert!(matches!(result, Err(DomainError::EmptyPillName)));
    }

    #[test]
    fn new_with_whitespace_only_returns_error() {
        let result = PillName::new("   ");

        assert!(matches!(result, Err(DomainError::EmptyPillName)));
    }

    #[test]
    fn new_trims_surrounding_whitespace() {
        let result = PillName::new("  Ibuprofen  ");

        assert_eq!(result.unwrap().value(), "Ibuprofen");
    }

    #[test]
    fn display_formats_as_pill_name_string() {
        let name = PillName::new("Paracetamol").unwrap();

        assert_eq!(name.to_string(), "Paracetamol");
    }
}
