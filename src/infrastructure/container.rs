use std::sync::Arc;

use crate::application::ports::settings_repository_port::SettingsRepositoryPortBox;
use crate::application::services::{
    create_medication_service::CreateMedicationService,
    get_medication_service::GetMedicationService,
    list_all_medications_service::ListAllMedicationsService,
    mark_dose_taken_service::MarkDoseTakenService, mark_medication_taken_service::MarkMedicationTakenService,
    schedule_dose_service::ScheduleDoseService,
    update_medication_service::UpdateMedicationService,
    edit_medication_service::EditMedicationService,
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
/// and wires them into application services.
pub struct Container {
    pub create_medication_service: CreateMedicationService,
    pub list_all_medications_service: ListAllMedicationsService,
    pub list_dose_records_service: crate::application::services::list_dose_records_service::ListDoseRecordsService,
    pub create_dose_record_service: crate::application::services::create_dose_record_service::CreateDoseRecordsService,
    pub mark_dose_taken_service: MarkDoseTakenService,
    pub mark_medication_taken_service: crate::application::services::mark_medication_taken_service::MarkMedicationTakenService,
    pub schedule_dose_service: ScheduleDoseService,
    pub get_medication_service: GetMedicationService,
    pub update_medication_service: UpdateMedicationService,
    pub edit_medication_service: EditMedicationService,
    pub delete_medication_service: crate::application::services::delete_medication_service::DeleteMedicationService,
    pub settings_repository: std::sync::Arc<
        crate::application::ports::settings_repository_port::SettingsRepositoryPortBox,
    >,
    pub settings_service: crate::application::services::settings_service::SettingsService,
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

        let settings_service = crate::application::services::settings_service::SettingsService::new(
            settings_repo.clone(),
        );

        Self {
            create_medication_service: CreateMedicationService::new(medication_repo.clone()),
            list_all_medications_service: ListAllMedicationsService::new(medication_repo.clone()),
            list_dose_records_service: crate::application::services::list_dose_records_service::ListDoseRecordsService::new(dose_record_repo.clone()),
            create_dose_record_service: crate::application::services::create_dose_record_service::CreateDoseRecordsService::new(dose_record_repo.clone()),
            mark_dose_taken_service: MarkDoseTakenService::new(dose_record_repo.clone()),
            mark_medication_taken_service: MarkMedicationTakenService::new(dose_record_repo.clone()),
            schedule_dose_service: ScheduleDoseService::new(
                medication_repo.clone(),
                dose_record_repo,
                notification,
                clock,
            ),
            get_medication_service: GetMedicationService::new(medication_repo.clone()),
            update_medication_service: UpdateMedicationService::new(medication_repo.clone()),
            edit_medication_service: EditMedicationService::new(medication_repo.clone()),
            delete_medication_service: crate::application::services::delete_medication_service::DeleteMedicationService::new(medication_repo),
            settings_repository: settings_repo,
            settings_service,
        }
    }

    pub fn get_settings_service(
        &self,
    ) -> &crate::application::services::settings_service::SettingsService {
        &self.settings_service
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
        let settings_service =
            crate::application::services::settings_service::SettingsService::new(
                settings_repo.clone(),
            );
        Self {
            create_medication_service: CreateMedicationService::new(medication_repo.clone()),
            list_all_medications_service: ListAllMedicationsService::new(medication_repo.clone()),
            list_dose_records_service: crate::application::services::list_dose_records_service::ListDoseRecordsService::new(dose_record_repo.clone()),
            create_dose_record_service: crate::application::services::create_dose_record_service::CreateDoseRecordsService::new(dose_record_repo.clone()),
            mark_dose_taken_service: MarkDoseTakenService::new(dose_record_repo.clone()),
            mark_medication_taken_service: MarkMedicationTakenService::new(dose_record_repo.clone()),
            schedule_dose_service: ScheduleDoseService::new(
                medication_repo.clone(),
                dose_record_repo,
                notification,
                clock,
            ),
            get_medication_service: GetMedicationService::new(medication_repo.clone()),
            update_medication_service: UpdateMedicationService::new(medication_repo.clone()),
            edit_medication_service: EditMedicationService::new(medication_repo.clone()),
            delete_medication_service: crate::application::services::delete_medication_service::DeleteMedicationService::new(medication_repo),
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
