use ratatui::Frame;

use crate::presentation::tui::{
    app::App, presenters::settings_presenter::SettingsPresenter, renderers::ScreenRenderer,
    screen::Screen,
};

pub struct SettingsRenderer;

impl ScreenRenderer for SettingsRenderer {
    fn render(&self, f: &mut Frame, app: &App) {
        let selected_index = match &app.current_screen {
            Screen::Settings { selected_index, .. } => *selected_index,
            Screen::SettingsHelp { selected_index, .. } => *selected_index,
            _ => return,
        };

        let settings_state = crate::presentation::tui::view_state::SettingsState { selected_index };

        SettingsPresenter.present(f, &settings_state);
    }
}
