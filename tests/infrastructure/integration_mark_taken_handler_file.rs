use bitpill::{
    application::dtos::responses::MedicationDto,
    infrastructure::container::Container,
    presentation::tui::{
        app::App,
        app_services::AppServices,
        handlers::{medication_list_handler::MedicationListHandler, port::Handler},
        input::Key,
        screen::Screen,
    },
};
use tempfile::tempdir;

#[test]
fn handler_opens_mark_dose_screen_on_m() {
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
    handler.handle(&mut app, Key::Char('m'));

    assert!(matches!(app.current_screen, Screen::MarkDose { .. }));
}
