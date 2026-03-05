use crate::presentation::tui::app::App;
use crate::presentation::tui::presenters::create_medication_presenter::{
    CreateMedicationPresenter, CreateMedicationPresenterDto,
};
use crate::presentation::tui::renderers::ScreenRenderer;
use crate::presentation::tui::screen::Screen;
use ratatui::Frame;

const FREQUENCY_OPTIONS: &[&str] = &["Once Daily", "Twice Daily", "Thrice Daily", "Custom"];

pub struct CreateMedicationRenderer;

impl ScreenRenderer for CreateMedicationRenderer {
    fn render(&self, f: &mut Frame, app: &App) {
        let Screen::CreateMedication {
            name,
            amount_mg,
            selected_frequency,
            scheduled_time,
            scheduled_idx,
            focused_field,
            insert_mode,
        } = &app.current_screen
        else {
            return;
        };

        CreateMedicationPresenter.present(
            f,
            &CreateMedicationPresenterDto {
                name,
                amount_mg,
                scheduled_time: scheduled_time.as_slice(),
                scheduled_idx: *scheduled_idx,
                focused_field: *focused_field,
                insert_mode: *insert_mode,
                status_message: app.status_message.as_deref(),
                frequency_options: FREQUENCY_OPTIONS,
                selected_frequency: *selected_frequency,
            },
        );
    }
}
