use crate::presentation::tui::app::App;
use crate::presentation::tui::presenters::mark_dose_presenter::{
    MarkDoseInput, MarkDosePresenter,
};
use crate::presentation::tui::renderers::ScreenRenderer;
use crate::presentation::tui::screen::Screen;
use ratatui::Frame;

pub struct MarkDoseRenderer;

impl ScreenRenderer for MarkDoseRenderer {
    fn render(&self, f: &mut Frame, app: &App) {
        let Screen::MarkDose { medication_id, records, selected_index } = &app.current_screen
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
