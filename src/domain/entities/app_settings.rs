use crate::domain::{
    errors::DomainError,
    value_objects::navigation_mode::NavigationMode
};

#[derive(Debug, Clone, PartialEq)]
pub struct AppSettings {
    navigation_mode: NavigationMode,
}

impl AppSettings {
    pub fn new(navigation_mode: NavigationMode) -> Self {
        Self { navigation_mode }
    }

    pub fn navigation_mode(&self) -> &NavigationMode {
        &self.navigation_mode
    }

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
