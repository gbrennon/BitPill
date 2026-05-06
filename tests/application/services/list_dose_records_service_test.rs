use std::sync::Arc;

use bitpill::{
    application::{
        dtos::requests::ListDoseRecordsRequest, errors::ApplicationError,
        ports::inbound::list_dose_records_port::ListDoseRecordsPort,
        services::list_dose_records_service::ListDoseRecordsService,
    },
    domain::{entities::dose_record::DoseRecord, value_objects::medication_id::MedicationId},
};
use chrono::NaiveDate;

use crate::fakes::FakeDoseRecordRepository;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_dose_records_returns_records_for_medication() {
        let med_id = MedicationId::generate();
        let record = DoseRecord::new(
            med_id.clone(),
            NaiveDate::from_ymd_opt(2025, 1, 1)
                .unwrap()
                .and_hms_opt(9, 0, 0)
                .unwrap(),
        );
        let repo = Arc::new(FakeDoseRecordRepository::with(record.clone()));
        let service = ListDoseRecordsService::new(repo);

        let req = ListDoseRecordsRequest {
            medication_id: med_id.to_string(),
        };
        let res = service.execute(req).expect("should list records");

        assert_eq!(res.records.len(), 1);
        assert_eq!(res.records[0].id, record.id().to_string());
    }

    #[test]
    fn list_dose_records_returns_newest_first() {
        let med_id = MedicationId::generate();

        let oldest = DoseRecord::new(
            med_id.clone(),
            NaiveDate::from_ymd_opt(2025, 1, 1)
                .unwrap()
                .and_hms_opt(8, 0, 0)
                .unwrap(),
        );
        let middle = DoseRecord::new(
            med_id.clone(),
            NaiveDate::from_ymd_opt(2025, 1, 1)
                .unwrap()
                .and_hms_opt(12, 0, 0)
                .unwrap(),
        );
        let newest = DoseRecord::new(
            med_id.clone(),
            NaiveDate::from_ymd_opt(2025, 1, 1)
                .unwrap()
                .and_hms_opt(18, 0, 0)
                .unwrap(),
        );

        // Pushed in chronological order (oldest first) — the fake repo preserves insertion order.
        let repo = Arc::new(FakeDoseRecordRepository::with_records(vec![
            oldest.clone(),
            middle.clone(),
            newest.clone(),
        ]));
        let service = ListDoseRecordsService::new(repo);

        let req = ListDoseRecordsRequest {
            medication_id: med_id.to_string(),
        };
        let res = service.execute(req).expect("should list records");

        assert_eq!(res.records.len(), 3, "should return all 3 records");
        // Service reverses repository order: newest (last pushed) must be first.
        assert_eq!(
            res.records[0].scheduled_at,
            newest.scheduled_at(),
            "first record must be the most recent dose"
        );
        assert_eq!(
            res.records[1].scheduled_at,
            middle.scheduled_at(),
            "second record must be the middle dose"
        );
        assert_eq!(
            res.records[2].scheduled_at,
            oldest.scheduled_at(),
            "third record must be the oldest dose"
        );
    }

    #[test]
    fn list_dose_records_invalid_medication_id_returns_invalid_input() {
        let repo = Arc::new(FakeDoseRecordRepository::new());
        let service = ListDoseRecordsService::new(repo);

        let req = ListDoseRecordsRequest {
            medication_id: "not-a-uuid".into(),
        };
        let res = service.execute(req);

        assert!(matches!(res, Err(ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn list_dose_records_when_repository_fails_returns_storage_error() {
        let repo = Arc::new(FakeDoseRecordRepository::failing_on_find_all_by_medication());
        let service = ListDoseRecordsService::new(repo);
        let med_id = MedicationId::generate().to_string();

        let res = service.execute(ListDoseRecordsRequest {
            medication_id: med_id,
        });

        assert!(matches!(res, Err(ApplicationError::Storage(_))));
    }
}
