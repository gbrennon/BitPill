use crate::application::dtos::requests::MarkMedicationTakenRequest;
use crate::application::dtos::responses::MarkMedicationTakenResponse;
use crate::application::errors::ApplicationError;

/// Inbound port: mark a medication dose as taken by creating a DoseRecord immediately.
pub trait MarkMedicationTakenPort: Send + Sync {
    fn execute(
        &self,
        request: MarkMedicationTakenRequest,
    ) -> Result<MarkMedicationTakenResponse, ApplicationError>;
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
        let req = MarkMedicationTakenRequest::new("med-id", dt);
        assert_eq!(req.medication_id, "med-id");
        assert_eq!(req.taken_at, dt);
        let res = MarkMedicationTakenResponse::new("new-id");
        assert_eq!(res.id, "new-id");
    }
}
