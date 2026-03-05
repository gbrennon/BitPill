use std::sync::Arc;
use std::time::Duration;

use crossterm::event::{self, Event};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use crate::application::ports::list_all_medications_port::{
    ListAllMedicationsPort, ListAllMedicationsRequest, MedicationDto,
};
use crate::infrastructure::container::Container;
use crate::presentation::tui::draw;
use crate::presentation::tui::handlers::event_handler::EventHandler;
use crate::presentation::tui::handlers::port::Handler;
use crate::presentation::tui::screen::Screen;

pub struct App {
    pub container: Arc<Container>,
    pub current_screen: Screen,
    pub medications: Vec<MedicationDto>,
    pub selected_index: usize,
    pub status_message: Option<String>,
    pub status_expires_at: Option<std::time::Instant>,
    pub should_quit: bool,
    pub show_welcome_modal: bool,
}

impl App {
    pub fn new(container: Arc<Container>) -> Self {
        let mut app = Self {
            container,
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
            .container
            .list_all_medications_service
            .execute(ListAllMedicationsRequest)
        {
            Ok(resp) => self.medications = resp.medications,
            Err(e) => self.status_message = Some(format!("Error loading medications: {e}")),
        }
    }

    pub fn run(container: Arc<Container>) -> Result<(), Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let mut app = App::new(container);

        loop {
            terminal.draw(|f| draw::draw(f, &app))?;

            if event::poll(Duration::from_millis(100))?
                && let Event::Key(key) = event::read()?
            {
                let mut event_handler = EventHandler::default();
                event_handler.handle(&mut app, key);
            }

            // Clear temporary status messages when expired
            if let Some(exp) = app.status_expires_at && std::time::Instant::now() >= exp {
                app.clear_status();
            }

            if app.should_quit {
                break;
            }
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

        Ok(())
    }
}
