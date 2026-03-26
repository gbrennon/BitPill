use std::path::PathBuf;
use std::sync::Arc;

use crate::{
    application::{
        ports::inbound::{
            create_dose_record_port::CreateDoseRecordPort,
            create_medication_port::CreateMedicationPort,
            delete_medication_port::DeleteMedicationPort, edit_medication_port::EditMedicationPort,
            get_medication_port::GetMedicationPort,
            list_all_medications_port::ListAllMedicationsPort,
            list_dose_records_port::ListDoseRecordsPort, mark_dose_taken_port::MarkDoseTakenPort,
            schedule_dose_port::ScheduleDosePort, settings_port::SettingsPort,
            update_medication_port::UpdateMedicationPort,
        },
        services::{
            create_dose_record_service::CreateDoseRecordService,
            create_medication_service::CreateMedicationService,
            delete_medication_service::DeleteMedicationService,
            edit_medication_service::EditMedicationService,
            get_medication_service::GetMedicationService,
            list_all_medications_service::ListAllMedicationsService,
            list_dose_records_service::ListDoseRecordsService,
            mark_dose_taken_service::MarkDoseTakenService,
            schedule_dose_service::ScheduleDoseService, settings_service::SettingsService,
            update_medication_service::UpdateMedicationService,
        },
    },
    infrastructure::{
        clock::system_clock::SystemClock,
        config::app_initializer::AppInitializer,
        notifications::console_notification_adapter::ConsoleNotificationAdapter,
        persistence::{
            json_dose_record_repository::JsonDoseRecordRepository,
            json_medication_repository::JsonMedicationRepository,
            json_settings_repository::JsonSettingsRepository,
        },
    },
};

/// Composition root — the only place that instantiates concrete adapters
/// and wires them into application services as `Arc<dyn Port>` trait objects.
pub struct Container {
    pub create_medication_service: Arc<dyn CreateMedicationPort>,
    pub list_all_medications_service: Arc<dyn ListAllMedicationsPort>,
    pub list_dose_records_service: Arc<dyn ListDoseRecordsPort>,
    pub create_dose_record_service: Arc<dyn CreateDoseRecordPort>,
    pub mark_dose_taken_service: Arc<dyn MarkDoseTakenPort>,
    /// Uses concrete type: REST handler calls a non-port `execute()` method.
    pub schedule_dose_service: Arc<dyn ScheduleDosePort>,
    pub get_medication_service: Arc<dyn GetMedicationPort>,
    pub update_medication_service: Arc<dyn UpdateMedicationPort>,
    pub edit_medication_service: Arc<dyn EditMedicationPort>,
    pub delete_medication_service: Arc<dyn DeleteMedicationPort>,
    pub settings_service: Arc<dyn SettingsPort>,
}

impl Container {
    /// Constructs a container with explicit paths for all persistence.
    ///
    /// # Arguments
    /// * `medications_path` - Path to medications JSON file
    /// * `dose_records_path` - Path to dose records JSON file  
    /// * `settings_path` - Path to settings JSON file
    pub fn new(
        medications_path: PathBuf,
        dose_records_path: PathBuf,
        settings_path: PathBuf,
    ) -> Self {
        let medication_repo = Arc::new(JsonMedicationRepository::new(medications_path));
        let dose_record_repo = Arc::new(JsonDoseRecordRepository::new(dose_records_path));
        let notification = Arc::new(ConsoleNotificationAdapter);
        let clock = Arc::new(SystemClock);
        let settings_repo = Arc::new(JsonSettingsRepository::new(settings_path));

        if let Err(e) = AppInitializer::initialize_from_paths(
            medication_repo.path(),
            dose_record_repo.path(),
            settings_repo.path(),
        ) {
            eprintln!("Warning: could not initialize config directory: {e}");
        }

        Self {
            create_medication_service: Arc::new(CreateMedicationService::new(
                medication_repo.clone(),
            )),
            list_all_medications_service: Arc::new(ListAllMedicationsService::new(
                medication_repo.clone(),
                dose_record_repo.clone(),
            )),
            list_dose_records_service: Arc::new(ListDoseRecordsService::new(
                dose_record_repo.clone(),
            )),
            create_dose_record_service: Arc::new(CreateDoseRecordService::new(
                dose_record_repo.clone(),
            )),
            mark_dose_taken_service: Arc::new(MarkDoseTakenService::new(
                dose_record_repo.clone(),
                medication_repo.clone(),
            )),
            schedule_dose_service: Arc::new(ScheduleDoseService::new(
                medication_repo.clone(),
                dose_record_repo.clone(),
                notification.clone(),
                clock.clone(),
            )),
            get_medication_service: Arc::new(GetMedicationService::new(
                medication_repo.clone(),
                dose_record_repo.clone(),
            )),
            update_medication_service: Arc::new(UpdateMedicationService::new(
                medication_repo.clone(),
            )),
            edit_medication_service: Arc::new(EditMedicationService::new(medication_repo.clone())),
            delete_medication_service: Arc::new(DeleteMedicationService::new(
                medication_repo.clone(),
            )),
            settings_service: Arc::new(SettingsService::new(settings_repo.clone())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn new_builds_successfully() {
        let dir = tempdir().unwrap();
        let _container = Container::new(
            dir.path().join("medications.json"),
            dir.path().join("doses.json"),
            dir.path().join("settings.json"),
        );
    }
}
