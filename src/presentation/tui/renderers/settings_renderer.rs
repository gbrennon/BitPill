use ratatui::Frame;

use crate::presentation::tui::{
    app::App, presenters::settings_presenter::SettingsPresenter, renderers::ScreenRenderer,
    screen::Screen,
};

pub struct SettingsRenderer;

impl ScreenRenderer for SettingsRenderer {
    fn render(&self, f: &mut Frame, app: &App) {
        let Screen::Settings { vim_enabled } = &app.current_screen else {
            return;
        };

        SettingsPresenter.present(f, *vim_enabled);
    }
}
