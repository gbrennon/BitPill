pub struct EditMedicationRequest {
    pub id: String,
    pub name: String,
    pub amount_mg: u32,
    pub scheduled_time: Vec<(u32, u32)>,
    pub dose_frequency: String,
}

impl EditMedicationRequest {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        amount_mg: u32,
        scheduled_time: Vec<(u32, u32)>,
        dose_frequency: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            amount_mg,
            scheduled_time,
            dose_frequency: dose_frequency.into(),
        }
    }
}
