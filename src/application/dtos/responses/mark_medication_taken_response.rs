pub struct MarkDoseTakenResponse {
    pub id: String,
}

impl MarkDoseTakenResponse {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}
