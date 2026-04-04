use crate::domain::value_objects::navigation_mode::NavigationMode;

#[derive(Clone)]
pub struct SettingsState {
    pub selected_index: usize,
}

impl SettingsState {
    pub fn new(navigation_mode: &NavigationMode) -> Self {
        let variants =
            crate::domain::value_objects::navigation_mode::NavigationModeVariant::variants();
        let selected_index = variants
            .iter()
            .position(|v| v == navigation_mode.value())
            .unwrap_or(0);
        Self { selected_index }
    }

    pub fn count() -> usize {
        crate::domain::value_objects::navigation_mode::NavigationModeVariant::count()
    }

    pub fn toggle(&mut self) {
        self.selected_index = (self.selected_index + 1) % Self::count();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::navigation_mode::NavigationModeVariant;

    #[test]
    fn settings_state_new_with_vi_mode_sets_index_0() {
        let mode = NavigationMode::new(NavigationModeVariant::Vi).unwrap();
        let state = SettingsState::new(&mode);
        assert_eq!(state.selected_index, 0);
    }

    #[test]
    fn settings_state_new_with_emacs_mode_sets_index_1() {
        let mode = NavigationMode::new(NavigationModeVariant::Emacs).unwrap();
        let state = SettingsState::new(&mode);
        assert_eq!(state.selected_index, 1);
    }

    #[test]
    fn settings_state_count_returns_2() {
        assert_eq!(SettingsState::count(), 2);
    }

    #[test]
    fn settings_state_toggle_cycles_from_vi_to_emacs() {
        let mode = NavigationMode::new(NavigationModeVariant::Vi).unwrap();
        let mut state = SettingsState::new(&mode);
        assert_eq!(state.selected_index, 0);
        state.toggle();
        assert_eq!(state.selected_index, 1);
    }

    #[test]
    fn settings_state_toggle_cycles_from_emacs_to_vi() {
        let mode = NavigationMode::new(NavigationModeVariant::Emacs).unwrap();
        let mut state = SettingsState::new(&mode);
        assert_eq!(state.selected_index, 1);
        state.toggle();
        assert_eq!(state.selected_index, 0);
    }

    #[test]
    fn settings_state_toggle_cycles_multiple_times() {
        let mode = NavigationMode::new(NavigationModeVariant::Vi).unwrap();
        let mut state = SettingsState::new(&mode);

        state.toggle();
        assert_eq!(state.selected_index, 1);

        state.toggle();
        assert_eq!(state.selected_index, 0);

        state.toggle();
        assert_eq!(state.selected_index, 1);
    }

    #[test]
    fn settings_state_clone_produces_equal_state() {
        let mode = NavigationMode::new(NavigationModeVariant::Vi).unwrap();
        let state = SettingsState::new(&mode);
        let cloned = state.clone();
        assert_eq!(state.selected_index, cloned.selected_index);
    }
}
