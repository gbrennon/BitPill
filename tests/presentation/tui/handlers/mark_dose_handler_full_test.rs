use std::sync::Arc;

use bitpill::{
    application::{
        dtos::{
            requests::{
                CreateMedicationRequest, DeleteMedicationRequest, GetSettingsRequest,
                ListAllMedicationsRequest, ListDoseRecordsRequest, MarkDoseTakenRequest,
                SaveSettingsRequest,
            },
            responses::{
                CreateMedicationResponse, DeleteMedicationResponse, DoseRecordDto,
                GetSettingsResponse, ListAllMedicationsResponse, ListDoseRecordsResponse,
                MarkDoseTakenResponse, MedicationDto, SaveSettingsResponse,
            },
        },
        errors::ApplicationError,
        ports::inbound::{
            create_medication_port::CreateMedicationPort,
            delete_medication_port::DeleteMedicationPort, edit_medication_port::EditMedicationPort,
            get_medication_port::GetMedicationPort, get_settings_port::GetSettingsPort,
            list_all_medications_port::ListAllMedicationsPort,
            list_dose_records_port::ListDoseRecordsPort, mark_dose_taken_port::MarkDoseTakenPort,
            save_settings_port::SaveSettingsPort, update_medication_port::UpdateMedicationPort,
        },
    },
    infrastructure::container::Container,
    presentation::tui::{
        app::App,
        app_services::AppServices,
        handlers::{mark_dose_handler::MarkDoseHandler, port::Handler},
        input::Key,
        screen::Screen,
    },
};
use tempfile::tempdir;

struct FakeListAll;
impl ListAllMedicationsPort for FakeListAll {
    fn execute(
        &self,
        _: ListAllMedicationsRequest,
    ) -> Result<ListAllMedicationsResponse, ApplicationError> {
        Ok(ListAllMedicationsResponse {
            medications: vec![],
        })
    }
}

struct FakeSettings(String);
impl GetSettingsPort for FakeSettings {
    fn execute(&self, _: GetSettingsRequest) -> Result<GetSettingsResponse, ApplicationError> {
        Ok(GetSettingsResponse {
            navigation_mode: self.0.clone(),
        })
    }
}

struct FakeMarkDose;
impl MarkDoseTakenPort for FakeMarkDose {
    fn execute(&self, _: MarkDoseTakenRequest) -> Result<MarkDoseTakenResponse, ApplicationError> {
        Ok(MarkDoseTakenResponse::new("fake-id"))
    }
}

struct FakeCreate;
impl CreateMedicationPort for FakeCreate {
    fn execute(
        &self,
        _: CreateMedicationRequest,
    ) -> Result<CreateMedicationResponse, ApplicationError> {
        Ok(CreateMedicationResponse { id: "fake".into() })
    }
}

struct FakeSave;
impl SaveSettingsPort for FakeSave {
    fn execute(&self, _: SaveSettingsRequest) -> Result<SaveSettingsResponse, ApplicationError> {
        Ok(SaveSettingsResponse {
            navigation_mode: "vi".into(),
        })
    }
}

struct FakeDelete;
impl DeleteMedicationPort for FakeDelete {
    fn execute(
        &self,
        _: DeleteMedicationRequest,
    ) -> Result<DeleteMedicationResponse, ApplicationError> {
        Ok(DeleteMedicationResponse {})
    }
}

struct FakeDoseRecords(Vec<DoseRecordDto>);
impl ListDoseRecordsPort for FakeDoseRecords {
    fn execute(
        &self,
        _: ListDoseRecordsRequest,
    ) -> Result<ListDoseRecordsResponse, ApplicationError> {
        Ok(ListDoseRecordsResponse {
            records: self.0.clone(),
        })
    }
}

struct Stub;
impl EditMedicationPort for Stub {
    fn execute(
        &self,
        _: bitpill::application::dtos::requests::EditMedicationRequest,
    ) -> Result<bitpill::application::dtos::responses::EditMedicationResponse, ApplicationError>
    {
        Ok(bitpill::application::dtos::responses::EditMedicationResponse { id: "x".into() })
    }
}
impl GetMedicationPort for Stub {
    fn execute(
        &self,
        _: bitpill::application::dtos::requests::GetMedicationRequest,
    ) -> Result<bitpill::application::dtos::responses::GetMedicationResponse, ApplicationError>
    {
        Err(ApplicationError::NotFound(
            bitpill::application::errors::NotFoundError,
        ))
    }
}
impl UpdateMedicationPort for Stub {
    fn execute(
        &self,
        _: bitpill::application::dtos::requests::UpdateMedicationRequest,
    ) -> Result<bitpill::application::dtos::responses::UpdateMedicationResponse, ApplicationError>
    {
        Ok(bitpill::application::dtos::responses::UpdateMedicationResponse { id: "x".into() })
    }
}

fn svc(recs: Vec<DoseRecordDto>, nav_mode: &str) -> AppServices {
    AppServices {
        list_all_medications: Arc::new(FakeListAll),
        create_medication: Arc::new(FakeCreate),
        edit_medication: Arc::new(Stub),
        update_medication: Arc::new(Stub),
        delete_medication: Arc::new(FakeDelete),
        get_medication: Arc::new(Stub),
        list_dose_records: Arc::new(FakeDoseRecords(recs)),
        mark_dose_taken: Arc::new(FakeMarkDose),
        get_settings: Arc::new(FakeSettings(nav_mode.to_string())),
        save_settings: Arc::new(FakeSave),
    }
}

fn make_dose_record(id: &str, scheduled: i64) -> DoseRecordDto {
    DoseRecordDto {
        id: id.to_string(),
        medication_id: "m1".to_string(),
        scheduled_at: scheduled,
        taken_at: None,
    }
}

#[test]
fn mark_dose_handler_default_impl() {
    let _handler = MarkDoseHandler::default();
}

#[test]
fn mark_dose_handler_vim_j_moves_down() {
    let recs = vec![make_dose_record("r1", 1000), make_dose_record("r2", 2000)];
    let mut app = App {
        services: svc(recs, "vi"),
        current_screen: Screen::MarkDose {
            medication_id: "m1".to_string(),
            records: vec![],
            selected_index: 0,
        },
        medications: vec![],
        selected_index: 0,
        status_message: None,
        status_expires_at: None,
        should_quit: false,
        show_welcome_modal: false,
    };
    let mut handler = MarkDoseHandler::default();
    handler.handle(&mut app, Key::Char('j'));
    if let Screen::MarkDose { selected_index, .. } = &app.current_screen {
        assert_eq!(*selected_index, 1);
    }
}

#[test]
fn mark_dose_handler_vim_k_moves_up() {
    let recs = vec![make_dose_record("r1", 1000), make_dose_record("r2", 2000)];
    let mut app = App {
        services: svc(recs, "vi"),
        current_screen: Screen::MarkDose {
            medication_id: "m1".to_string(),
            records: vec![],
            selected_index: 1,
        },
        medications: vec![],
        selected_index: 0,
        status_message: None,
        status_expires_at: None,
        should_quit: false,
        show_welcome_modal: false,
    };
    let mut handler = MarkDoseHandler::default();
    handler.handle(&mut app, Key::Char('k'));
    if let Screen::MarkDose { selected_index, .. } = &app.current_screen {
        assert_eq!(*selected_index, 0);
    }
}

#[test]
fn mark_dose_handler_space_marks_taken() {
    let recs = vec![make_dose_record("r1", 1000)];
    let mut app = App {
        services: svc(recs, "vi"),
        current_screen: Screen::MarkDose {
            medication_id: "m1".to_string(),
            records: vec![make_dose_record("r1", 1000)],
            selected_index: 0,
        },
        medications: vec![],
        selected_index: 0,
        status_message: None,
        status_expires_at: None,
        should_quit: false,
        show_welcome_modal: false,
    };
    let mut handler = MarkDoseHandler::default();
    handler.handle(&mut app, Key::Char(' '));
    assert!(app.status_message.is_some());
}

#[test]
fn mark_dose_handler_enter_marks_taken() {
    let recs = vec![make_dose_record("r1", 1000)];
    let mut app = App {
        services: svc(recs, "vi"),
        current_screen: Screen::MarkDose {
            medication_id: "m1".to_string(),
            records: vec![make_dose_record("r1", 1000)],
            selected_index: 0,
        },
        medications: vec![],
        selected_index: 0,
        status_message: None,
        status_expires_at: None,
        should_quit: false,
        show_welcome_modal: false,
    };
    let mut handler = MarkDoseHandler::default();
    handler.handle(&mut app, Key::Enter);
    assert!(app.status_message.is_some());
}

#[test]
fn mark_dose_handler_esc_returns_home() {
    let recs = vec![make_dose_record("r1", 1000)];
    let mut app = App {
        services: svc(recs, "vi"),
        current_screen: Screen::MarkDose {
            medication_id: "m1".to_string(),
            records: vec![],
            selected_index: 0,
        },
        medications: vec![],
        selected_index: 0,
        status_message: None,
        status_expires_at: None,
        should_quit: false,
        show_welcome_modal: false,
    };
    let mut handler = MarkDoseHandler::default();
    handler.handle(&mut app, Key::Esc);
    assert!(matches!(app.current_screen, Screen::HomeScreen));
}

#[test]
fn mark_dose_handler_emacs_n_moves_down() {
    let recs = vec![make_dose_record("r1", 1000), make_dose_record("r2", 2000)];
    let mut app = App {
        services: svc(recs, "emacs"),
        current_screen: Screen::MarkDose {
            medication_id: "m1".to_string(),
            records: vec![],
            selected_index: 0,
        },
        medications: vec![],
        selected_index: 0,
        status_message: None,
        status_expires_at: None,
        should_quit: false,
        show_welcome_modal: false,
    };
    let mut handler = MarkDoseHandler::default();
    handler.handle(&mut app, Key::Char('n'));
    if let Screen::MarkDose { selected_index, .. } = &app.current_screen {
        assert_eq!(*selected_index, 1);
    }
}

#[test]
fn mark_dose_handler_emacs_p_moves_up() {
    let recs = vec![make_dose_record("r1", 1000), make_dose_record("r2", 2000)];
    let mut app = App {
        services: svc(recs, "emacs"),
        current_screen: Screen::MarkDose {
            medication_id: "m1".to_string(),
            records: vec![],
            selected_index: 1,
        },
        medications: vec![],
        selected_index: 0,
        status_message: None,
        status_expires_at: None,
        should_quit: false,
        show_welcome_modal: false,
    };
    let mut handler = MarkDoseHandler::default();
    handler.handle(&mut app, Key::Char('p'));
    if let Screen::MarkDose { selected_index, .. } = &app.current_screen {
        assert_eq!(*selected_index, 0);
    }
}

#[test]
fn mark_dose_handler_e2e_with_container() {
    let dir = tempdir().unwrap();
    std::fs::write(dir.path().join("medications.json"), r#"[{"id":"m1","name":"Test","amount_mg":100,"scheduled_time":[],"dose_frequency":"OnceDaily","taken_today":0,"scheduled_today":0}]"#).unwrap();
    std::fs::write(
        dir.path().join("doses.json"),
        r#"[{"id":"r1","medication_id":"m1","scheduled_at":1000,"taken_at":null}]"#,
    )
    .unwrap();
    std::fs::write(dir.path().join("settings.json"), r#"{"vim_enabled":true}"#).unwrap();

    let container = Container::new(
        dir.path().join("medications.json"),
        dir.path().join("doses.json"),
        dir.path().join("settings.json"),
    );
    let mut app = App::new(AppServices::from_container(&container));
    let mut handler = MarkDoseHandler::default();
    handler.handle(&mut app, Key::Esc);
    assert!(matches!(app.current_screen, Screen::HomeScreen));
}
