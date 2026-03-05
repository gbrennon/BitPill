use crate::presentation::tui::app::App;
use crate::presentation::tui::presenters::medication_list_presenter::MedicationListPresenter;
use crate::presentation::tui::renderers::ScreenRenderer;
use ratatui::Frame;

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
