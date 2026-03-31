use crate::domain::errors::DomainError;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NavigationModeVariant {
    Vi,
}

impl NavigationModeVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            NavigationModeVariant::Vi => "vi",
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NavigationMode(NavigationModeVariant);

impl NavigationMode {
    pub fn new(variant: NavigationModeVariant) -> Result<Self, DomainError> {
        Ok(Self(variant))
    }

    pub fn value(&self) -> &NavigationModeVariant {
        &self.0
    }

    pub fn as_str(&self) -> &'static str {
        self.0.as_str()
    }
}

impl TryFrom<&str> for NavigationMode {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "vi" => Ok(Self(NavigationModeVariant::Vi)),
            _other => Err(DomainError::InvalidNavigationMode),
        }
    }
}

impl std::fmt::Display for NavigationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_with_valid_variant_returns_ok() {
        let result = NavigationMode::new(NavigationModeVariant::Vi);
        assert!(result.is_ok());
    }

    #[test]
    fn value_returns_inner_variant() {
        let mode = NavigationMode::new(NavigationModeVariant::Vi).unwrap();
        assert_eq!(mode.value(), &NavigationModeVariant::Vi);
    }

    #[test]
    fn as_str_returns_expected_string() {
        let mode = NavigationMode::new(NavigationModeVariant::Vi).unwrap();
        assert_eq!(mode.as_str(), "vi");
    }

    #[test]
    fn try_from_valid_str_returns_ok() {
        let result = NavigationMode::try_from("vi");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "vi");
    }

    #[test]
    fn try_from_invalid_str_returns_err() {
        let result = NavigationMode::try_from("emacs");
        assert!(result.is_err());
    }

    #[test]
    fn display_matches_as_str() {
        let mode = NavigationMode::new(NavigationModeVariant::Vi).unwrap();
        assert_eq!(mode.to_string(), mode.as_str());
    }

    #[test]
    fn equality_holds_for_same_variant() {
        let a = NavigationMode::new(NavigationModeVariant::Vi).unwrap();
        let b = NavigationMode::new(NavigationModeVariant::Vi).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn clone_produces_equal_value() {
        let original = NavigationMode::new(NavigationModeVariant::Vi).unwrap();
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn try_from_invalid_str_returns_error() {
        let result = NavigationMode::try_from("vim");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), "invalid navigation mode");
    }

    #[test]
    fn navigation_mode_variant_as_str() {
        assert_eq!(NavigationModeVariant::Vi.as_str(), "vi");
    }

    #[test]
    fn clone_preserves_variant() {
        let mode = NavigationMode::new(NavigationModeVariant::Vi).unwrap();
        let cloned = NavigationMode::try_from(mode.as_str()).unwrap();
        assert_eq!(mode, cloned);
    }
}
