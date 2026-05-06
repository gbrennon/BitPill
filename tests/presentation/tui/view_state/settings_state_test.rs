use bitpill::{
    domain::value_objects::navigation_mode::{NavigationMode, NavigationModeVariant},
    presentation::tui::view_state::settings_state::SettingsState,
};

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
