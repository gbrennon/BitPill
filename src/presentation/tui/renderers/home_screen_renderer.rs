use ratatui::Frame;

use crate::presentation::tui::{
    app::App, presenters::medication_list_presenter::MedicationListPresenter,
    renderers::ScreenRenderer,
};

pub struct HomeScreenRenderer;

impl ScreenRenderer for HomeScreenRenderer {
    fn render(&self, f: &mut Frame, app: &App) {
        MedicationListPresenter.present(
            f,
            &app.medications,
            app.selected_index,
            app.status_message.as_ref(),
        );
    }
}
