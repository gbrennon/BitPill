use crate::application::dtos::requests::ListDoseRecordsRequest;
use crate::application::ports::inbound::list_dose_records_port::ListDoseRecordsPort;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::dtos::responses::MedicationDto;
    use crate::presentation::tui::app::App;
    use crate::presentation::tui::app_services::AppServices;
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    #[test]
    fn render_on_medication_details_screen_does_not_panic() {
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        let mut app = App::new(AppServices::fake());
        app.medications = vec![MedicationDto {
            id: "m1".to_string(),
            name: "Aspirin".to_string(),
            amount_mg: 100,
            dose_frequency: "OnceDaily".to_string(),
            scheduled_time: vec![(8, 0)],
        }];
        app.current_screen = Screen::MedicationDetails {
            id: "m1".to_string(),
        };
        terminal
            .draw(|f| MedicationDetailsRenderer.render(f, &app))
            .unwrap();
        let buffer = terminal.backend().buffer();
        assert!(buffer.content.iter().any(|c| c.symbol() != " "));
    }

    #[test]
    fn render_with_unknown_id_shows_not_found() {
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        let mut app = App::new(AppServices::fake());
        app.current_screen = Screen::MedicationDetails {
            id: "ghost".to_string(),
        };
        terminal
            .draw(|f| MedicationDetailsRenderer.render(f, &app))
            .unwrap();
        let content: String = terminal
            .backend()
            .buffer()
            .content
            .iter()
            .map(|c| c.symbol())
            .collect();
        assert!(content.contains("not found"));
    }

    #[test]
    fn render_on_wrong_screen_returns_without_panic() {
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        let app = App::new(AppServices::fake());
        // HomeScreen — guard clause returns early
        terminal
            .draw(|f| MedicationDetailsRenderer.render(f, &app))
            .unwrap();
    }
}
