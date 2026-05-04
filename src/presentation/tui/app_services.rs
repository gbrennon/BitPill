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
