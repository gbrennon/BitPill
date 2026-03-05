use crate::application::errors::ApplicationError;
use crate::application::ports::inbound::create_medication_port::{
    CreateMedicationPort, CreateMedicationRequest, CreateMedicationResponse,
};
use crate::application::ports::inbound::delete_medication_port::{
    DeleteMedicationPort, DeleteMedicationRequest, DeleteMedicationResponse,
};
use crate::application::ports::inbound::edit_medication_port::{
    EditMedicationPort, EditMedicationRequest, EditMedicationResponse,
};
use crate::application::ports::inbound::get_medication_port::{
    GetMedicationPort, GetMedicationRequest, GetMedicationResponse,
    MedicationDto as GetMedicationDto,
};
use crate::application::ports::inbound::list_all_medications_port::{
    ListAllMedicationsPort, ListAllMedicationsRequest, ListAllMedicationsResponse,
};
use crate::application::ports::inbound::list_dose_records_port::{
    ListDoseRecordsPort, ListDoseRecordsRequest, ListDoseRecordsResponse,
};
use crate::application::ports::inbound::mark_dose_taken_port::{
    MarkDoseTakenPort, MarkDoseTakenRequest, MarkDoseTakenResponse,
};
use crate::application::ports::inbound::mark_medication_taken_port::{
    MarkMedicationTakenPort, MarkMedicationTakenRequest, MarkMedicationTakenResponse,
};
use crate::application::ports::inbound::settings_port::{
    SettingsPort, SettingsRequest, SettingsResponse,
};

pub struct FakeListAllMedicationsPort;
impl ListAllMedicationsPort for FakeListAllMedicationsPort {
    fn execute(
        &self,
        _request: ListAllMedicationsRequest,
    ) -> Result<ListAllMedicationsResponse, ApplicationError> {
        Ok(ListAllMedicationsResponse { medications: vec![] })
    }
}

pub struct FakeCreateMedicationPort;
impl CreateMedicationPort for FakeCreateMedicationPort {
    fn execute(
        &self,
        _request: CreateMedicationRequest,
    ) -> Result<CreateMedicationResponse, ApplicationError> {
        Ok(CreateMedicationResponse { id: "fake-id".into() })
    }
}

pub struct FakeEditMedicationPort;
impl EditMedicationPort for FakeEditMedicationPort {
    fn execute(
        &self,
        _request: EditMedicationRequest,
    ) -> Result<EditMedicationResponse, ApplicationError> {
        Ok(EditMedicationResponse { id: "fake-id".into() })
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
    pub medication: GetMedicationDto,
}
impl GetMedicationPort for FakeGetMedicationPortOk {
    fn execute(
        &self,
        _request: GetMedicationRequest,
    ) -> Result<GetMedicationResponse, ApplicationError> {
        Ok(GetMedicationResponse {
            medication: GetMedicationDto {
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

pub struct FakeMarkMedicationTakenPort;
impl MarkMedicationTakenPort for FakeMarkMedicationTakenPort {
    fn execute(
        &self,
        _request: MarkMedicationTakenRequest,
    ) -> Result<MarkMedicationTakenResponse, ApplicationError> {
        Ok(MarkMedicationTakenResponse::new("fake-id"))
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
