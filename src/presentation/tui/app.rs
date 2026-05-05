use std::{sync::Arc, time::Duration};

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::{
    application::dtos::{requests::ListAllMedicationsRequest, responses::MedicationDto},
    infrastructure::container::Container,
    presentation::tui::{
        app_services::AppServices,
        draw,
        event_source::{EventSource, RealEventSource},
        handlers::{event_handler::EventHandler, port::Handler},
        input::Key,
        screen::Screen,
    },
};

pub struct App {
    pub services: AppServices,
    pub current_screen: Screen,
    pub medications: Vec<MedicationDto>,
    pub selected_index: usize,
    pub status_message: Option<String>,
    pub status_expires_at: Option<std::time::Instant>,
    pub should_quit: bool,
    pub show_welcome_modal: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            services: AppServices::default(),
            current_screen: Screen::HomeScreen,
            medications: Vec::new(),
            selected_index: 0,
            status_message: None,
            status_expires_at: None,
            should_quit: false,
            show_welcome_modal: false,
        }
    }
}

impl App {
    pub fn new(services: AppServices) -> Self {
        let mut app = Self {
            services,
            current_screen: Screen::HomeScreen,
            medications: Vec::new(),
            selected_index: 0,
            status_message: None,
            status_expires_at: None,
            should_quit: false,
            show_welcome_modal: false,
        };
        app.load_medications();
        app
    }

    pub fn get_navigation_mode(
        &self,
    ) -> Option<crate::domain::value_objects::navigation_mode::NavigationModeVariant> {
        use crate::application::{
            dtos::requests::GetSettingsRequest, ports::inbound::get_settings_port::GetSettingsPort,
        };

        match GetSettingsPort::execute(&*self.services.get_settings, GetSettingsRequest {}) {
            Ok(settings) => {
                crate::domain::value_objects::navigation_mode::NavigationMode::try_from(
                    settings.navigation_mode.as_str(),
                )
                .ok()
                .map(|m| m.value().clone())
            }
            Err(_) => None,
        }
    }

    pub fn is_vim_mode(&self) -> bool {
        self.get_navigation_mode()
            .map(|m| m.is_vi())
            .unwrap_or(true)
    }

    /// Set a temporary status message that will expire after `duration_ms` milliseconds.
    pub fn set_status(&mut self, msg: impl Into<String>, duration_ms: u64) {
        self.status_message = Some(msg.into());
        self.status_expires_at =
            Some(std::time::Instant::now() + Duration::from_millis(duration_ms));
    }

    pub fn clear_status(&mut self) {
        self.status_message = None;
        self.status_expires_at = None;
    }

    pub fn pop_screen(&mut self) {
        self.current_screen = Screen::HomeScreen;
    }

    pub fn load_medications(&mut self) {
        match self
            .services
            .list_all_medications
            .execute(ListAllMedicationsRequest)
        {
            Ok(resp) => self.medications = resp.medications,
            Err(e) => self.status_message = Some(format!("Error loading medications: {e}")),
        }
    }

    /// Runs one iteration of the event loop. Returns `true` if the app should quit.
    /// If `key` is None, polls from RealEventSource (requires TTY).
    pub fn tick<B: ratatui::backend::Backend>(
        terminal: &mut Terminal<B>,
        app: &mut App,
        key: Option<Key>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        terminal.draw(|f| draw::draw(f, app))?;

        if let Some(k) = key {
            let mut event_handler = EventHandler::default();
            event_handler.handle(app, k);
        } else if RealEventSource.poll(Duration::from_millis(100))? {
            match RealEventSource.read_key()? {
                Key::Other => {}
                k => {
                    let mut event_handler = EventHandler::default();
                    event_handler.handle(app, k);
                }
            }
        }

        if let Some(exp) = app.status_expires_at
            && std::time::Instant::now() >= exp
        {
            app.clear_status();
        }

        Ok(app.should_quit)
    }

    /// Inner event loop, generic over the backend for testability.
    /// If `key` is None, polls from RealEventSource.
    pub fn run_with<B: ratatui::backend::Backend>(
        terminal: &mut Terminal<B>,
        app: &mut App,
    ) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            if App::tick(terminal, app, None)? {
                break;
            }
        }
        Ok(())
    }

    pub fn run(container: Arc<Container>) -> Result<(), Box<dyn std::error::Error>> {
        let services = AppServices::from_container(&container);

        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let mut app = App::new(services);

        App::run_with(&mut terminal, &mut app)?;

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::presentation::tui::{app_services::AppServices, input::Key};

    struct FakeGetSettings;
    impl crate::application::ports::inbound::get_settings_port::GetSettingsPort for FakeGetSettings {
        fn execute(
            &self,
            _: crate::application::dtos::requests::GetSettingsRequest,
        ) -> Result<
            crate::application::dtos::responses::GetSettingsResponse,
            crate::application::errors::ApplicationError,
        > {
            Ok(crate::application::dtos::responses::GetSettingsResponse {
                navigation_mode: "vi".into(),
            })
        }
    }

    #[test]
    fn app_status_and_navigation() {
        let mut app = App::default();
        app.set_status("hi", 10);
        assert!(app.status_message.is_some());
        app.clear_status();
        assert!(app.status_message.is_none());

        // is_vim_mode depends on get_settings; default returns none -> true
        assert!(app.is_vim_mode());

        // get_navigation_mode with fake service
        app.services.get_settings = Arc::new(FakeGetSettings);
        let mode = app.get_navigation_mode();
        assert!(mode.is_some());
    }

    #[test]
    fn tick_with_key_invokes_handler() {
        use ratatui::{Terminal, backend::TestBackend};
        let mut app = App::default();
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let res = App::tick(&mut terminal, &mut app, Some(Key::Char('x')));
        assert!(res.is_ok());
    }
}
