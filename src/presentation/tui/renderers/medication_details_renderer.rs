use ratatui::Frame;

use crate::{
    application::{
        dtos::requests::ListDoseRecordsRequest,
        ports::inbound::list_dose_records_port::ListDoseRecordsPort,
    },
    presentation::tui::{
        app::App,
        presenters::medication_details_presenter::{
            MedicationDetailsInput, MedicationDetailsPresenter,
        },
        renderers::ScreenRenderer,
        screen::Screen,
    },
};

pub struct MedicationDetailsRenderer;

impl ScreenRenderer for MedicationDetailsRenderer {
    fn render(&self, f: &mut Frame, app: &App) {
        let Screen::MedicationDetails { id } = &app.current_screen else {
            return;
        };

        let medication = app.medications.iter().find(|m| &m.id == id);
        let records = match medication {
            Some(m) => match ListDoseRecordsPort::execute(
                &*app.services.list_dose_records,
                ListDoseRecordsRequest {
                    medication_id: m.id.clone(),
                },
            ) {
                Ok(resp) => resp.records,
                Err(_) => vec![],
            },
            None => vec![],
        };

        MedicationDetailsPresenter.present(
            f,
            &MedicationDetailsInput {
                medication,
                records,
            },
        );
    }
}
