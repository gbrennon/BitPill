#[derive(Clone)]
pub struct CreateMedicationState {
    pub name: String,
    pub amount_mg: String,
    pub selected_frequency: usize,
    pub scheduled_time: Vec<String>,
    pub scheduled_idx: usize,
    pub focused_field: u8,
    pub insert_mode: bool,
}
