use crate::application::dtos::responses::DoseRecordDto;

#[derive(Clone)]
pub struct MarkDoseState {
    pub medication_id: String,
    pub records: Vec<DoseRecordDto>,
    pub selected_index: usize,
}
