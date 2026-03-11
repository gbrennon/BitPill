use bitpill::{
    application::dtos::requests::{
        CreateDoseRecordRequest, CreateMedicationRequest, MarkDoseTakenRequest,
    },
    infrastructure::container::Container,
};
use chrono::NaiveDate;
use std::fs;
use tempfile::tempdir;

#[test]
fn create_dose_record_persists_to_disk() {
    let dir = tempdir().unwrap();
    let dose_path = dir.path().join("dose_records.json");
    let container = Container::new_with_paths(
        dir.path().join("medications.json"),
        dose_path.clone(),
        dir.path().join("settings.json"),
    );

    let scheduled_at = NaiveDate::from_ymd_opt(2020, 1, 1)
        .unwrap()
        .and_hms_opt(9, 0, 0)
        .unwrap();
    let req = CreateDoseRecordRequest::new(uuid::Uuid::nil().to_string(), scheduled_at);
    let res = container
        .create_dose_record_service
        .execute(req)
        .expect("create should succeed");

    assert!(!res.id.is_empty());
    let data = fs::read_to_string(&dose_path).unwrap();
    assert!(data.trim().starts_with("["));
}

/// Verifies end-to-end: create a medication, then mark a dose taken via its ID.
/// `MarkDoseTakenService` interprets an unknown DoseRecordId as a MedicationId,
/// creating and persisting a taken record when the medication exists.
#[test]
fn mark_dose_taken_creates_taken_record_when_id_is_medication_id() {
    let dir = tempdir().unwrap();
    let container = Container::new_with_paths(
        dir.path().join("medications.json"),
        dir.path().join("dose_records.json"),
        dir.path().join("settings.json"),
    );

    let med_res = container
        .create_medication_service
        .execute(CreateMedicationRequest::new(
            "TestMed",
            50,
            vec![(8, 0)],
            "OnceDaily",
        ))
        .expect("medication creation should succeed");

    let taken_at = NaiveDate::from_ymd_opt(2025, 6, 1)
        .unwrap()
        .and_hms_opt(8, 0, 0)
        .unwrap();
    let req = MarkDoseTakenRequest::new(med_res.id.clone(), taken_at);
    let res = container
        .mark_dose_taken_service
        .execute(req)
        .expect("marking dose as taken should succeed");

    assert!(!res.record_id.is_empty());
}
