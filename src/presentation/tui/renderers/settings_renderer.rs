use crate::presentation::tui::app::App;
use crate::presentation::tui::presenters::settings_presenter::SettingsPresenter;
use crate::presentation::tui::renderers::ScreenRenderer;
use crate::presentation::tui::screen::Screen;
use ratatui::Frame;

pub struct SettingsRenderer;

impl ScreenRenderer for SettingsRenderer {
    fn render(&self, f: &mut Frame, app: &App) {
        let Screen::Settings { vim_enabled } = &app.current_screen else {
            return;
        };

        SettingsPresenter.present(f, *vim_enabled);
    }
}
