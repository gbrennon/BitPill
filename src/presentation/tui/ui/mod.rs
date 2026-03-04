use crate::presentation::tui::styles::content_style;
use ratatui::Frame;
use ratatui::widgets::Block;

use crate::presentation::tui::app::App;
use crate::presentation::tui::screen::Screen;

use crate::presentation::tui::presenters::{
    create_medication_presenter::{CreateMedicationPresenter, CreateMedicationPresenterDto},
    medication_details_presenter::{MedicationDetailsInput, MedicationDetailsPresenter},
    medication_list_presenter::MedicationListPresenter,
};

use crate::presentation::tui::components::modal::render_modal;

pub fn draw(f: &mut Frame, app: &App) {
    f.render_widget(Block::default().style(content_style()), f.area());
    match &app.current_screen {
        Screen::HomeScreen => {
            MedicationListPresenter.present(
                f,
                &app.medications,
                app.selected_index,
                app.status_message.as_ref(),
            );
        }

        Screen::CreateMedication {
            name,
            amount_mg,
            selected_frequency,
            scheduled_time,
            scheduled_idx,
            focused_field,
            insert_mode,
        } => CreateMedicationPresenter.present(
            f,
            &CreateMedicationPresenterDto {
                name,
                amount_mg,
                scheduled_time: scheduled_time.as_slice(),
                scheduled_idx: *scheduled_idx,
                focused_field: *focused_field,
                insert_mode: *insert_mode,
                status_message: app.status_message.as_deref(),
                frequency_options: &["Once Daily", "Twice Daily", "Thrice Daily"],
                selected_frequency: *selected_frequency,
            },
        ),
        Screen::EditMedication {
            name,
            amount_mg,
            selected_frequency,
            scheduled_time,
            scheduled_idx,
            focused_field,
            insert_mode,
            ..
        } => CreateMedicationPresenter.present(
            f,
            &CreateMedicationPresenterDto {
                name,
                amount_mg,
                scheduled_time: scheduled_time.as_slice(),
                scheduled_idx: *scheduled_idx,
                focused_field: *focused_field,
                insert_mode: *insert_mode,
                status_message: app.status_message.as_deref(),
                frequency_options: &["Once Daily", "Twice Daily", "Thrice Daily"],
                selected_frequency: *selected_frequency,
            },
        ),
        Screen::MedicationDetails { id } => {
            let medication = app.medications.iter().find(|m| &m.id == id);
            let taken_at = if let Some(m) = medication {
                match crate::application::ports::inbound::list_dose_records_port::ListDoseRecordsPort::execute(
                    &app.container.list_dose_records_service,
                    crate::application::ports::inbound::list_dose_records_port::ListDoseRecordsRequest { medication_id: m.id.clone() },
                ) {
                    Ok(resp) => resp.records.into_iter().filter_map(|r| r.taken_at).collect(),
                    Err(_) => vec![],
                }
            } else { vec![] };
            MedicationDetailsPresenter.present(f, &MedicationDetailsInput { medication, taken_at })
        }
        Screen::MarkDose { medication_id, records, selected_index } => {
            use crate::presentation::tui::presenters::mark_dose_presenter::MarkDosePresenter;
            let presenter = MarkDosePresenter;
            presenter.present(f, &crate::presentation::tui::presenters::mark_dose_presenter::MarkDoseInput { medication_id: medication_id, records: records.as_slice(), selected_index: *selected_index });
        }
        Screen::ConfirmDelete { id: _, name } => {
            let title = "Confirm Delete";
            let content = &format!("Delete medication '{}'?  (y/N)", name);
            render_modal(f, f.area(), title, content);
        }
        Screen::ConfirmQuit { previous: _ } => {
            let title = "Confirm Quit";
            let content = &"Quit application?  (y/N)";
            render_modal(f, f.area(), title, content);
        }
    }
}
