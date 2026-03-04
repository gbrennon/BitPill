// Internal imports first
use crate::application::ports::inbound::list_all_medications_port::MedicationDto;
use crate::presentation::tui::components::table::medication_table;
use crate::presentation::tui::templates::screen_template::ScreenTemplate;

// External crates
use ratatui::Frame;
use ratatui::widgets::ListState;

pub struct MedicationListPresenter;

impl MedicationListPresenter {
    pub fn present(
        &self,
        f: &mut Frame,
        medications: &[MedicationDto],
        selected_index: usize,
        status_message: Option<&String>,
    ) {
        let default_help = if medications.is_empty() {
            " [c] Create  [q] Quit"
        } else {
            " [c] Create  [Enter] Details  [s] Mark Taken  [e] Edit  [d] Delete  [q] Quit"
        };
        let help_text = status_message.map(String::as_str).unwrap_or(default_help);

        let mut state = ListState::default();
        if !medications.is_empty() {
            state.select(Some(selected_index));
        }

        ScreenTemplate {
            subtitle: "Medications",
            help: help_text,
            mode: "NORMAL",
        }
        .render(f, |f, area| {
            let selected = if medications.is_empty() { None } else { Some(selected_index) };
            if medications.is_empty() {
                f.render_widget(medication_table("", &["Name", "mg"], medications, selected), area);
            } else {
                f.render_widget(medication_table("", &["Name", "mg", "Actions"], medications, selected), area);
            }
        });
    }
}
