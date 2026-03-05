mod fakes;
mod common;

#[path = "services/create_medication_service_tests.rs"]
mod create_medication_service_tests;

#[path = "services/create_dose_record_service_tests.rs"]
mod create_dose_record_service_tests;

#[path = "services/delete_medication_service_tests.rs"]
mod delete_medication_service_tests;

#[path = "services/edit_medication_service_tests.rs"]
mod edit_medication_service_tests;

#[path = "services/get_medication_service_tests.rs"]
mod get_medication_service_tests;

#[path = "services/list_all_medications_service_tests.rs"]
mod list_all_medications_service_tests;

#[path = "services/list_dose_records_service_tests.rs"]
mod list_dose_records_service_tests;

#[path = "services/mark_dose_taken_service_tests.rs"]
mod mark_dose_taken_service_tests;

#[path = "services/mark_medication_taken_service_tests.rs"]
mod mark_medication_taken_service_tests;

#[path = "services/schedule_dose_service_tests.rs"]
mod schedule_dose_service_tests;

#[path = "services/settings_service_tests.rs"]
mod settings_service_tests;

#[path = "services/update_medication_service_tests.rs"]
mod update_medication_service_tests;
