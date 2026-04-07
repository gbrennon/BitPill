use crate::domain::errors::DomainError;

/// Supported keyboard navigation modes.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NavigationModeVariant {
    Emacs,
    Vi,
}

impl NavigationModeVariant {
    /// Returns the string identifier for this variant.
    pub fn as_str(&self) -> &'static str {
        match self {
            NavigationModeVariant::Vi => "vi",
            NavigationModeVariant::Emacs => "emacs",
        }
    }

    /// Returns `true` if this is the vi variant.
    pub fn is_vi(&self) -> bool {
        matches!(self, NavigationModeVariant::Vi)
    }

    /// Returns all supported navigation mode variants.
    pub fn variants() -> &'static [NavigationModeVariant] {
        &[NavigationModeVariant::Vi, NavigationModeVariant::Emacs]
    }

    /// Returns the number of supported variants.
    pub fn count() -> usize {
        Self::variants().len()
    }

    /// Gets a variant by index (0-based).
    pub fn from_index(index: usize) -> Option<Self> {
        Self::variants().get(index).cloned()
    }

    /// Returns help text describing the key bindings for this mode.
    pub fn help_text(&self) -> &'static str {
        match self {
            NavigationModeVariant::Vi => {
                "VI MODE: j/k or ↑/↓ to move, l or → to next, h or ← to prev, i insert, Esc normal"
            }
            NavigationModeVariant::Emacs => {
                "EMACS MODE: C-n/C-p or ↑/↓ to move, C-f/C-b or ←/→ to move, C-a/C-e line edges"
            }
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
            "emacs" => Ok(Self(NavigationModeVariant::Emacs)),
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
    fn try_from_vi_str_returns_ok() {
        let result = NavigationMode::try_from("vi");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "vi");
    }

    #[test]
    fn try_from_emacs_str_returns_ok() {
        let result = NavigationMode::try_from("emacs");

        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "emacs");
    }

    #[test]
    fn try_from_invalid_str_returns_err() {
        let result = NavigationMode::try_from("invalid");
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
