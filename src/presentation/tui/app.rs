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
    /// If `event_source` is None, uses RealEventSource (requires TTY).
    pub fn tick<B: ratatui::backend::Backend>(
        terminal: &mut Terminal<B>,
        app: &mut App,
        event_source: Option<&dyn EventSource>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        terminal.draw(|f| draw::draw(f, app))?;

        let es = event_source.unwrap_or(&RealEventSource);
        if es.poll(Duration::from_millis(100))? {
            match es.read_key()? {
                Key::Other => {}
                key => {
                    let mut event_handler = EventHandler::default();
                    event_handler.handle(app, key);
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
    /// If `event_source` is None, uses RealEventSource (requires TTY).
    pub fn run_with<B: ratatui::backend::Backend>(
        terminal: &mut Terminal<B>,
        app: &mut App,
        event_source: Option<&dyn EventSource>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            if App::tick(terminal, app, event_source)? {
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

        App::run_with(&mut terminal, &mut app, None)?;

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

        Ok(())
    }
}
