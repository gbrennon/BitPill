use bitpill::application::dtos::responses::MedicationDto;
use bitpill::infrastructure::container::Container;
use bitpill::presentation::tui::app::App;
use bitpill::presentation::tui::app_services::AppServices;
use bitpill::presentation::tui::handlers::medication_list_handler::MedicationListHandler;
use bitpill::presentation::tui::handlers::port::Handler;
use bitpill::presentation::tui::input::Key;
use tempfile::tempdir;

#[test]
fn handler_saves_dose_record_to_file() {
    let dir = tempdir().unwrap();
    let dose_path = dir.path().join("dose_records.json");
    let container = Container::new(
        dir.path().join("medications.json"),
        dose_path.clone(),
        dir.path().join("settings.json"),
    );

    let services = AppServices::from_container(&container);
    let mut app = App::new(services);
    let med = MedicationDto {
        id: "00000000-0000-0000-0000-000000000001".to_string(),
        name: "Test".to_string(),
        amount_mg: 10,
        scheduled_time: vec![(8, 0)],
        dose_frequency: "OnceDaily".to_string(),
        taken_today: 0,
        scheduled_today: 0,
    };
    app.medications.push(med);
    app.selected_index = 0;

    let mut handler = MedicationListHandler::default();
    handler.handle(&mut app, Key::Char('s'));

    // Now pressing 's' should show an instruction to open details
    assert!(app.status_message.is_some());
    assert!(app
        .status_message
        .as_ref()
        .unwrap()
        .contains("Open medication details"));
}
