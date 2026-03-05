use crate::application::dtos::requests::MarkDoseTakenRequest;
use crate::application::dtos::responses::MarkDoseTakenResponse;
use crate::application::errors::ApplicationError;

pub trait MarkDoseTakenPort: Send + Sync {
    fn execute(
        &self,
        request: MarkDoseTakenRequest,
    ) -> Result<MarkDoseTakenResponse, ApplicationError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn request_and_response_new_and_fields() {
        let dt = NaiveDate::from_ymd_opt(2026, 3, 5)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap();
        let req = MarkDoseTakenRequest::new("some-id", dt);
        assert_eq!(req.record_id, "some-id");
        assert_eq!(req.taken_at, dt);
        let res = MarkDoseTakenResponse::new("rid");
        assert_eq!(res.record_id, "rid");
    }
}
