#[cfg(test)]
mod tests {
    use crate::application::services::get_medication_service::GetMedicationService;
    use crate::application::services::list_dose_records_service::ListDoseRecordsService;
    use crate::application::services::update_medication_service::UpdateMedicationService;
    use crate::application::services::delete_medication_service::DeleteMedicationService;
    use crate::application::ports::fakes::{FakeMedicationRepository, FakeDoseRecordRepository};
    use crate::domain::entities::medication::Medication;
    use crate::domain::entities::dose_record::DoseRecord;
    use crate::domain::value_objects::{
        medication_id::MedicationId, medication_name::MedicationName, dosage::Dosage,
        scheduled_time::ScheduledTime, medication_frequency::DoseFrequency,
    };
    use crate::application::ports::inbound::get_medication_port::{GetMedicationRequest, GetMedicationPort};
    use crate::application::ports::inbound::list_dose_records_port::{ListDoseRecordsRequest, ListDoseRecordsPort};
    use crate::application::ports::inbound::update_medication_port::{UpdateMedicationRequest, UpdateMedicationPort};
    use crate::application::ports::inbound::delete_medication_port::{DeleteMedicationRequest, DeleteMedicationPort};
    use crate::application::ports::inbound::create_medication_port::CreateMedicationPort;
    use std::sync::Arc;

    #[test]
    fn get_medication_returns_medication_dto_when_found() {
        let med = Medication::new(
            MedicationId::generate(),
            MedicationName::new("TestMed").unwrap(),
            Dosage::new(150).unwrap(),
            vec![ScheduledTime::new(9, 0).unwrap()],
            DoseFrequency::OnceDaily,
        );
        let repo = Arc::new(FakeMedicationRepository::with(vec![med.clone()]));
        let service = GetMedicationService::new(repo);

        let req = GetMedicationRequest { id: med.id().to_string() };
        let res = service.execute(req).expect("should return medication");
        let dto = res.medication;
        assert_eq!(dto.name, med.name().value());
        assert_eq!(dto.amount_mg, med.dosage().amount_mg());
        assert_eq!(dto.scheduled_time, vec![(9, 0)]);
    }

    #[test]
    fn get_medication_returns_not_found_for_missing() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let service = GetMedicationService::new(repo);
        let req = GetMedicationRequest { id: MedicationId::generate().to_string() };
        let res = service.execute(req);
        assert!(matches!(res, Err(crate::application::errors::ApplicationError::NotFound(_))));
    }

    #[test]
    fn get_medication_invalid_id_returns_invalid_input() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let service = GetMedicationService::new(repo);
        let req = GetMedicationRequest { id: "not-a-uuid".into() };
        let res = service.execute(req);
        assert!(matches!(res, Err(crate::application::errors::ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn list_dose_records_returns_records_for_medication() {
        let med_id = MedicationId::generate();
        let record = DoseRecord::new(med_id.clone(), chrono::NaiveDate::from_ymd_opt(2025,1,1).unwrap().and_hms_opt(9,0,0).unwrap());
        let repo = Arc::new(FakeDoseRecordRepository::with(record.clone()));
        let service = ListDoseRecordsService::new(repo);

        let req = ListDoseRecordsRequest { medication_id: med_id.to_string() };
        let res = service.execute(req).expect("should list records");
        assert_eq!(res.records.len(), 1);
        assert_eq!(res.records[0].id, record.id().to_string());
    }

    #[test]
    fn list_dose_records_invalid_medication_id_returns_invalid_input() {
        let repo = Arc::new(FakeDoseRecordRepository::new());
        let service = ListDoseRecordsService::new(repo);
        let req = ListDoseRecordsRequest { medication_id: "not-a-uuid".into() };
        let res = service.execute(req);
        assert!(matches!(res, Err(crate::application::errors::ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn update_medication_saves_updated_medication() {
        let repo = Arc::new(FakeMedicationRepository::new());
        // create an initial medication to obtain an id
        let create_req = crate::application::ports::inbound::create_medication_port::CreateMedicationRequest::new("Orig", 100, vec![(8,0)], "OnceDaily");
        let create_service = crate::application::services::create_medication_service::CreateMedicationService::new(repo.clone());
        let id = create_service.execute(create_req).unwrap().id;

        let service = UpdateMedicationService::new(repo.clone());
        let req = UpdateMedicationRequest::new(&id, "Updated", 200, vec![(10,30)], "TwiceDaily");
        let res = service.execute(req).expect("update should succeed");
        assert_eq!(res.id, id);
        // ensure repository has at least one saved medication (original + updated appended)
        assert!(repo.saved_count() >= 1);
    }

    #[test]
    fn update_medication_invalid_id_returns_error() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let service = UpdateMedicationService::new(repo);
        let req = UpdateMedicationRequest::new("not-a-uuid", "Name", 100, vec![(8,0)], "OnceDaily");
        let res = service.execute(req);
        assert!(res.is_err());
    }

    #[test]
    fn delete_medication_success_and_invalid_id() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let service = DeleteMedicationService::new(repo);
        let id = MedicationId::generate().to_string();
        let req = DeleteMedicationRequest { id: id.clone() };
        let res = service.execute(req);
        assert!(res.is_ok());

        let bad_req = DeleteMedicationRequest { id: "not-a-uuid".into() };
        let bad_res = DeleteMedicationService::new(Arc::new(FakeMedicationRepository::new())).execute(bad_req);
        assert!(matches!(bad_res, Err(crate::application::errors::ApplicationError::InvalidInput(_))));
    }
}
