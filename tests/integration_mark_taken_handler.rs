use std::sync::Arc;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use bitpill::application::ports::fakes::FakeDoseRecordRepository;
use bitpill::application::ports::outbound::dose_record_repository_port::DoseRecordRepository;
use bitpill::application::services::mark_medication_taken_service::MarkMedicationTakenService;
use bitpill::presentation::tui::app::App;
use bitpill::presentation::tui::handlers::medication_list_handler::MedicationListHandler;
use bitpill::presentation::tui::handlers::port::Handler;
use bitpill::application::ports::inbound::list_all_medications_port::MedicationDto;

#[test]
fn medication_list_handler_saves_taken_dose_record_on_s() {
    let fake_repo = Arc::new(FakeDoseRecordRepository::new());
    let repo_trait: Arc<dyn DoseRecordRepository> = fake_repo.clone();
    let mut container = bitpill::infrastructure::container::Container::new();
    container.mark_medication_taken_service = MarkMedicationTakenService::new(repo_trait);
    let arc = Arc::new(container);

    let mut app = App::new(arc);
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

    assert_eq!(fake_repo.saved_count(), 1);
}
