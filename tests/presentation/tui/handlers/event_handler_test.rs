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
                MarkDoseTakenResponse, SaveSettingsResponse,
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
        handlers::{event_handler::EventHandler, port::Handler},
        input::Key,
        screen::Screen,
    },
};

// ---- Fake ports ----

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
        get_settings: Arc::new(FakeSettings("vi".into())),
        save_settings: Arc::new(FakeSave),
    }
}

fn app(screen: Screen) -> App {
    App {
        services: svc(),
        current_screen: screen,
        medications: vec![],
        selected_index: 0,
        status_message: None,
        status_expires_at: None,
        should_quit: false,
        show_welcome_modal: false,
    }
}

fn app_with_meds(
    screen: Screen,
    meds: Vec<bitpill::application::dtos::responses::MedicationDto>,
) -> App {
    let mut a = App {
        services: svc(),
        current_screen: screen,
        medications: meds,
        selected_index: 0,
        status_message: None,
        status_expires_at: None,
        should_quit: false,
        show_welcome_modal: false,
    };
    a
}

// ---- Tests ----

#[test]
fn home_screen_q_opens_confirm_quit() {
    let mut a = app(Screen::HomeScreen);
    EventHandler::default().handle(&mut a, Key::Char('q'));
    assert!(matches!(a.current_screen, Screen::ConfirmQuit { .. }));
}

#[test]
fn confirm_quit_y_sets_should_quit() {
    let prev = Box::new(Screen::HomeScreen);
    let mut a = app(Screen::ConfirmQuit { previous: prev });
    EventHandler::default().handle(&mut a, Key::Char('y'));
    assert!(a.should_quit);
}

#[test]
fn confirm_quit_n_returns_to_previous() {
    let prev = Box::new(Screen::HomeScreen);
    let mut a = app(Screen::ConfirmQuit { previous: prev });
    EventHandler::default().handle(&mut a, Key::Char('n'));
    assert!(matches!(a.current_screen, Screen::HomeScreen));
}

#[test]
fn confirm_quit_esc_returns_to_previous() {
    let prev = Box::new(Screen::Settings {
        vim_enabled: true,
        selected_index: 0,
    });
    let mut a = app(Screen::ConfirmQuit { previous: prev });
    EventHandler::default().handle(&mut a, Key::Esc);
    assert!(matches!(a.current_screen, Screen::Settings { .. }));
}

#[test]
fn confirm_delete_y_deletes_and_returns_home() {
    let mut a = app(Screen::ConfirmDelete {
        id: "m1".into(),
        name: "Test".into(),
    });
    EventHandler::default().handle(&mut a, Key::Char('y'));
    assert!(matches!(a.current_screen, Screen::HomeScreen));
}

#[test]
fn confirm_delete_n_returns_home() {
    let mut a = app(Screen::ConfirmDelete {
        id: "m1".into(),
        name: "Test".into(),
    });
    EventHandler::default().handle(&mut a, Key::Char('n'));
    assert!(matches!(a.current_screen, Screen::HomeScreen));
}

#[test]
fn confirm_cancel_y_returns_home() {
    let prev = Box::new(Screen::CreateMedication {
        name: "".into(),
        amount_mg: "".into(),
        selected_frequency: 0,
        scheduled_time: vec!["".into()],
        scheduled_idx: 0,
        focused_field: 0,
        insert_mode: false,
    });
    let mut a = app(Screen::ConfirmCancel { previous: prev });
    EventHandler::default().handle(&mut a, Key::Char('y'));
    assert!(matches!(a.current_screen, Screen::HomeScreen));
}

#[test]
fn confirm_cancel_n_returns_previous() {
    let prev = Box::new(Screen::CreateMedication {
        name: "x".into(),
        amount_mg: "100".into(),
        selected_frequency: 0,
        scheduled_time: vec!["08:00".into()],
        scheduled_idx: 0,
        focused_field: 0,
        insert_mode: false,
    });
    let mut a = app(Screen::ConfirmCancel { previous: prev });
    EventHandler::default().handle(&mut a, Key::Char('n'));
    assert!(matches!(a.current_screen, Screen::CreateMedication { .. }));
}

#[test]
fn validation_error_any_key_dismisses() {
    let prev = Box::new(Screen::HomeScreen);
    let mut a = app(Screen::ValidationError {
        messages: vec!["err".into()],
        previous: prev,
    });
    EventHandler::default().handle(&mut a, Key::Esc);
    assert!(matches!(a.current_screen, Screen::HomeScreen));
}

#[test]
fn settings_help_any_key_closes() {
    let prev = Box::new(Screen::HomeScreen);
    let mut a = app(Screen::SettingsHelp {
        vim_enabled: true,
        selected_index: 0,
        help_text: "help".into(),
        previous: prev,
    });
    EventHandler::default().handle(&mut a, Key::Esc);
    assert!(matches!(a.current_screen, Screen::HomeScreen));
}

#[test]
fn settings_question_mark_opens_help() {
    let mut a = app(Screen::Settings {
        vim_enabled: true,
        selected_index: 0,
    });
    EventHandler::default().handle(&mut a, Key::Char('?'));
    assert!(matches!(a.current_screen, Screen::SettingsHelp { .. }));
}

#[test]
fn settings_space_toggles_mode() {
    let mut a = app(Screen::Settings {
        vim_enabled: true,
        selected_index: 0,
    });
    EventHandler::default().handle(&mut a, Key::Char(' '));
    assert!(matches!(a.current_screen, Screen::Settings { .. }));
}

#[test]
fn settings_s_saves_and_returns_home() {
    let mut a = app(Screen::Settings {
        vim_enabled: true,
        selected_index: 0,
    });
    EventHandler::default().handle(&mut a, Key::Char('s'));
    assert!(matches!(a.current_screen, Screen::HomeScreen));
    assert!(a.status_message.is_some());
}

#[test]
fn settings_esc_returns_home() {
    let mut a = app(Screen::Settings {
        vim_enabled: true,
        selected_index: 0,
    });
    EventHandler::default().handle(&mut a, Key::Esc);
    assert!(matches!(a.current_screen, Screen::HomeScreen));
}

#[test]
fn home_question_mark_opens_settings_help() {
    let mut a = app(Screen::HomeScreen);
    EventHandler::default().handle(&mut a, Key::Char('?'));
    assert!(matches!(a.current_screen, Screen::SettingsHelp { .. }));
}

#[test]
fn medication_details_esc_returns_home() {
    let mut a = app(Screen::MedicationDetails { id: "m1".into() });
    EventHandler::default().handle(&mut a, Key::Esc);
    assert!(matches!(a.current_screen, Screen::HomeScreen));
}

#[test]
fn settings_j_in_vim_moves_down() {
    let mut a = app(Screen::Settings {
        vim_enabled: true,
        selected_index: 0,
    });
    EventHandler::default().handle(&mut a, Key::Char('j'));
    if let Screen::Settings { selected_index, .. } = &a.current_screen {
        assert_eq!(*selected_index, 1);
    }
}

#[test]
fn settings_k_in_vim_moves_up() {
    let mut a = app(Screen::Settings {
        vim_enabled: true,
        selected_index: 1,
    });
    EventHandler::default().handle(&mut a, Key::Char('k'));
    if let Screen::Settings { selected_index, .. } = &a.current_screen {
        assert_eq!(*selected_index, 0);
    }
}

#[test]
fn home_screen_question_mark_in_emacs_opens_help() {
    let mut a = app(Screen::HomeScreen);
    EventHandler::default().handle(&mut a, Key::Char('?'));
    assert!(matches!(a.current_screen, Screen::SettingsHelp { .. }));
}

#[test]
fn medication_details_e_opens_edit_screen() {
    let meds = vec![bitpill::application::dtos::responses::MedicationDto {
        id: "m1".to_string(),
        name: "Aspirin".to_string(),
        amount_mg: 100,
        scheduled_time: vec![(8, 0)],
        dose_frequency: "OnceDaily".to_string(),
        taken_today: 0,
        scheduled_today: 1,
    }];
    let mut a = app_with_meds(
        Screen::MedicationDetails {
            id: "m1".to_string(),
        },
        meds,
    );
    EventHandler::default().handle(&mut a, Key::Char('e'));
    assert!(matches!(a.current_screen, Screen::EditMedication { .. }));
}

#[test]
fn medication_details_m_opens_mark_dose_screen() {
    let meds = vec![bitpill::application::dtos::responses::MedicationDto {
        id: "m1".to_string(),
        name: "Aspirin".to_string(),
        amount_mg: 100,
        scheduled_time: vec![(8, 0)],
        dose_frequency: "OnceDaily".to_string(),
        taken_today: 0,
        scheduled_today: 1,
    }];
    let mut a = app_with_meds(
        Screen::MedicationDetails {
            id: "m1".to_string(),
        },
        meds,
    );
    EventHandler::default().handle(&mut a, Key::Char('m'));
    assert!(matches!(a.current_screen, Screen::MarkDose { .. }));
}

#[test]
fn settings_l_in_vim_moves_down() {
    let mut a = app(Screen::Settings {
        vim_enabled: true,
        selected_index: 0,
    });
    EventHandler::default().handle(&mut a, Key::Char('l'));
    if let Screen::Settings { selected_index, .. } = &a.current_screen {
        assert_eq!(*selected_index, 1);
    }
}

#[test]
fn settings_h_in_vim_moves_up() {
    let mut a = app(Screen::Settings {
        vim_enabled: true,
        selected_index: 1,
    });
    EventHandler::default().handle(&mut a, Key::Char('h'));
    if let Screen::Settings { selected_index, .. } = &a.current_screen {
        assert_eq!(*selected_index, 0);
    }
}

#[test]
fn settings_down_arrow_moves_down() {
    let mut a = app(Screen::Settings {
        vim_enabled: true,
        selected_index: 0,
    });
    EventHandler::default().handle(&mut a, Key::Down);
    if let Screen::Settings { selected_index, .. } = &a.current_screen {
        assert_eq!(*selected_index, 1);
    }
}

#[test]
fn settings_up_arrow_moves_up() {
    let mut a = app(Screen::Settings {
        vim_enabled: true,
        selected_index: 1,
    });
    EventHandler::default().handle(&mut a, Key::Up);
    if let Screen::Settings { selected_index, .. } = &a.current_screen {
        assert_eq!(*selected_index, 0);
    }
}

#[test]
fn settings_right_arrow_moves_down() {
    let mut a = app(Screen::Settings {
        vim_enabled: true,
        selected_index: 0,
    });
    EventHandler::default().handle(&mut a, Key::Right);
    if let Screen::Settings { selected_index, .. } = &a.current_screen {
        assert_eq!(*selected_index, 1);
    }
}

#[test]
fn settings_left_arrow_moves_up() {
    let mut a = app(Screen::Settings {
        vim_enabled: true,
        selected_index: 1,
    });
    EventHandler::default().handle(&mut a, Key::Left);
    if let Screen::Settings { selected_index, .. } = &a.current_screen {
        assert_eq!(*selected_index, 0);
    }
}

#[test]
fn settings_g_in_vim_goes_to_first() {
    let mut a = app(Screen::Settings {
        vim_enabled: true,
        selected_index: 1,
    });
    EventHandler::default().handle(&mut a, Key::Char('g'));
    if let Screen::Settings { selected_index, .. } = &a.current_screen {
        assert_eq!(*selected_index, 0);
    }
}

#[test]
fn settings_g_in_vim_twice_stays() {
    let mut a = app(Screen::Settings {
        vim_enabled: true,
        selected_index: 0,
    });
    EventHandler::default().handle(&mut a, Key::Char('g'));
    EventHandler::default().handle(&mut a, Key::Char('g'));
    if let Screen::Settings { selected_index, .. } = &a.current_screen {
        assert_eq!(*selected_index, 0);
    }
}
