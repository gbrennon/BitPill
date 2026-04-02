use bitpill::{
    application::dtos::requests::CreateMedicationRequest,
    infrastructure::container::Container,
    presentation::tui::{
        app::App,
        app_services::AppServices,
        handlers::{edit_medication_handler::EditMedicationHandler, port::Handler},
        input::Key,
        screen::Screen,
    },
};
use tempfile::tempdir;

#[test]
fn handle_enter_updates_medication() {
    let dir = tempdir().unwrap();
    let container = Container::new(
        dir.path().join("medications.json"),
        dir.path().join("doses.json"),
        dir.path().join("settings.json"),
    );

    // create initial medication via service
    let req = CreateMedicationRequest::new("Initial", 50, vec![(8, 0)], "OnceDaily".to_string());
    let resp = container
        .create_medication_service
        .execute(req)
        .expect("create failed");
    let med_id = resp.id;

    let services = AppServices::from_container(&container);
    let mut app = App::new(services);
    app.current_screen = Screen::EditMedication {
        id: med_id.clone(),
        name: "UpdatedName".into(),
        amount_mg: "150".into(),
        selected_frequency: 0,
        scheduled_time: vec!["08:00".into()],
        scheduled_idx: 0,
        focused_field: 0,
        insert_mode: false,
    };

    let mut handler = EditMedicationHandler::default();
    handler.handle(&mut app, Key::Enter);

    assert!(matches!(app.current_screen, Screen::HomeScreen));
    assert_eq!(
        app.status_message.as_deref(),
        Some("Medication updated successfully")
    );
}
