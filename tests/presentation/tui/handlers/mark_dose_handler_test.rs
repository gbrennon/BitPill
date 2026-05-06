use std::sync::Arc;

use bitpill::{
    application::{
        dtos::{
            requests::{
                CreateMedicationRequest, GetSettingsRequest, ListAllMedicationsRequest,
                ListDoseRecordsRequest, MarkDoseTakenRequest, SaveSettingsRequest,
            },
            responses::{
                CreateMedicationResponse, DoseRecordDto, GetSettingsResponse,
                ListAllMedicationsResponse, ListDoseRecordsResponse, MarkDoseTakenResponse,
                SaveSettingsResponse,
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
        handlers::{mark_dose_handler::MarkDoseHandler, port::Handler},
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
impl DeleteMedicationPort for Stub {
    fn execute(
        &self,
        _: bitpill::application::dtos::requests::DeleteMedicationRequest,
    ) -> Result<bitpill::application::dtos::responses::DeleteMedicationResponse, ApplicationError>
    {
        Ok(bitpill::application::dtos::responses::DeleteMedicationResponse {})
    }
}
impl EditMedicationPort for Stub {
    fn execute(
        &self,
        _: bitpill::application::dtos::requests::EditMedicationRequest,
    ) -> Result<bitpill::application::dtos::responses::EditMedicationResponse, ApplicationError>
    {
        Err(ApplicationError::NotFound(
            bitpill::application::errors::NotFoundError,
        ))
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
        Err(ApplicationError::NotFound(
            bitpill::application::errors::NotFoundError,
        ))
    }
}

fn svc() -> AppServices {
    AppServices {
        list_all_medications: Arc::new(FakeListAll),
        create_medication: Arc::new(FakeCreate),
        edit_medication: Arc::new(Stub),
        update_medication: Arc::new(Stub),
        delete_medication: Arc::new(Stub),
        get_medication: Arc::new(Stub),
        list_dose_records: Arc::new(FakeDoseRecords(vec![])),
        mark_dose_taken: Arc::new(FakeMarkDose),
        get_settings: Arc::new(FakeSettings("vi".into())),
        save_settings: Arc::new(FakeSave),
    }
}

fn svc_with_doses(recs: Vec<DoseRecordDto>) -> AppServices {
    AppServices {
        list_dose_records: Arc::new(FakeDoseRecords(recs)),
        ..svc()
    }
}

fn app(med_id: &str, recs: Vec<DoseRecordDto>) -> App {
    let r = recs.clone();
    App {
        services: svc_with_doses(recs),
        current_screen: Screen::MarkDose {
            medication_id: med_id.to_string(),
            records: r,
            selected_index: 0,
        },
        medications: vec![],
        selected_index: 0,
        status_message: None,
        status_expires_at: None,
        should_quit: false,
        show_welcome_modal: false,
    }
}

fn rec(med_id: &str, h: u32, m: u32) -> DoseRecordDto {
    DoseRecordDto {
        id: format!("r-{}", med_id),
        medication_id: med_id.to_string(),
        scheduled_at: chrono::NaiveDate::from_ymd_opt(2025, 1, 1)
            .unwrap()
            .and_hms_opt(h, m, 0)
            .unwrap(),
        taken_at: None,
    }
}

fn slot(med_id: &str, i: usize, h: u32, m: u32) -> DoseRecordDto {
    DoseRecordDto {
        id: format!("slot:{}", i),
        medication_id: med_id.to_string(),
        scheduled_at: chrono::NaiveDate::from_ymd_opt(2025, 1, 1)
            .unwrap()
            .and_hms_opt(h, m, 0)
            .unwrap(),
        taken_at: None,
    }
}

#[test]
fn esc_returns_home() {
    let mut a = app("m1", vec![rec("m1", 8, 0)]);
    MarkDoseHandler.handle(&mut a, Key::Esc);
    assert!(matches!(a.current_screen, Screen::HomeScreen));
}

#[test]
fn j_moves_down() {
    let mut a = app("m1", vec![rec("m1", 8, 0), rec("m1", 20, 0)]);
    MarkDoseHandler.handle(&mut a, Key::Char('j'));
    if let Screen::MarkDose { selected_index, .. } = &a.current_screen {
        assert_eq!(*selected_index, 1);
    }
}

#[test]
fn k_moves_up() {
    let r = vec![rec("m1", 8, 0), rec("m1", 20, 0)];
    let mut a = app("m1", r.clone());
    a.current_screen = Screen::MarkDose {
        medication_id: "m1".into(),
        records: r,
        selected_index: 1,
    };
    MarkDoseHandler.handle(&mut a, Key::Char('k'));
    if let Screen::MarkDose { selected_index, .. } = &a.current_screen {
        assert_eq!(*selected_index, 0);
    }
}

#[test]
fn enter_marks_and_updates() {
    let mut a = app("m1", vec![rec("m1", 8, 0)]);
    MarkDoseHandler.handle(&mut a, Key::Enter);
    assert!(a.status_message.is_some());
}

#[test]
fn enter_on_slot_navigates_details() {
    let mut a = app("m1", vec![slot("m1", 0, 8, 0)]);
    MarkDoseHandler.handle(&mut a, Key::Enter);
    assert!(matches!(a.current_screen, Screen::MedicationDetails { .. }));
}

#[test]
fn enter_empty_shows_status() {
    let mut a = app("m1", vec![]);
    MarkDoseHandler.handle(&mut a, Key::Enter);
    assert!(a.status_message.is_some());
    assert!(matches!(a.current_screen, Screen::HomeScreen));
}
