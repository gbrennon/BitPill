use bitpill::application::dtos::requests::ScheduleDoseRequest;
use bitpill::application::ports::inbound::schedule_dose_port::ScheduleDosePort;
use bitpill::application::services::schedule_dose_service::ScheduleDoseService;
use bitpill::domain::{
    entities::medication::Medication,
    value_objects::{
        dosage::Dosage,
        medication_frequency::DoseFrequency,
        medication_id::MedicationId,
        medication_name::MedicationName,
        scheduled_time::ScheduledTime,
    },
};
use crate::fakes::{
    FakeClock, FakeDoseRecordRepository, FakeMedicationRepository, FakeNotificationPort,
};
use std::sync::Arc;

fn make_medication(name: &str, hour: u32, minute: u32) -> Medication {
    Medication::new(
        MedicationId::generate(),
        MedicationName::new(name).unwrap(),
        Dosage::new(500).unwrap(),
        vec![ScheduledTime::new(hour, minute).unwrap()],
        DoseFrequency::OnceDaily,
    )
}

fn make_service(
    medications: Vec<Medication>,
    clock: FakeClock,
) -> (ScheduleDoseService, Arc<FakeDoseRecordRepository>, Arc<FakeNotificationPort>) {
    let dose_repo = Arc::new(FakeDoseRecordRepository::new());
    let notif = Arc::new(FakeNotificationPort::new());
    let service = ScheduleDoseService::new(
        Arc::new(FakeMedicationRepository::with(medications)),
        dose_repo.clone(),
        notif.clone(),
        Arc::new(clock),
    );
    (service, dose_repo, notif)
}

#[test]
fn execute_with_no_medications_returns_empty_vec() {
    let (service, _, _) = make_service(vec![], FakeClock::at(8, 0));

    let result = service.execute().unwrap();

    assert!(result.is_empty());
}

#[test]
fn execute_with_matching_time_creates_dose_record_and_notifies() {
    let medication = make_medication("Aspirin", 8, 0);
    let (service, dose_repo, notif) = make_service(vec![medication], FakeClock::at(8, 0));

    let result = service.execute().unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(dose_repo.saved_count(), 1);
    assert_eq!(notif.call_count(), 1);
}

#[test]
fn execute_with_non_matching_time_creates_no_records() {
    let medication = make_medication("Aspirin", 8, 0);
    let (service, dose_repo, notif) = make_service(vec![medication], FakeClock::at(20, 0));

    let result = service.execute().unwrap();

    assert!(result.is_empty());
    assert_eq!(dose_repo.saved_count(), 0);
    assert_eq!(notif.call_count(), 0);
}

#[test]
fn execute_notifies_only_medications_due_at_current_time() {
    let aspirin = make_medication("Aspirin", 8, 0);
    let ibuprofen = make_medication("Ibuprofen", 20, 0);
    let (service, dose_repo, notif) = make_service(vec![aspirin, ibuprofen], FakeClock::at(8, 0));

    let result = service.execute().unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(dose_repo.saved_count(), 1);
    assert_eq!(notif.call_count(), 1);
}

#[test]
fn execute_notifies_all_medications_due_at_same_time() {
    let med_a = make_medication("Aspirin", 8, 0);
    let med_b = make_medication("Ibuprofen", 8, 0);
    let (service, dose_repo, notif) = make_service(vec![med_a, med_b], FakeClock::at(8, 0));

    let result = service.execute().unwrap();

    assert_eq!(result.len(), 2);
    assert_eq!(dose_repo.saved_count(), 2);
    assert_eq!(notif.call_count(), 2);
}

#[test]
fn execute_created_record_links_to_correct_medication() {
    let medication = make_medication("Aspirin", 8, 0);
    let medication_id = medication.id().clone();
    let (service, _, _) = make_service(vec![medication], FakeClock::at(8, 0));

    let records = service.execute().unwrap();

    assert_eq!(records[0].medication_id(), &medication_id);
}

#[test]
fn execute_created_record_scheduled_at_matches_clock_now() {
    let medication = make_medication("Aspirin", 8, 0);
    let clock = FakeClock::at(8, 0);
    let expected_now = clock.datetime;
    let (service, _, _) = make_service(vec![medication], clock);

    let records = service.execute().unwrap();

    assert_eq!(records[0].scheduled_at(), expected_now);
}

#[test]
fn execute_medication_with_no_scheduled_time_is_ignored() {
    let medication = Medication::new(
        MedicationId::generate(),
        MedicationName::new("On-demand").unwrap(),
        Dosage::new(100).unwrap(),
        vec![],
        DoseFrequency::OnceDaily,
    );
    let (service, dose_repo, notif) = make_service(vec![medication], FakeClock::at(8, 0));

    let result = service.execute().unwrap();

    assert!(result.is_empty());
    assert_eq!(dose_repo.saved_count(), 0);
    assert_eq!(notif.call_count(), 0);
}

#[test]
fn port_execute_with_matching_medication_returns_dose_record_in_response() {
    let medication = make_medication("Aspirin", 8, 0);
    let (service, dose_repo, _) = make_service(vec![medication], FakeClock::at(8, 0));

    let result = ScheduleDosePort::execute(&service, ScheduleDoseRequest).unwrap();

    assert_eq!(result.created.len(), 1);
    assert_eq!(dose_repo.saved_count(), 1);
}

#[test]
fn port_execute_with_no_medications_returns_empty_response() {
    let (service, _, _) = make_service(vec![], FakeClock::at(8, 0));

    let result = ScheduleDosePort::execute(&service, ScheduleDoseRequest).unwrap();

    assert!(result.created.is_empty());
}

#[test]
fn execute_when_medication_repository_fails_returns_storage_error() {
    use bitpill::application::errors::ApplicationError;
    let dose_repo = Arc::new(FakeDoseRecordRepository::new());
    let notif = Arc::new(FakeNotificationPort::new());
    let service = ScheduleDoseService::new(
        Arc::new(FakeMedicationRepository::failing_on_find_all()),
        dose_repo,
        notif,
        Arc::new(FakeClock::at(8, 0)),
    );

    let result = service.execute();

    assert!(matches!(result, Err(ApplicationError::Storage(_))));
}

#[test]
fn execute_when_dose_record_repository_fails_during_due_medication_returns_storage_error() {
    use bitpill::application::errors::ApplicationError;
    let medication = make_medication("Aspirin", 8, 0);
    let dose_repo = Arc::new(FakeDoseRecordRepository::failing());
    let notif = Arc::new(FakeNotificationPort::new());
    let service = ScheduleDoseService::new(
        Arc::new(FakeMedicationRepository::with(vec![medication])),
        dose_repo,
        notif,
        Arc::new(FakeClock::at(8, 0)),
    );

    let result = service.execute();

    assert!(matches!(result, Err(ApplicationError::Storage(_))));
}

#[test]
fn execute_when_notification_fails_during_due_medication_returns_error() {
    let medication = make_medication("Aspirin", 8, 0);
    let dose_repo = Arc::new(FakeDoseRecordRepository::new());
    let notif = Arc::new(FakeNotificationPort::failing());
    let service = ScheduleDoseService::new(
        Arc::new(FakeMedicationRepository::with(vec![medication])),
        dose_repo,
        notif,
        Arc::new(FakeClock::at(8, 0)),
    );

    let result = service.execute();

    assert!(result.is_err());
}

#[test]
fn port_execute_when_repository_fails_returns_storage_error() {
    use bitpill::application::errors::ApplicationError;
    let dose_repo = Arc::new(FakeDoseRecordRepository::new());
    let notif = Arc::new(FakeNotificationPort::new());
    let service = ScheduleDoseService::new(
        Arc::new(FakeMedicationRepository::failing_on_find_all()),
        dose_repo,
        notif,
        Arc::new(FakeClock::at(8, 0)),
    );

    let result = ScheduleDosePort::execute(&service, ScheduleDoseRequest);

    assert!(matches!(result, Err(ApplicationError::Storage(_))));
}
