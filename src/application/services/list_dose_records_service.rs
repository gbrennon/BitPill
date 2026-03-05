use std::sync::Arc;

use crate::application::errors::ApplicationError;
use crate::application::ports::dose_record_repository_port::DoseRecordRepository;
use crate::application::ports::inbound::list_dose_records_port::{
    DoseRecordDto, ListDoseRecordsPort, ListDoseRecordsRequest, ListDoseRecordsResponse,
};

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
    use crate::domain::entities::dose_record::DoseRecord as DomainDoseRecord;
    use crate::domain::value_objects::medication_id::MedicationId;
    use chrono::NaiveDate;

    #[test]
    fn list_dose_records_returns_records_for_medication() {
        let med_id = MedicationId::generate();
        let record = DomainDoseRecord::new(
            med_id.clone(),
            NaiveDate::from_ymd_opt(2025,1,1).unwrap().and_hms_opt(9,0,0).unwrap(),
        );
        let repo = std::sync::Arc::new(FakeDoseRecordRepository::with(record.clone()));
        let service = ListDoseRecordsService::new(repo);

        let req = ListDoseRecordsRequest { medication_id: med_id.to_string() };
        let res = service.execute(req).expect("should list records");
        assert_eq!(res.records.len(), 1);
        assert_eq!(res.records[0].id, record.id().to_string());
    }

    #[test]
    fn list_dose_records_invalid_medication_id_returns_invalid_input() {
        let repo = std::sync::Arc::new(FakeDoseRecordRepository::new());
        let service = ListDoseRecordsService::new(repo);
        let req = ListDoseRecordsRequest { medication_id: "not-a-uuid".into() };
        let res = service.execute(req);
        assert!(matches!(res, Err(crate::application::errors::ApplicationError::InvalidInput(_))));
    }
}
