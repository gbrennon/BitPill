use crate::application::dtos::requests::{
    CreateMedicationRequest, DeleteMedicationRequest, EditMedicationRequest, GetMedicationRequest,
    ListAllMedicationsRequest, ListDoseRecordsRequest, MarkDoseTakenRequest, SettingsRequest,
};
use crate::application::dtos::responses::{
    CreateMedicationResponse, DeleteMedicationResponse, EditMedicationResponse,
    GetMedicationResponse, ListAllMedicationsResponse, ListDoseRecordsResponse,
    MarkDoseTakenResponse, MedicationDto, SettingsResponse,
};
use crate::application::errors::ApplicationError;
use crate::application::ports::inbound::create_medication_port::CreateMedicationPort;
use crate::application::ports::inbound::delete_medication_port::DeleteMedicationPort;
use crate::application::ports::inbound::edit_medication_port::EditMedicationPort;
use crate::application::ports::inbound::get_medication_port::GetMedicationPort;
use crate::application::ports::inbound::list_all_medications_port::ListAllMedicationsPort;
use crate::application::ports::inbound::list_dose_records_port::ListDoseRecordsPort;
use crate::application::ports::inbound::mark_dose_taken_port::MarkDoseTakenPort;
use crate::application::ports::inbound::settings_port::SettingsPort;

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

pub struct FakeDeleteMedicationPort;
impl DeleteMedicationPort for FakeDeleteMedicationPort {
    fn execute(
        &self,
        _request: DeleteMedicationRequest,
    ) -> Result<DeleteMedicationResponse, ApplicationError> {
        Ok(DeleteMedicationResponse {})
    }
}

/// Returns a `NotFound` error so handlers can handle the "not found" branch.
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

/// Variant that always returns a successful response with a single medication.
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

pub struct FakeSettingsPort;
impl SettingsPort for FakeSettingsPort {
    fn execute(&self, _req: SettingsRequest) -> Result<SettingsResponse, ApplicationError> {
        Ok(SettingsResponse {
            settings: serde_json::json!({}),
        })
    }
}
