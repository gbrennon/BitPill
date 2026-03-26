/// Data transfer object representing a medication returned from the list query.
pub struct MedicationDto {
    pub id: String,
    pub name: String,
    pub amount_mg: u32,
    /// Scheduled administration times as `(hour, minute)` pairs.
    pub scheduled_time: Vec<(u32, u32)>,
    /// Dose frequency as string (e.g. "OnceDaily", "TwiceDaily")
    pub dose_frequency: String,
    /// Number of doses taken today
    pub taken_today: usize,
    /// Total scheduled doses for today
    pub scheduled_today: usize,
}

pub struct ListAllMedicationsResponse {
    pub medications: Vec<MedicationDto>,
}
