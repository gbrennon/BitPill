// Existing presentation tests
#[path = "presentation/create_medication_handler_enter.rs"]
mod create_medication_handler_enter;

#[path = "presentation/draw_e2e.rs"]
mod draw_e2e;

#[path = "presentation/edit_medication_handler_enter.rs"]
mod edit_medication_handler_enter;

#[path = "presentation/mark_dose_presenter_tests.rs"]
mod mark_dose_presenter_tests;

#[path = "presentation/multiple_screens.rs"]
mod multiple_screens;

#[path = "presentation/presentation_e2e.rs"]
mod presentation_e2e;

#[path = "presentation/presentation_modal_e2e.rs"]
mod presentation_modal_e2e;

#[path = "presentation/render_all_screens.rs"]
mod render_all_screens;

// REST handler e2e tests
#[path = "presentation/rest/handlers/medications_tests.rs"]
mod rest_medications_tests;

#[path = "presentation/rest/handlers/doses_tests.rs"]
mod rest_doses_tests;
