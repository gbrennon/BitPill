use crate::application::ports::inbound::list_dose_records_port::DoseRecordDto;

#[derive(Clone)]
pub struct MarkDoseState {
    pub medication_id: String,
    pub records: Vec<DoseRecordDto>,
    pub selected_index: usize,
}
