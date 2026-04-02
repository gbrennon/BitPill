use ratatui::Frame;

use crate::presentation::tui::{
    app::App,
    presenters::mark_dose_presenter::{MarkDoseInput, MarkDosePresenter},
    renderers::ScreenRenderer,
    screen::Screen,
};

pub struct MarkDoseRenderer;

impl ScreenRenderer for MarkDoseRenderer {
    fn render(&self, f: &mut Frame, app: &App) {
        let Screen::MarkDose {
            medication_id,
            records,
            selected_index,
        } = &app.current_screen
        else {
            return;
        };

        MarkDosePresenter.present(
            f,
            &MarkDoseInput {
                medication_id,
                records: records.as_slice(),
                selected_index: *selected_index,
            },
        );
    }
}
