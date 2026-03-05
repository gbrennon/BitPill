use std::sync::Arc;

use crate::application::ports::inbound::create_medication_port::CreateMedicationPort;
use crate::application::ports::inbound::delete_medication_port::DeleteMedicationPort;
use crate::application::ports::inbound::edit_medication_port::EditMedicationPort;
use crate::application::ports::inbound::get_medication_port::GetMedicationPort;
use crate::application::ports::inbound::list_all_medications_port::ListAllMedicationsPort;
use crate::application::ports::inbound::list_dose_records_port::ListDoseRecordsPort;
use crate::application::ports::inbound::mark_dose_taken_port::MarkDoseTakenPort;
use crate::application::ports::inbound::mark_medication_taken_port::MarkMedicationTakenPort;
use crate::application::ports::inbound::settings_port::SettingsPort;
use crate::application::ports::inbound::update_medication_port::UpdateMedicationPort;
use crate::application::ports::settings_repository_port::SettingsRepositoryPortBox;
use crate::application::services::{
    create_medication_service::CreateMedicationService,
    edit_medication_service::EditMedicationService, get_medication_service::GetMedicationService,
    list_all_medications_service::ListAllMedicationsService,
    mark_dose_taken_service::MarkDoseTakenService,
    mark_medication_taken_service::MarkMedicationTakenService,
    schedule_dose_service::ScheduleDoseService, update_medication_service::UpdateMedicationService,
};
use crate::infrastructure::{
    clock::system_clock::SystemClock,
    notifications::console_notification_adapter::ConsoleNotificationAdapter,
    persistence::{
        json_dose_record_repository::JsonDoseRecordRepository,
        json_medication_repository::JsonMedicationRepository,
        json_settings_repository::JsonSettingsRepository,
    },
};

/// Composition root — the only place that instantiates concrete adapters
/// and wires them into application services as `Arc<dyn Port>` trait objects.
pub struct Container {
    pub create_medication_service: Arc<dyn CreateMedicationPort>,
    pub list_all_medications_service: Arc<dyn ListAllMedicationsPort>,
    pub list_dose_records_service: Arc<dyn ListDoseRecordsPort>,
    pub create_dose_record_service:
        crate::application::services::create_dose_record_service::CreateDoseRecordsService,
    pub mark_dose_taken_service: Arc<dyn MarkDoseTakenPort>,
    pub mark_medication_taken_service: Arc<dyn MarkMedicationTakenPort>,
    /// Uses concrete type: REST handler calls a non-port `execute()` method.
    pub schedule_dose_service: ScheduleDoseService,
    pub get_medication_service: Arc<dyn GetMedicationPort>,
    pub update_medication_service: Arc<dyn UpdateMedicationPort>,
    pub edit_medication_service: Arc<dyn EditMedicationPort>,
    pub delete_medication_service: Arc<dyn DeleteMedicationPort>,
    pub settings_repository: Arc<SettingsRepositoryPortBox>,
    pub settings_service: Arc<dyn SettingsPort>,
}

impl Container {
    pub fn new() -> Self {
        let medication_repo = Arc::new(JsonMedicationRepository::with_default_path());
        let dose_record_repo = Arc::new(JsonDoseRecordRepository::with_default_path());
        let notification = Arc::new(ConsoleNotificationAdapter);
        let clock = Arc::new(SystemClock);
        let settings_repo: Arc<SettingsRepositoryPortBox> = Arc::new(JsonSettingsRepository::new(
            std::env::var("BITPILL_SETTINGS_FILE")
                .map(std::path::PathBuf::from)
                .unwrap_or_else(|_| std::path::PathBuf::from("settings.json")),
        ));

        let settings_service: Arc<dyn SettingsPort> = Arc::new(
            crate::application::services::settings_service::SettingsService::new(
                settings_repo.clone(),
            ),
        );

        Self {
            create_medication_service: Arc::new(CreateMedicationService::new(
                medication_repo.clone(),
            )),
            list_all_medications_service: Arc::new(ListAllMedicationsService::new(
                medication_repo.clone(),
            )),
            list_dose_records_service: Arc::new(
                crate::application::services::list_dose_records_service::ListDoseRecordsService::new(dose_record_repo.clone()),
            ),
            create_dose_record_service: crate::application::services::create_dose_record_service::CreateDoseRecordsService::new(dose_record_repo.clone()),
            mark_dose_taken_service: Arc::new(MarkDoseTakenService::new(dose_record_repo.clone())),
            mark_medication_taken_service: Arc::new(MarkMedicationTakenService::new(
                dose_record_repo.clone(),
            )),
            schedule_dose_service: ScheduleDoseService::new(
                medication_repo.clone(),
                dose_record_repo,
                notification,
                clock,
            ),
            get_medication_service: Arc::new(GetMedicationService::new(medication_repo.clone())),
            update_medication_service: Arc::new(UpdateMedicationService::new(
                medication_repo.clone(),
            )),
            edit_medication_service: Arc::new(EditMedicationService::new(medication_repo.clone())),
            delete_medication_service: Arc::new(
                crate::application::services::delete_medication_service::DeleteMedicationService::new(medication_repo),
            ),
            settings_repository: settings_repo,
            settings_service,
        }
    }

    /// Constructs a container pointing all persistence to the given paths.
    /// Intended for integration tests — avoids touching any real data files.
    #[cfg(any(test, feature = "test-helpers"))]
    pub fn new_with_paths(
        medications_path: std::path::PathBuf,
        dose_records_path: std::path::PathBuf,
        settings_path: std::path::PathBuf,
    ) -> Self {
        let medication_repo = Arc::new(JsonMedicationRepository::new(medications_path));
        let dose_record_repo = Arc::new(JsonDoseRecordRepository::new(dose_records_path));
        let notification = Arc::new(ConsoleNotificationAdapter);
        let clock = Arc::new(SystemClock);
        let settings_repo: Arc<SettingsRepositoryPortBox> =
            Arc::new(JsonSettingsRepository::new(settings_path));
        let settings_service: Arc<dyn SettingsPort> = Arc::new(
            crate::application::services::settings_service::SettingsService::new(
                settings_repo.clone(),
            ),
        );
        Self {
            create_medication_service: Arc::new(CreateMedicationService::new(
                medication_repo.clone(),
            )),
            list_all_medications_service: Arc::new(ListAllMedicationsService::new(
                medication_repo.clone(),
            )),
            list_dose_records_service: Arc::new(
                crate::application::services::list_dose_records_service::ListDoseRecordsService::new(dose_record_repo.clone()),
            ),
            create_dose_record_service: crate::application::services::create_dose_record_service::CreateDoseRecordsService::new(dose_record_repo.clone()),
            mark_dose_taken_service: Arc::new(MarkDoseTakenService::new(dose_record_repo.clone())),
            mark_medication_taken_service: Arc::new(MarkMedicationTakenService::new(
                dose_record_repo.clone(),
            )),
            schedule_dose_service: ScheduleDoseService::new(
                medication_repo.clone(),
                dose_record_repo,
                notification,
                clock,
            ),
            get_medication_service: Arc::new(GetMedicationService::new(medication_repo.clone())),
            update_medication_service: Arc::new(UpdateMedicationService::new(
                medication_repo.clone(),
            )),
            edit_medication_service: Arc::new(EditMedicationService::new(medication_repo.clone())),
            delete_medication_service: Arc::new(
                crate::application::services::delete_medication_service::DeleteMedicationService::new(medication_repo),
            ),
            settings_repository: settings_repo,
            settings_service,
        }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn new_with_paths_builds_successfully_in_test_cfg() {
        let dir = tempdir().unwrap();
        let _container = Container::new_with_paths(
            dir.path().join("medications.json"),
            dir.path().join("doses.json"),
            dir.path().join("settings.json"),
        );
    }
}

