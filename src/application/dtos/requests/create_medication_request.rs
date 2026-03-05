pub struct CreateMedicationRequest {
    pub name: String,
    pub amount_mg: u32,
    pub scheduled_time: Vec<(u32, u32)>,
    pub dose_frequency: String,
}

impl CreateMedicationRequest {
    pub fn new(
        name: impl Into<String>,
        amount_mg: u32,
        scheduled_time: Vec<(u32, u32)>,
        dose_frequency: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            amount_mg,
            scheduled_time,
            dose_frequency: dose_frequency.into(),
        }
    }
}
