use bitpill::{
    application::{
        dtos::responses::MedicationDto,
        ports::{
            fakes::{FakeDoseRecordRepository, FakeMedicationRepository},
            inbound::mark_dose_taken_port::MarkDoseTakenPort,
            outbound::{
                dose_record_repository_port::DoseRecordRepository,
                medication_repository_port::MedicationRepository,
            },
        },
        services::mark_dose_taken_service::MarkDoseTakenService,
    },
    infrastructure::container::Container,
    presentation::tui::{
        app::App, app_services::AppServices, input::Key,
        handlers::medication_list_handler::MedicationListHandler, handlers::port::Handler,
    },
};
use std::sync::Arc;

#[test]
fn medication_list_handler_saves_taken_dose_record_on_s() {
    let fake_dose_repo: Arc<dyn DoseRecordRepository> = Arc::new(FakeDoseRecordRepository::new());
    let fake_med_repo: Arc<dyn MedicationRepository> = Arc::new(FakeMedicationRepository::new());
    let mut container = Container::new();
    container.mark_dose_taken_service =
        Arc::new(MarkDoseTakenService::new(fake_dose_repo, fake_med_repo))
            as Arc<dyn MarkDoseTakenPort>;
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
    handler.handle(&mut app, Key::Char('s'));

    assert!(app.status_message.is_some());
    assert!(
        app.status_message
            .as_ref()
            .unwrap()
            .contains("Open medication details")
    );
}
