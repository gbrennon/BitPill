use std::sync::Arc;
use std::time::Duration;

use crossterm::event::{self, Event};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use crate::application::dtos::requests::ListAllMedicationsRequest;
use crate::application::dtos::responses::MedicationDto;
use crate::infrastructure::container::Container;
use crate::presentation::tui::app_services::AppServices;
use crate::presentation::tui::draw;
use crate::presentation::tui::handlers::event_handler::EventHandler;
use crate::presentation::tui::handlers::port::Handler;
use crate::presentation::tui::screen::Screen;

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

    #[cfg(test)]
    pub fn new_fake() -> Self {
        App::new(crate::presentation::tui::app_services::AppServices::fake())
    }

    /// Runs one iteration of the event loop. Returns `true` if the app should quit.
    pub fn tick<B: ratatui::backend::Backend>(
        terminal: &mut Terminal<B>,
        app: &mut App,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        terminal.draw(|f| draw::draw(f, app))?;

        if event::poll(Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
        {
            let mut event_handler = EventHandler::default();
            event_handler.handle(app, key);
        }

        if let Some(exp) = app.status_expires_at && std::time::Instant::now() >= exp {
            app.clear_status();
        }

        Ok(app.should_quit)
    }

    /// Inner event loop, generic over the backend for testability.
    pub fn run_with<B: ratatui::backend::Backend>(
        terminal: &mut Terminal<B>,
        app: &mut App,
    ) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            if App::tick(terminal, app)? {
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
    use std::time::Duration;

    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    use crate::application::dtos::requests::ListAllMedicationsRequest;
    use crate::application::dtos::responses::ListAllMedicationsResponse;
    use crate::application::errors::{ApplicationError, StorageError};
    use crate::application::ports::inbound::list_all_medications_port::ListAllMedicationsPort;
    use crate::presentation::tui::app_services::AppServices;
    use crate::presentation::tui::screen::Screen;

    use super::*;

    struct ErrorListAllMedicationsPort;
    impl ListAllMedicationsPort for ErrorListAllMedicationsPort {
        fn execute(
            &self,
            _: ListAllMedicationsRequest,
        ) -> Result<ListAllMedicationsResponse, ApplicationError> {
            Err(ApplicationError::Storage(StorageError("test error".to_string())))
        }
    }

    fn app_with_error_list_port() -> App {
        let services = AppServices {
            list_all_medications: Arc::new(ErrorListAllMedicationsPort),
            ..AppServices::fake()
        };
        App::new(services)
    }

    #[test]
    fn new_starts_on_home_screen() {
        let app = App::new_fake();
        assert!(matches!(app.current_screen, Screen::HomeScreen));
        assert!(!app.should_quit);
    }

    #[test]
    fn set_status_stores_message_and_expiry() {
        let mut app = App::new_fake();
        app.set_status("hello", 500);
        assert_eq!(app.status_message.as_deref(), Some("hello"));
        assert!(app.status_expires_at.is_some());
    }

    #[test]
    fn clear_status_removes_message_and_expiry() {
        let mut app = App::new_fake();
        app.set_status("hello", 500);
        app.clear_status();
        assert!(app.status_message.is_none());
        assert!(app.status_expires_at.is_none());
    }

    #[test]
    fn load_medications_populates_list() {
        let mut app = App::new_fake();
        app.medications.clear();
        app.load_medications();
        // FakeListAllMedicationsPort returns an empty list – load should not error
        assert!(app.status_message.is_none());
    }

    #[test]
    fn load_medications_sets_status_message_when_port_returns_error() {
        let app = app_with_error_list_port();

        assert!(app.status_message.is_some());
        assert!(app.status_message.as_deref().unwrap().contains("Error loading medications"));
    }

    #[test]
    fn run_with_exits_immediately_when_should_quit_is_true() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut app = App::new_fake();
        app.should_quit = true;

        App::run_with(&mut terminal, &mut app).unwrap();
    }

    #[test]
    fn run_with_clears_expired_status_before_exiting() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut app = App::new_fake();
        app.set_status("expiring", 1);
        std::thread::sleep(Duration::from_millis(10));
        app.should_quit = true;

        App::run_with(&mut terminal, &mut app).unwrap();

        assert!(app.status_message.is_none());
    }

    #[test]
    fn run_with_preserves_status_that_has_not_yet_expired() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut app = App::new_fake();
        app.set_status("still active", 60_000);
        app.should_quit = true;

        App::run_with(&mut terminal, &mut app).unwrap();

        assert_eq!(app.status_message.as_deref(), Some("still active"));
    }

    #[test]
    fn tick_returns_false_when_should_quit_is_false() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut app = App::new_fake();
        app.should_quit = false;

        let quit = App::tick(&mut terminal, &mut app).unwrap();

        assert!(!quit);
    }

    #[test]
    fn tick_returns_true_when_should_quit_is_true() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut app = App::new_fake();
        app.should_quit = true;

        let quit = App::tick(&mut terminal, &mut app).unwrap();

        assert!(quit);
    }
}

