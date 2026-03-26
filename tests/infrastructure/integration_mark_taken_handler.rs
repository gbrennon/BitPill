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
        app::App, app_services::AppServices,
        handlers::medication_list_handler::MedicationListHandler, handlers::port::Handler,
        input::Key,
    },
};
use std::sync::Arc;
use tempfile::tempdir;

#[test]
fn medication_list_handler_saves_taken_dose_record_on_s() {
    let dir = tempdir().unwrap();
    let container = Container::new(
        dir.path().join("meds.json"),
        dir.path().join("doses.json"),
        dir.path().join("settings.json"),
    );
    let fake_dose_repo: Arc<dyn DoseRecordRepository> = Arc::new(FakeDoseRecordRepository::new());
    let fake_med_repo: Arc<dyn MedicationRepository> = Arc::new(FakeMedicationRepository::new());
    let services = AppServices {
        list_all_medications: container.list_all_medications_service.clone(),
        create_medication: container.create_medication_service.clone(),
        edit_medication: container.edit_medication_service.clone(),
        delete_medication: container.delete_medication_service.clone(),
        get_medication: container.get_medication_service.clone(),
        list_dose_records: container.list_dose_records_service.clone(),
        mark_dose_taken: Arc::new(MarkDoseTakenService::new(fake_dose_repo, fake_med_repo))
            as Arc<dyn MarkDoseTakenPort>,
        settings: container.settings_service.clone(),
    };

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
    handler.handle(&mut app, Key::from(Key::Char('s')));

    assert!(app.status_message.is_some());
}
