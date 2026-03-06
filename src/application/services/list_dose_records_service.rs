use std::sync::Arc;

use crate::application::dtos::requests::ListDoseRecordsRequest;
use crate::application::dtos::responses::{DoseRecordDto, ListDoseRecordsResponse};
use crate::application::errors::ApplicationError;
use crate::application::ports::dose_record_repository_port::DoseRecordRepository;
use crate::application::ports::inbound::list_dose_records_port::ListDoseRecordsPort;

pub struct ListDoseRecordsService {
    repository: Arc<dyn DoseRecordRepository>,
}

impl ListDoseRecordsService {
    pub fn new(repository: Arc<dyn DoseRecordRepository>) -> Self {
        Self { repository }
    }
}

impl ListDoseRecordsPort for ListDoseRecordsService {
    fn execute(
        &self,
        request: ListDoseRecordsRequest,
    ) -> Result<ListDoseRecordsResponse, ApplicationError> {
        let medication_id = crate::domain::value_objects::medication_id::MedicationId::from(
            uuid::Uuid::parse_str(&request.medication_id).map_err(|_| {
                ApplicationError::InvalidInput(format!(
                    "invalid medication id: {}",
                    request.medication_id
                ))
            })?,
        );
        let records = self.repository.find_all_by_medication(&medication_id)?;
        let dtos = records
            .into_iter()
            .map(|r| DoseRecordDto {
                id: r.id().to_string(),
                medication_id: r.medication_id().to_string(),
                scheduled_at: r.scheduled_at(),
                taken_at: r.taken_at(),
            })
            .collect();
        Ok(ListDoseRecordsResponse { records: dtos })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::fakes::FakeDoseRecordRepository;
    use crate::domain::entities::dose_record::DoseRecord;
    use crate::domain::value_objects::medication_id::MedicationId;
    use chrono::NaiveDate;

    fn make_service(repo: std::sync::Arc<FakeDoseRecordRepository>) -> ListDoseRecordsService {
        ListDoseRecordsService::new(repo)
    }

    fn make_datetime(h: u32, m: u32) -> chrono::NaiveDateTime {
        NaiveDate::from_ymd_opt(2025, 1, 1)
            .unwrap()
            .and_hms_opt(h, m, 0)
            .unwrap()
    }

    #[test]
    fn execute_with_invalid_medication_id_returns_invalid_input() {
        let repo = std::sync::Arc::new(FakeDoseRecordRepository::new());
        let service = make_service(repo);
        let req = super::ListDoseRecordsRequest { medication_id: "not-a-uuid".into() };

        let res = service.execute(req);
        assert!(matches!(res, Err(ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn execute_with_records_returns_dtos() {
        let med_id = MedicationId::generate();
        let record = DoseRecord::new(med_id.clone(), make_datetime(8,0));
        let repo = std::sync::Arc::new(FakeDoseRecordRepository::with(record.clone()));
        let service = make_service(repo);
        let req = super::ListDoseRecordsRequest { medication_id: med_id.to_string() };

        let res = service.execute(req).unwrap();
        assert_eq!(res.records.len(), 1);
        assert_eq!(res.records[0].id, record.id().to_string());
    }
}
