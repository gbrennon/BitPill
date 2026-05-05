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
    presentation::tui::{
        app::App,
        app_services::AppServices,
        handlers::{medication_list_handler::MedicationListHandler, port::Handler},
        input::Key,
        screen::Screen,
    },
};

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

fn svc() -> AppServices {
    AppServices {
        list_all_medications: Arc::new(FakeListAll),
        create_medication: Arc::new(FakeCreate),
        edit_medication: Arc::new(Stub),
        update_medication: Arc::new(Stub),
        delete_medication: Arc::new(FakeDelete),
        get_medication: Arc::new(Stub),
        list_dose_records: Arc::new(FakeDoseRecords(vec![])),
        mark_dose_taken: Arc::new(FakeMarkDose),
        get_settings: Arc::new(FakeSettings("vi".to_string())),
        save_settings: Arc::new(FakeSave),
    }
}

fn make_med(id: &str, name: &str) -> MedicationDto {
    MedicationDto {
        id: id.to_string(),
        name: name.to_string(),
        amount_mg: 100,
        scheduled_time: vec![],
        dose_frequency: "OnceDaily".to_string(),
        taken_today: 0,
        scheduled_today: 0,
    }
}

#[test]
fn list_handler_j_moves_down() {
    let meds = vec![make_med("m1", "Aspirin"), make_med("m2", "Ibuprofen")];
    let mut app = App {
        services: svc(),
        current_screen: Screen::HomeScreen,
        medications: meds,
        selected_index: 0,
        status_message: None,
        status_expires_at: None,
        should_quit: false,
        show_welcome_modal: false,
    };
    let mut handler = MedicationListHandler::default();
    handler.handle(&mut app, Key::Char('j'));
    assert_eq!(app.selected_index, 1);
}

#[test]
fn list_handler_k_moves_up() {
    let meds = vec![make_med("m1", "Aspirin"), make_med("m2", "Ibuprofen")];
    let mut app = App {
        services: svc(),
        current_screen: Screen::HomeScreen,
        medications: meds,
        selected_index: 1,
        status_message: None,
        status_expires_at: None,
        should_quit: false,
        show_welcome_modal: false,
    };
    let mut handler = MedicationListHandler::default();
    handler.handle(&mut app, Key::Char('k'));
    assert_eq!(app.selected_index, 0);
}

#[test]
fn list_handler_enter_opens_details() {
    let meds = vec![make_med("m1", "Aspirin")];
    let mut app = App {
        services: svc(),
        current_screen: Screen::HomeScreen,
        medications: meds,
        selected_index: 0,
        status_message: None,
        status_expires_at: None,
        should_quit: false,
        show_welcome_modal: false,
    };
    let mut handler = MedicationListHandler::default();
    handler.handle(&mut app, Key::Enter);
    assert!(matches!(
        app.current_screen,
        Screen::MedicationDetails { .. }
    ));
}

#[test]
fn list_handler_space_opens_details() {
    let meds = vec![make_med("m1", "Aspirin")];
    let mut app = App {
        services: svc(),
        current_screen: Screen::HomeScreen,
        medications: meds,
        selected_index: 0,
        status_message: None,
        status_expires_at: None,
        should_quit: false,
        show_welcome_modal: false,
    };
    let mut handler = MedicationListHandler::default();
    handler.handle(&mut app, Key::Char(' '));
    assert!(matches!(
        app.current_screen,
        Screen::MedicationDetails { .. }
    ));
}

#[test]
fn list_handler_c_opens_create() {
    let meds = vec![];
    let mut app = App {
        services: svc(),
        current_screen: Screen::HomeScreen,
        medications: meds,
        selected_index: 0,
        status_message: None,
        status_expires_at: None,
        should_quit: false,
        show_welcome_modal: false,
    };
    let mut handler = MedicationListHandler::default();
    handler.handle(&mut app, Key::Char('c'));
    assert!(matches!(
        app.current_screen,
        Screen::CreateMedication { .. }
    ));
}

#[test]
fn list_handler_d_opens_confirm_delete() {
    let meds = vec![make_med("m1", "Aspirin")];
    let mut app = App {
        services: svc(),
        current_screen: Screen::HomeScreen,
        medications: meds,
        selected_index: 0,
        status_message: None,
        status_expires_at: None,
        should_quit: false,
        show_welcome_modal: false,
    };
    let mut handler = MedicationListHandler::default();
    handler.handle(&mut app, Key::Char('d'));
    handler.handle(&mut app, Key::Char('d'));
    assert!(matches!(app.current_screen, Screen::ConfirmDelete { .. }));
}

#[test]
fn list_handler_r_reloads() {
    let meds = vec![make_med("m1", "Aspirin")];
    let mut app = App {
        services: svc(),
        current_screen: Screen::HomeScreen,
        medications: meds,
        selected_index: 0,
        status_message: None,
        status_expires_at: None,
        should_quit: false,
        show_welcome_modal: false,
    };
    let mut handler = MedicationListHandler::default();
    handler.handle(&mut app, Key::Char('r'));
}

#[test]
fn list_handler_g_goes_to_first() {
    let meds = vec![make_med("m1", "Aspirin"), make_med("m2", "Ibuprofen")];
    let mut app = App {
        services: svc(),
        current_screen: Screen::HomeScreen,
        medications: meds,
        selected_index: 1,
        status_message: None,
        status_expires_at: None,
        should_quit: false,
        show_welcome_modal: false,
    };
    let mut handler = MedicationListHandler::default();
    handler.handle(&mut app, Key::Char('g'));
    assert_eq!(app.selected_index, 0);
}

#[test]
fn list_handler_gg_twice_stays() {
    let meds = vec![make_med("m1", "Aspirin"), make_med("m2", "Ibuprofen")];
    let mut app = App {
        services: svc(),
        current_screen: Screen::HomeScreen,
        medications: meds,
        selected_index: 0,
        status_message: None,
        status_expires_at: None,
        should_quit: false,
        show_welcome_modal: false,
    };
    let mut handler = MedicationListHandler::default();
    handler.handle(&mut app, Key::Char('g'));
    handler.handle(&mut app, Key::Char('g'));
}

#[test]
fn list_handler_down_arrow_moves() {
    let meds = vec![make_med("m1", "Aspirin"), make_med("m2", "Ibuprofen")];
    let mut app = App {
        services: svc(),
        current_screen: Screen::HomeScreen,
        medications: meds,
        selected_index: 0,
        status_message: None,
        status_expires_at: None,
        should_quit: false,
        show_welcome_modal: false,
    };
    let mut handler = MedicationListHandler::default();
    handler.handle(&mut app, Key::Down);
    assert_eq!(app.selected_index, 1);
}

#[test]
fn list_handler_up_arrow_moves() {
    let meds = vec![make_med("m1", "Aspirin"), make_med("m2", "Ibuprofen")];
    let mut app = App {
        services: svc(),
        current_screen: Screen::HomeScreen,
        medications: meds,
        selected_index: 1,
        status_message: None,
        status_expires_at: None,
        should_quit: false,
        show_welcome_modal: false,
    };
    let mut handler = MedicationListHandler::default();
    handler.handle(&mut app, Key::Up);
    assert_eq!(app.selected_index, 0);
}
