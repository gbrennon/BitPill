use crate::domain::{errors::DomainError, value_objects::navigation_mode::NavigationMode};

/// Application-wide settings configuration.
///
/// `AppSettings` is an entity that tracks user preferences such as
/// keyboard navigation mode (vi or emacs). It follows an immutable-update
/// pattern — mutating methods return a new instance rather than modifying
/// the existing one.
///
/// # Examples
///
/// ```rust
/// use bitpill::domain::{
///     entities::app_settings::AppSettings,
///     value_objects::navigation_mode::{NavigationMode, NavigationModeVariant},
/// };
///
/// let settings = AppSettings::new(NavigationMode::new(NavigationModeVariant::Vi).unwrap());
/// assert_eq!(settings.navigation_mode().as_str(), "vi");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct AppSettings {
    navigation_mode: NavigationMode,
}

impl AppSettings {
    /// Creates new `AppSettings` with the given navigation mode.
    pub fn new(navigation_mode: NavigationMode) -> Self {
        Self { navigation_mode }
    }

    /// Returns the current navigation mode.
    pub fn navigation_mode(&self) -> &NavigationMode {
        &self.navigation_mode
    }

    /// Changes the navigation mode, returning new settings (immutable update).
    pub fn change_navigation_mode(
        &self,
        navigation_mode: NavigationMode,
    ) -> Result<Self, DomainError> {
        Ok(Self { navigation_mode })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::navigation_mode::NavigationModeVariant;

    fn vi_mode() -> NavigationMode {
        NavigationMode::new(NavigationModeVariant::Vi).unwrap()
    }

    fn emacs_mode() -> NavigationMode {
        NavigationMode::new(NavigationModeVariant::Emacs).unwrap()
    }

    #[test]
    fn new_sets_navigation_mode() {
        let settings = AppSettings::new(vi_mode());

        assert_eq!(settings.navigation_mode(), &vi_mode());
    }

    #[test]
    fn change_navigation_mode_does_not_mutate_original() {
        let original_mode = vi_mode();
        let new_mode = emacs_mode();

        let settings = AppSettings::new(original_mode.clone());

        let updated = settings.change_navigation_mode(new_mode.clone()).unwrap();

        assert_eq!(updated.navigation_mode(), &new_mode);

        assert_eq!(settings.navigation_mode(), &original_mode);
    }

    #[test]
    fn equality_holds_for_same_state() {
        let a = AppSettings::new(vi_mode());
        let b = AppSettings::new(vi_mode());
        assert_eq!(a, b);
    }

    #[test]
    fn clone_produces_equal_entity() {
        let original = AppSettings::new(vi_mode());
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }
}
