use std::sync::Arc;

use crate::application::services::{
    create_medication_service::CreateMedicationService,
    list_all_medications_service::ListAllMedicationsService,
    mark_dose_taken_service::MarkDoseTakenService,
    schedule_dose_service::ScheduleDoseService,
};
use crate::infrastructure::{
    clock::system_clock::SystemClock,
    notifications::console_notification_adapter::ConsoleNotificationAdapter,
    persistence::{
        json_dose_record_repository::JsonDoseRecordRepository,
        json_medication_repository::JsonMedicationRepository,
    },
};

/// Composition root — the only place that instantiates concrete adapters
/// and wires them into application services.
pub struct Container {
    pub create_medication_service: CreateMedicationService,
    pub list_all_medications_service: ListAllMedicationsService,
    pub mark_dose_taken_service: MarkDoseTakenService,
    pub schedule_dose_service: ScheduleDoseService,
}

impl Container {
    pub fn new() -> Self {
        let medication_repo = Arc::new(JsonMedicationRepository::new());
        let dose_record_repo = Arc::new(JsonDoseRecordRepository::new());
        let notification = Arc::new(ConsoleNotificationAdapter);
        let clock = Arc::new(SystemClock);

        Self {
            create_medication_service: CreateMedicationService::new(medication_repo.clone()),
            list_all_medications_service: ListAllMedicationsService::new(medication_repo.clone()),
            mark_dose_taken_service: MarkDoseTakenService::new(dose_record_repo.clone()),
            schedule_dose_service: ScheduleDoseService::new(
                medication_repo,
                dose_record_repo,
                notification,
                clock,
            ),
        }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}
