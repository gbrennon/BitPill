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
    fn list_dose_records_returns_newest_taken_first() {
        let med_id = MedicationId::generate();
        let base = NaiveDate::from_ymd_opt(2025, 1, 1)
            .unwrap()
            .and_hms_opt(8, 0, 0)
            .unwrap();

        let mut morning = DoseRecord::new(med_id.clone(), base);
        let mut afternoon = DoseRecord::new(med_id.clone(), base);
        let mut evening = DoseRecord::new(med_id.clone(), base);

        let date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        morning
            .mark_taken(date.and_hms_opt(9, 0, 0).unwrap())
            .unwrap();
        afternoon
            .mark_taken(date.and_hms_opt(12, 0, 0).unwrap())
            .unwrap();
        evening
            .mark_taken(date.and_hms_opt(18, 0, 0).unwrap())
            .unwrap();

        // Pushed in arbitrary order — service sorts by taken_at descending.
        let repo = Arc::new(FakeDoseRecordRepository::with_records(vec![
            afternoon, evening, morning,
        ]));
        let service = ListDoseRecordsService::new(repo);

        let req = ListDoseRecordsRequest {
            medication_id: med_id.to_string(),
        };
        let res = service.execute(req).expect("should list records");

        assert_eq!(res.records.len(), 3, "should return all 3 records");
        // Most recently taken first
        assert_eq!(
            res.records[0].taken_at,
            Some(
                NaiveDate::from_ymd_opt(2025, 1, 1)
                    .unwrap()
                    .and_hms_opt(18, 0, 0)
                    .unwrap()
            ),
            "first record must have the latest taken_at"
        );
        assert_eq!(
            res.records[1].taken_at,
            Some(
                NaiveDate::from_ymd_opt(2025, 1, 1)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap()
            ),
            "second record must have the middle taken_at"
        );
        assert_eq!(
            res.records[2].taken_at,
            Some(
                NaiveDate::from_ymd_opt(2025, 1, 1)
                    .unwrap()
                    .and_hms_opt(9, 0, 0)
                    .unwrap()
            ),
            "third record must have the earliest taken_at"
        );
    }

    #[test]
    fn list_dose_records_returns_none_taken_at_last() {
        let med_id = MedicationId::generate();
        let base = NaiveDate::from_ymd_opt(2025, 1, 1)
            .unwrap()
            .and_hms_opt(8, 0, 0)
            .unwrap();

        let mut taken = DoseRecord::new(med_id.clone(), base);
        taken
            .mark_taken(
                NaiveDate::from_ymd_opt(2025, 1, 1)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap(),
            )
            .unwrap();
        let not_taken = DoseRecord::new(med_id.clone(), base);

        let repo = Arc::new(FakeDoseRecordRepository::with_records(vec![
            taken, not_taken,
        ]));
        let service = ListDoseRecordsService::new(repo);

        let req = ListDoseRecordsRequest {
            medication_id: med_id.to_string(),
        };
        let res = service.execute(req).expect("should list records");

        assert_eq!(res.records.len(), 2);
        assert!(
            res.records[0].taken_at.is_some(),
            "taken records must appear before not-taken ones"
        );
        assert!(
            res.records[1].taken_at.is_none(),
            "not-taken records must appear last"
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
