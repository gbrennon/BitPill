use std::sync::Arc;

use crate::{
    application::services::{
        create_medication_service::CreateMedicationService,
        mark_dose_taken_service::MarkDoseTakenService,
    },
    infrastructure::persistence::{
        in_memory_dose_record_repository::InMemoryDoseRecordRepository,
        in_memory_medication_repository::InMemoryMedicationRepository,
    },
};

pub struct Container {
    pub create_medication_service: CreateMedicationService,
    pub mark_dose_taken_service: MarkDoseTakenService,
}

impl Container {
    pub fn new() -> Self {
        let medication_repo = Arc::new(InMemoryMedicationRepository::new());
        let dose_record_repo = Arc::new(InMemoryDoseRecordRepository::new());
        Self {
            create_medication_service: CreateMedicationService::new(medication_repo),
            mark_dose_taken_service: MarkDoseTakenService::new(dose_record_repo),
        }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}
