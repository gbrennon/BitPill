use ratatui::Frame;

use crate::presentation::tui::{components::settings::settings_view, view_state::SettingsState};

pub struct SettingsPresenter;

impl SettingsPresenter {
    pub fn present(&self, f: &mut Frame, settings_state: &SettingsState) {
        settings_view(f, settings_state.selected_index);
    }
}
