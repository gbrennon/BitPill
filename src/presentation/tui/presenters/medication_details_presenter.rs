// Internal imports first
use crate::application::ports::inbound::list_all_medications_port::MedicationDto;
use crate::presentation::tui::components::detail::medication_detail;
use crate::presentation::tui::templates::screen_template::ScreenTemplate;

// External crates
use chrono::NaiveDateTime;
use ratatui::Frame;

pub struct MedicationDetailsInput<'a> {
    pub medication: Option<&'a MedicationDto>,
    pub taken_at: Vec<NaiveDateTime>,
}

pub struct MedicationDetailsPresenter;

impl MedicationDetailsPresenter {
    pub fn present(&self, f: &mut Frame, input: &MedicationDetailsInput) {
        ScreenTemplate {
            subtitle: "Medication Details",
            help: " [e] Edit  [Esc] Back",
            mode: "NORMAL",
        }
        .render(f, |f, area| {
            if let Some(m) = input.medication {
                f.render_widget(medication_detail(m, &input.taken_at), area);
            }
        });
    }
}
