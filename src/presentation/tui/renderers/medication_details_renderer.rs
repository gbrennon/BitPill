use crate::application::ports::inbound::list_dose_records_port::{
    ListDoseRecordsPort, ListDoseRecordsRequest,
};
use crate::presentation::tui::app::App;
use crate::presentation::tui::presenters::medication_details_presenter::{
    MedicationDetailsInput, MedicationDetailsPresenter,
};
use crate::presentation::tui::renderers::ScreenRenderer;
use crate::presentation::tui::screen::Screen;
use ratatui::Frame;

pub struct MedicationDetailsRenderer;

impl ScreenRenderer for MedicationDetailsRenderer {
    fn render(&self, f: &mut Frame, app: &App) {
        let Screen::MedicationDetails { id } = &app.current_screen else {
            return;
        };

        let medication = app.medications.iter().find(|m| &m.id == id);
        let records = match medication {
            Some(m) => match ListDoseRecordsPort::execute(
                &app.container.list_dose_records_service,
                ListDoseRecordsRequest { medication_id: m.id.clone() },
            ) {
                Ok(resp) => resp.records,
                Err(_) => vec![],
            },
            None => vec![],
        };

        MedicationDetailsPresenter.present(f, &MedicationDetailsInput { medication, records });
    }
}
