use bitpill::{
    application::dtos::responses::MedicationDto,
    infrastructure::container::Container,
    presentation::tui::{
        app::App,
        app_services::AppServices,
        handlers::{medication_list_handler::MedicationListHandler, port::Handler},
        input::Key,
    },
};
use tempfile::tempdir;

#[test]
fn medication_list_handler_saves_taken_dose_record_on_m() {
    let dir = tempdir().unwrap();
    let container = Container::new(
        dir.path().join("meds.json"),
        dir.path().join("doses.json"),
        dir.path().join("settings.json"),
    );
    let services = AppServices::from_container(&container);

    let mut app = App::new(services);
    let med = MedicationDto {
        id: "med-1".to_string(),
        name: "Test".to_string(),
        amount_mg: 10,
        scheduled_time: vec![(8, 0)],
        dose_frequency: "OnceDaily".to_string(),
        taken_today: 0,
        scheduled_today: 0,
    };
    app.medications.push(med);

    let mut handler = MedicationListHandler;
    handler.handle(&mut app, Key::from(Key::Char('m')));

    assert!(app.status_message.is_some());
}
