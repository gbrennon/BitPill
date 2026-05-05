use std::sync::Arc;

use bitpill::{
    application::{
        dtos::{
            requests::{
                CreateMedicationRequest, DeleteMedicationRequest, GetMedicationRequest,
                GetSettingsRequest, ListAllMedicationsRequest, ListDoseRecordsRequest,
                MarkDoseTakenRequest, SaveSettingsRequest,
            },
            responses::{
                CreateMedicationResponse, DeleteMedicationResponse, GetMedicationResponse,
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
    presentation::tui::{app::App, app_services::AppServices, screen::Screen},
};

// ---- Minimal fake ports for presentation tests ----

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

struct FakeCreate;
impl CreateMedicationPort for FakeCreate {
    fn execute(
        &self,
        _: CreateMedicationRequest,
    ) -> Result<CreateMedicationResponse, ApplicationError> {
        Ok(CreateMedicationResponse { id: "fake".into() })
    }
}

struct FakeGetOk;
impl GetMedicationPort for FakeGetOk {
    fn execute(&self, _: GetMedicationRequest) -> Result<GetMedicationResponse, ApplicationError> {
        Ok(GetMedicationResponse {
            medication: MedicationDto {
                id: "m1".into(),
                name: "Test".into(),
                amount_mg: 100,
                scheduled_time: vec![(8, 0)],
                dose_frequency: "OnceDaily".into(),
                taken_today: 0,
                scheduled_today: 1,
            },
        })
    }
}

struct FakeDoses;
impl ListDoseRecordsPort for FakeDoses {
    fn execute(
        &self,
        _: ListDoseRecordsRequest,
    ) -> Result<ListDoseRecordsResponse, ApplicationError> {
        Ok(ListDoseRecordsResponse { records: vec![] })
    }
}

struct FakeMarkDose;
impl MarkDoseTakenPort for FakeMarkDose {
    fn execute(&self, _: MarkDoseTakenRequest) -> Result<MarkDoseTakenResponse, ApplicationError> {
        Ok(MarkDoseTakenResponse::new("fake"))
    }
}

struct FakeSettings;
impl GetSettingsPort for FakeSettings {
    fn execute(&self, _: GetSettingsRequest) -> Result<GetSettingsResponse, ApplicationError> {
        Ok(GetSettingsResponse {
            navigation_mode: "vi".into(),
        })
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
impl UpdateMedicationPort for Stub {
    fn execute(
        &self,
        _: bitpill::application::dtos::requests::UpdateMedicationRequest,
    ) -> Result<bitpill::application::dtos::responses::UpdateMedicationResponse, ApplicationError>
    {
        Ok(bitpill::application::dtos::responses::UpdateMedicationResponse { id: "x".into() })
    }
}

pub fn fake_services() -> AppServices {
    AppServices {
        list_all_medications: Arc::new(FakeListAll),
        create_medication: Arc::new(FakeCreate),
        edit_medication: Arc::new(Stub),
        update_medication: Arc::new(Stub),
        delete_medication: Arc::new(FakeDelete),
        get_medication: Arc::new(FakeGetOk),
        list_dose_records: Arc::new(FakeDoses),
        mark_dose_taken: Arc::new(FakeMarkDose),
        get_settings: Arc::new(FakeSettings),
        save_settings: Arc::new(FakeSave),
    }
}

pub fn make_app(screen: Screen) -> App {
    App {
        services: fake_services(),
        current_screen: screen,
        medications: vec![],
        selected_index: 0,
        status_message: None,
        status_expires_at: None,
        should_quit: false,
        show_welcome_modal: false,
    }
}
