use crate::presentation::tui::components::medication_form::render_medication_form;
use ratatui::Frame;

pub struct UpdateMedicationPresenter;

pub struct UpdateMedicationPresenterDto<'a> {
    pub name: &'a str,
    pub amount_mg: &'a str,
    pub scheduled_time: &'a [String],
    pub scheduled_idx: usize,
    pub focused_field: u8,
    pub insert_mode: bool,
    pub status_message: Option<&'a str>,
    pub frequency_options: &'a [&'a str],
    pub selected_frequency: usize,
}

impl UpdateMedicationPresenter {
    pub fn present(&self, f: &mut Frame, dto: &UpdateMedicationPresenterDto) {
        let subtitle = if dto.insert_mode {
            "Edit Medication (INSERT)"
        } else {
            "Edit Medication"
        };
        render_medication_form(
            f,
            subtitle,
            dto.name,
            dto.amount_mg,
            dto.scheduled_time,
            dto.scheduled_idx,
            dto.focused_field,
            dto.insert_mode,
            dto.status_message,
            dto.frequency_options,
            dto.selected_frequency,
        );
    }
}
