use bitpill::presentation::tui::{
    renderers::{ScreenRenderer, settings_renderer::SettingsRenderer},
    screen::Screen,
};
use ratatui::{Terminal, backend::TestBackend};

use crate::helpers::make_app;

#[test]
fn render_settings_does_not_panic() {
    let mut t = Terminal::new(TestBackend::new(80, 24)).unwrap();
    let app = make_app(Screen::Settings {
        vim_enabled: true,
        selected_index: 0,
    });
    t.draw(|f| SettingsRenderer.render(f, &app)).unwrap();
}

#[test]
fn render_settings_help_does_not_panic() {
    let mut t = Terminal::new(TestBackend::new(80, 24)).unwrap();
    let prev = Box::new(Screen::HomeScreen);
    let app = make_app(Screen::SettingsHelp {
        vim_enabled: true,
        selected_index: 0,
        help_text: "help".into(),
        previous: prev,
    });
    t.draw(|f| SettingsRenderer.render(f, &app)).unwrap();
}

#[test]
fn render_settings_vim_disabled_does_not_panic() {
    let mut t = Terminal::new(TestBackend::new(80, 24)).unwrap();
    let app = make_app(Screen::Settings {
        vim_enabled: false,
        selected_index: 0,
    });
    t.draw(|f| SettingsRenderer.render(f, &app)).unwrap();
}
