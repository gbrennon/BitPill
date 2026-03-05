pub struct MarkMedicationTakenResponse {
    pub id: String,
}

impl MarkMedicationTakenResponse {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}
