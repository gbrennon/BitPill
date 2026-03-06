use bitpill::application::ports::fakes::FakeDoseRecordRepository;
use bitpill::application::dtos::responses::MedicationDto;
use bitpill::application::ports::inbound::mark_medication_taken_port::MarkDoseTakenPort;
use bitpill::application::ports::outbound::dose_record_repository_port::DoseRecordRepository;
use bitpill::application::services::mark_medication_taken_service::MarkDoseTakenService;
use bitpill::presentation::tui::app::App;
use bitpill::presentation::tui::app_services::AppServices;
use bitpill::presentation::tui::handlers::medication_list_handler::MedicationListHandler;
use bitpill::presentation::tui::handlers::port::Handler;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::sync::Arc;

#[test]
fn medication_list_handler_saves_taken_dose_record_on_s() {
    let fake_repo = Arc::new(FakeDoseRecordRepository::new());
    let repo_trait: Arc<dyn DoseRecordRepository> = fake_repo.clone();
    let mut container = bitpill::infrastructure::container::Container::new();
    container.mark_medication_taken_service =
        Arc::new(MarkDoseTakenService::new(repo_trait)) as Arc<dyn MarkDoseTakenPort>;
    let services = AppServices::from_container(&container);

    let mut app = App::new(services);
    let med = MedicationDto {
        id: "med-1".to_string(),
        name: "Test".to_string(),
        amount_mg: 10,
        scheduled_time: vec![(8, 0)],
        dose_frequency: "OnceDaily".to_string(),
    };
    app.medications.push(med);
    app.selected_index = 0;

    let mut handler = MedicationListHandler::default();
    let key = KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE);
    handler.handle(&mut app, key);

    // Now pressing 's' on the list should instruct the user to open details to mark as taken.
    assert!(app.status_message.is_some());
    assert!(
        app.status_message
            .as_ref()
            .unwrap()
            .contains("Open medication details")
    );
}
