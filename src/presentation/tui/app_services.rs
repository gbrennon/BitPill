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

#[cfg(any(test, feature = "test-helpers"))]
impl AppServices {
    /// Constructs an `AppServices` backed entirely by in-memory fakes.
    /// No file I/O or concrete infrastructure is used.
    pub fn fake() -> Self {
        use crate::application::ports::fakes::{
            FakeCreateMedicationPort, FakeDeleteMedicationPort, FakeEditMedicationPort,
            FakeGetMedicationPort, FakeGetSettingsPort, FakeListAllMedicationsPort,
            FakeListDoseRecordsPort, FakeMarkDoseTakenPort, FakeSaveSettingsPort,
            FakeUpdateMedicationPort,
        };
        Self {
            list_all_medications: Arc::new(FakeListAllMedicationsPort),
            create_medication: Arc::new(FakeCreateMedicationPort),
            edit_medication: Arc::new(FakeEditMedicationPort),
            update_medication: Arc::new(FakeUpdateMedicationPort),
            delete_medication: Arc::new(FakeDeleteMedicationPort),
            get_medication: Arc::new(FakeGetMedicationPort),
            list_dose_records: Arc::new(FakeListDoseRecordsPort),
            mark_dose_taken: Arc::new(FakeMarkDoseTakenPort),
            get_settings: Arc::new(FakeGetSettingsPort),
            save_settings: Arc::new(FakeSaveSettingsPort),
        }
    }

    /// Constructs an `AppServices` with a specific navigation mode.
    pub fn fake_with_mode(mode: &'static str) -> Self {
        use crate::application::ports::fakes::{
            FakeCreateMedicationPort, FakeDeleteMedicationPort, FakeEditMedicationPort,
            FakeGetMedicationPort, FakeGetSettingsPortWithMode, FakeListAllMedicationsPort,
            FakeListDoseRecordsPort, FakeMarkDoseTakenPort, FakeSaveSettingsPort,
            FakeUpdateMedicationPort,
        };
        Self {
            list_all_medications: Arc::new(FakeListAllMedicationsPort),
            create_medication: Arc::new(FakeCreateMedicationPort),
            edit_medication: Arc::new(FakeEditMedicationPort),
            update_medication: Arc::new(FakeUpdateMedicationPort),
            delete_medication: Arc::new(FakeDeleteMedicationPort),
            get_medication: Arc::new(FakeGetMedicationPort),
            list_dose_records: Arc::new(FakeListDoseRecordsPort),
            mark_dose_taken: Arc::new(FakeMarkDoseTakenPort),
            get_settings: Arc::new(FakeGetSettingsPortWithMode::new(mode)),
            save_settings: Arc::new(FakeSaveSettingsPort),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fake_constructs_all_services() {
        let services = AppServices::fake();
        let _ = &services.list_all_medications;
        let _ = &services.create_medication;
        let _ = &services.edit_medication;
        let _ = &services.update_medication;
        let _ = &services.delete_medication;
        let _ = &services.get_medication;
        let _ = &services.list_dose_records;
        let _ = &services.mark_dose_taken;
        let _ = &services.get_settings;
        let _ = &services.save_settings;
    }
}
