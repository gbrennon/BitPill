use std::sync::Arc;

use crate::{
    application::ports::inbound::{
        create_medication_port::CreateMedicationPort, delete_medication_port::DeleteMedicationPort,
        edit_medication_port::EditMedicationPort, get_medication_port::GetMedicationPort,
        get_settings_port::GetSettingsPort, list_all_medications_port::ListAllMedicationsPort,
        list_dose_records_port::ListDoseRecordsPort, mark_dose_taken_port::MarkDoseTakenPort,
        save_settings_port::SaveSettingsPort, update_medication_port::UpdateMedicationPort,
    },
    infrastructure::container::Container,
};

/// Holds `Arc<dyn Port>` for every inbound application port the TUI needs.
/// The TUI depends only on these abstractions — never on concrete infrastructure.
pub struct AppServices {
    pub list_all_medications: Arc<dyn ListAllMedicationsPort>,
    pub create_medication: Arc<dyn CreateMedicationPort>,
    pub edit_medication: Arc<dyn EditMedicationPort>,
    pub update_medication: Arc<dyn UpdateMedicationPort>,
    pub delete_medication: Arc<dyn DeleteMedicationPort>,
    pub get_medication: Arc<dyn GetMedicationPort>,
    pub list_dose_records: Arc<dyn ListDoseRecordsPort>,
    pub mark_dose_taken: Arc<dyn MarkDoseTakenPort>,
    pub get_settings: Arc<dyn GetSettingsPort>,
    pub save_settings: Arc<dyn SaveSettingsPort>,
}

// Minimal mock implementations for Default
struct MockListAllMedicationsPort;
impl ListAllMedicationsPort for MockListAllMedicationsPort {
    fn execute(
        &self,
        _: crate::application::dtos::requests::ListAllMedicationsRequest,
    ) -> Result<
        crate::application::dtos::responses::ListAllMedicationsResponse,
        crate::application::errors::ApplicationError,
    > {
        Ok(
            crate::application::dtos::responses::ListAllMedicationsResponse {
                medications: vec![],
            },
        )
    }
}
struct MockCreateMedicationPort;
impl CreateMedicationPort for MockCreateMedicationPort {
    fn execute(
        &self,
        _: crate::application::dtos::requests::CreateMedicationRequest,
    ) -> Result<
        crate::application::dtos::responses::CreateMedicationResponse,
        crate::application::errors::ApplicationError,
    > {
        Ok(crate::application::dtos::responses::CreateMedicationResponse { id: "".into() })
    }
}

struct MockEditMedicationPort;
impl EditMedicationPort for MockEditMedicationPort {
    fn execute(
        &self,
        _: crate::application::dtos::requests::EditMedicationRequest,
    ) -> Result<
        crate::application::dtos::responses::EditMedicationResponse,
        crate::application::errors::ApplicationError,
    > {
        Ok(crate::application::dtos::responses::EditMedicationResponse { id: "".into() })
    }
}

struct MockUpdateMedicationPort;
impl UpdateMedicationPort for MockUpdateMedicationPort {
    fn execute(
        &self,
        _: crate::application::dtos::requests::UpdateMedicationRequest,
    ) -> Result<
        crate::application::dtos::responses::UpdateMedicationResponse,
        crate::application::errors::ApplicationError,
    > {
        Ok(crate::application::dtos::responses::UpdateMedicationResponse { id: "".into() })
    }
}

struct MockDeleteMedicationPort;
impl DeleteMedicationPort for MockDeleteMedicationPort {
    fn execute(
        &self,
        _: crate::application::dtos::requests::DeleteMedicationRequest,
    ) -> Result<
        crate::application::dtos::responses::DeleteMedicationResponse,
        crate::application::errors::ApplicationError,
    > {
        Ok(crate::application::dtos::responses::DeleteMedicationResponse {})
    }
}

struct MockGetMedicationPort;
impl GetMedicationPort for MockGetMedicationPort {
    fn execute(
        &self,
        _: crate::application::dtos::requests::GetMedicationRequest,
    ) -> Result<
        crate::application::dtos::responses::GetMedicationResponse,
        crate::application::errors::ApplicationError,
    > {
        Ok(crate::application::dtos::responses::GetMedicationResponse {
            medication: crate::application::dtos::responses::MedicationDto {
                id: "".into(),
                name: "".into(),
                amount_mg: 0,
                scheduled_time: vec![],
                dose_frequency: "".into(),
                taken_today: 0,
                scheduled_today: 0,
            },
        })
    }
}

struct MockListDoseRecordsPort;
impl ListDoseRecordsPort for MockListDoseRecordsPort {
    fn execute(
        &self,
        _: crate::application::dtos::requests::ListDoseRecordsRequest,
    ) -> Result<
        crate::application::dtos::responses::ListDoseRecordsResponse,
        crate::application::errors::ApplicationError,
    > {
        Ok(crate::application::dtos::responses::ListDoseRecordsResponse { records: vec![] })
    }
}

struct MockMarkDoseTakenPort;
impl MarkDoseTakenPort for MockMarkDoseTakenPort {
    fn execute(
        &self,
        _: crate::application::dtos::requests::MarkDoseTakenRequest,
    ) -> Result<
        crate::application::dtos::responses::MarkDoseTakenResponse,
        crate::application::errors::ApplicationError,
    > {
        Ok(crate::application::dtos::responses::MarkDoseTakenResponse::new(""))
    }
}

struct MockGetSettingsPort;
impl GetSettingsPort for MockGetSettingsPort {
    fn execute(
        &self,
        _: crate::application::dtos::requests::GetSettingsRequest,
    ) -> Result<
        crate::application::dtos::responses::GetSettingsResponse,
        crate::application::errors::ApplicationError,
    > {
        Ok(crate::application::dtos::responses::GetSettingsResponse {
            navigation_mode: "".into(),
        })
    }
}

struct MockSaveSettingsPort;
impl SaveSettingsPort for MockSaveSettingsPort {
    fn execute(
        &self,
        _: crate::application::dtos::requests::SaveSettingsRequest,
    ) -> Result<
        crate::application::dtos::responses::SaveSettingsResponse,
        crate::application::errors::ApplicationError,
    > {
        Ok(crate::application::dtos::responses::SaveSettingsResponse {
            navigation_mode: "".into(),
        })
    }
}

impl Default for AppServices {
    fn default() -> Self {
        Self {
            list_all_medications: Arc::new(MockListAllMedicationsPort),
            create_medication: Arc::new(MockCreateMedicationPort),
            edit_medication: Arc::new(MockEditMedicationPort),
            update_medication: Arc::new(MockUpdateMedicationPort),
            delete_medication: Arc::new(MockDeleteMedicationPort),
            get_medication: Arc::new(MockGetMedicationPort),
            list_dose_records: Arc::new(MockListDoseRecordsPort),
            mark_dose_taken: Arc::new(MockMarkDoseTakenPort),
            get_settings: Arc::new(MockGetSettingsPort),
            save_settings: Arc::new(MockSaveSettingsPort),
        }
    }
}

// END OF STRUCT AND DEFAULT. DO NOT REPEAT FIELDS BELOW THIS LINE.

impl AppServices {
    /// Constructs `AppServices` by cloning the port `Arc`s from a `Container`.
    /// Used at the composition root (`App::run`, `PresentationRoot`) and in integration tests.
    pub fn from_container(container: &Container) -> Self {
        Self {
            list_all_medications: container.list_all_medications_service.clone(),
            create_medication: container.create_medication_service.clone(),
            edit_medication: container.edit_medication_service.clone(),
            update_medication: container.update_medication_service.clone(),
            delete_medication: container.delete_medication_service.clone(),
            get_medication: container.get_medication_service.clone(),
            list_dose_records: container.list_dose_records_service.clone(),
            mark_dose_taken: container.mark_dose_taken_service.clone(),
            get_settings: container.settings_service.clone(),
            save_settings: container.save_settings_service.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn default_and_from_container() {
        let default = AppServices::default();
        // ensure calling a method on a port works
        let _ = default
            .list_all_medications
            .execute(crate::application::dtos::requests::ListAllMedicationsRequest {});

        let dir = tempdir().unwrap();
        let container = crate::infrastructure::container::Container::new(
            dir.path().join("medications.json"),
            dir.path().join("doses.json"),
            dir.path().join("settings.json"),
        );
        let _svc = AppServices::from_container(&container);
    }
}
