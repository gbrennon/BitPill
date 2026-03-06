pub struct MarkDoseTakenResponse {
    /// UUID string of the dose record that was marked as taken.
    pub record_id: String,
}

impl MarkDoseTakenResponse {
    pub fn new(record_id: impl Into<String>) -> Self {
        Self {
            record_id: record_id.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn response_new_and_fields() {
        let raw_id = "some-id";

        let response = MarkDoseTakenResponse::new("some-id");

        assert_eq!(response.record_id, raw_id);
    }
}
