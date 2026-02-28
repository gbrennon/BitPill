use std::sync::Arc;
use std::time::Duration;

use crossterm::event::{self, Event};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::application::ports::list_all_medications_port::{
    ListAllMedicationsPort, ListAllMedicationsRequest, MedicationDto,
};
use crate::infrastructure::container::Container;
use crate::presentation::tui::event_handler::EventHandler;
use crate::presentation::tui::screen::Screen;
use crate::presentation::tui::ui;

pub struct App {
    pub container: Arc<Container>,
    pub current_screen: Screen,
    pub medications: Vec<MedicationDto>,
    pub selected_index: usize,
    pub status_message: Option<String>,
    pub should_quit: bool,
}

impl App {
    pub fn new(container: Arc<Container>) -> Self {
        let mut app = Self {
            container,
            current_screen: Screen::MedicationList,
            medications: Vec::new(),
            selected_index: 0,
            status_message: None,
            should_quit: false,
        };
        app.load_medications();
        app
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
            terminal.draw(|f| ui::draw(f, &app))?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    EventHandler::handle(&mut app, key);
                }
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
