use ratatui::Frame;

use crate::presentation::tui::{
    app::App,
    presenters::update_medication_presenter::{
        UpdateMedicationPresenter, UpdateMedicationPresenterDto,
    },
    renderers::ScreenRenderer,
    screen::Screen,
};

const FREQUENCY_OPTIONS: &[&str] = &["Once Daily", "Twice Daily", "Thrice Daily", "Custom"];

pub struct EditMedicationRenderer;

impl ScreenRenderer for EditMedicationRenderer {
    fn render(&self, f: &mut Frame, app: &App) {
        let Screen::EditMedication {
            name,
            amount_mg,
            selected_frequency,
            scheduled_time,
            scheduled_idx,
            focused_field,
            insert_mode,
            ..
        } = &app.current_screen
        else {
            return;
        };

        UpdateMedicationPresenter.present(
            f,
            &UpdateMedicationPresenterDto {
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
