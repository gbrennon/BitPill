use crate::application::{
    dtos::{
        requests::{
            CreateMedicationRequest, DeleteMedicationRequest, EditMedicationRequest,
            GetMedicationRequest, GetSettingsRequest, ListAllMedicationsRequest,
            ListDoseRecordsRequest, MarkDoseTakenRequest, SaveSettingsRequest,
            UpdateMedicationRequest,
        },
        responses::{
            CreateMedicationResponse, DeleteMedicationResponse, EditMedicationResponse,
            GetMedicationResponse, GetSettingsResponse, ListAllMedicationsResponse,
            ListDoseRecordsResponse, MarkDoseTakenResponse, MedicationDto, SaveSettingsResponse,
            UpdateMedicationResponse,
        },
    },
    errors::ApplicationError,
    ports::inbound::{
        create_medication_port::CreateMedicationPort, delete_medication_port::DeleteMedicationPort,
        edit_medication_port::EditMedicationPort, get_medication_port::GetMedicationPort,
        get_settings_port::GetSettingsPort, list_all_medications_port::ListAllMedicationsPort,
        list_dose_records_port::ListDoseRecordsPort, mark_dose_taken_port::MarkDoseTakenPort,
        save_settings_port::SaveSettingsPort, update_medication_port::UpdateMedicationPort,
    },
};

pub struct FakeListAllMedicationsPort;
impl ListAllMedicationsPort for FakeListAllMedicationsPort {
    fn execute(
        &self,
        _request: ListAllMedicationsRequest,
    ) -> Result<ListAllMedicationsResponse, ApplicationError> {
        Ok(ListAllMedicationsResponse {
            medications: vec![],
        })
    }
}

pub struct FakeCreateMedicationPort;
impl CreateMedicationPort for FakeCreateMedicationPort {
    fn execute(
        &self,
        _request: CreateMedicationRequest,
    ) -> Result<CreateMedicationResponse, ApplicationError> {
        Ok(CreateMedicationResponse {
            id: "fake-id".into(),
        })
    }
}

pub struct FakeEditMedicationPort;
impl EditMedicationPort for FakeEditMedicationPort {
    fn execute(
        &self,
        _request: EditMedicationRequest,
    ) -> Result<EditMedicationResponse, ApplicationError> {
        Ok(EditMedicationResponse {
            id: "fake-id".into(),
        })
    }
}

pub struct FakeUpdateMedicationPort;
impl UpdateMedicationPort for FakeUpdateMedicationPort {
    fn execute(
        &self,
        _request: UpdateMedicationRequest,
    ) -> Result<UpdateMedicationResponse, ApplicationError> {
        Ok(UpdateMedicationResponse {
            id: "fake-id".into(),
        })
    }
}

pub struct FakeDeleteMedicationPort;
impl DeleteMedicationPort for FakeDeleteMedicationPort {
    fn execute(
        &self,
        _request: DeleteMedicationRequest,
    ) -> Result<DeleteMedicationResponse, ApplicationError> {
        Ok(DeleteMedicationResponse {})
    }
}

pub struct FakeGetMedicationPort;
impl GetMedicationPort for FakeGetMedicationPort {
    fn execute(
        &self,
        _request: GetMedicationRequest,
    ) -> Result<GetMedicationResponse, ApplicationError> {
        Err(ApplicationError::NotFound(
            crate::application::errors::NotFoundError,
        ))
    }
}

pub struct FakeGetMedicationPortOk {
    pub medication: MedicationDto,
}
impl GetMedicationPort for FakeGetMedicationPortOk {
    fn execute(
        &self,
        _request: GetMedicationRequest,
    ) -> Result<GetMedicationResponse, ApplicationError> {
        Ok(GetMedicationResponse {
            medication: MedicationDto {
                id: self.medication.id.clone(),
                name: self.medication.name.clone(),
                amount_mg: self.medication.amount_mg,
                scheduled_time: self.medication.scheduled_time.clone(),
                dose_frequency: self.medication.dose_frequency.clone(),
                taken_today: 0,
                scheduled_today: 0,
            },
        })
    }
}

pub struct FakeListDoseRecordsPort;
impl ListDoseRecordsPort for FakeListDoseRecordsPort {
    fn execute(
        &self,
        _request: ListDoseRecordsRequest,
    ) -> Result<ListDoseRecordsResponse, ApplicationError> {
        Ok(ListDoseRecordsResponse { records: vec![] })
    }
}

pub struct FakeMarkDoseTakenPort;
impl MarkDoseTakenPort for FakeMarkDoseTakenPort {
    fn execute(
        &self,
        _request: MarkDoseTakenRequest,
    ) -> Result<MarkDoseTakenResponse, ApplicationError> {
        Ok(MarkDoseTakenResponse::new("fake-id"))
    }
}

pub struct FakeGetSettingsPort;
impl GetSettingsPort for FakeGetSettingsPort {
    fn execute(
        &self,
        _request: GetSettingsRequest,
    ) -> Result<GetSettingsResponse, ApplicationError> {
        Ok(GetSettingsResponse {
            navigation_mode: "vi".to_string(),
        })
    }
}

pub struct FakeGetSettingsPortWithMode {
    pub mode: String,
}

impl FakeGetSettingsPortWithMode {
    pub fn new(mode: &str) -> Self {
        Self {
            mode: mode.to_string(),
        }
    }
}

impl GetSettingsPort for FakeGetSettingsPortWithMode {
    fn execute(
        &self,
        _request: GetSettingsRequest,
    ) -> Result<GetSettingsResponse, ApplicationError> {
        Ok(GetSettingsResponse {
            navigation_mode: self.mode.clone(),
        })
    }
}

pub struct FakeSaveSettingsPort;
impl SaveSettingsPort for FakeSaveSettingsPort {
    fn execute(
        &self,
        _request: SaveSettingsRequest,
    ) -> Result<SaveSettingsResponse, ApplicationError> {
        Ok(SaveSettingsResponse {
            navigation_mode: "vi".to_string(),
        })
    }
}
