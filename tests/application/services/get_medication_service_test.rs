use std::sync::Arc;

use bitpill::{
    application::{
        dtos::requests::GetMedicationRequest, errors::ApplicationError,
        ports::inbound::get_medication_port::GetMedicationPort,
        services::get_medication_service::GetMedicationService,
    },
    domain::{
        entities::medication::Medication,
        value_objects::{
            dosage::Dosage, medication_frequency::DoseFrequency, medication_id::MedicationId,
            medication_name::MedicationName, scheduled_time::ScheduledTime,
        },
    },
};

use crate::fakes::{FakeDoseRecordRepository, FakeMedicationRepository};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_medication_returns_medication_dto_when_found() {
        let med = Medication::new(
            MedicationId::generate(),
            MedicationName::new("TestMed").unwrap(),
            Dosage::new(150).unwrap(),
            vec![ScheduledTime::new(9, 0).unwrap()],
            DoseFrequency::OnceDaily,
        )
        .unwrap();
        let repo = Arc::new(FakeMedicationRepository::with(vec![med.clone()]));
        let service = GetMedicationService::new(repo, Arc::new(FakeDoseRecordRepository::new()));

        let res = service
            .execute(GetMedicationRequest {
                id: med.id().to_string(),
            })
            .expect("should return medication");

        let dto = res.medication;
        assert_eq!(dto.name, med.name().value());
        assert_eq!(dto.amount_mg, med.dosage().amount_mg());
        assert_eq!(dto.scheduled_time, vec![(9, 0)]);
    }

    #[test]
    fn get_medication_returns_not_found_for_missing() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let service = GetMedicationService::new(repo, Arc::new(FakeDoseRecordRepository::new()));

        let res = service.execute(GetMedicationRequest {
            id: MedicationId::generate().to_string(),
        });

        assert!(matches!(res, Err(ApplicationError::NotFound(_))));
    }

    #[test]
    fn get_medication_invalid_id_returns_invalid_input() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let service = GetMedicationService::new(repo, Arc::new(FakeDoseRecordRepository::new()));

        let res = service.execute(GetMedicationRequest {
            id: "not-a-uuid".into(),
        });

        assert!(matches!(res, Err(ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn get_medication_handles_custom_and_everyxhours_frequency() {
        let id1 = MedicationId::generate();
        let custom_times = vec![
            ScheduledTime::new(8, 0).unwrap(),
            ScheduledTime::new(12, 0).unwrap(),
            ScheduledTime::new(16, 0).unwrap(),
            ScheduledTime::new(20, 0).unwrap(),
        ];
        let med_custom = Medication::new(
            id1.clone(),
            MedicationName::new("CustomMed").unwrap(),
            Dosage::new(50).unwrap(),
            custom_times.clone(),
            DoseFrequency::Custom(custom_times.clone()),
        )
        .unwrap();

        let id2 = MedicationId::generate();
        let med_thrice = Medication::new(
            id2.clone(),
            MedicationName::new("ThriceMed").unwrap(),
            Dosage::new(25).unwrap(),
            vec![
                ScheduledTime::new(8, 0).unwrap(),
                ScheduledTime::new(14, 0).unwrap(),
                ScheduledTime::new(20, 0).unwrap(),
            ],
            DoseFrequency::ThriceDaily,
        )
        .unwrap();

        let repo = Arc::new(FakeMedicationRepository::with(vec![med_custom, med_thrice]));
        let svc = GetMedicationService::new(repo, Arc::new(FakeDoseRecordRepository::new()));

        let res_custom = svc
            .execute(GetMedicationRequest {
                id: id1.to_string(),
            })
            .unwrap();
        assert_eq!(res_custom.medication.dose_frequency, "Custom");

        let res_thrice = svc
            .execute(GetMedicationRequest {
                id: id2.to_string(),
            })
            .unwrap();
        assert_eq!(res_thrice.medication.dose_frequency, "ThriceDaily");
    }

    #[test]
    fn get_medication_fixed_frequencies_map_to_strings() {
        let id_once = MedicationId::generate();
        let m_once = Medication::new(
            id_once.clone(),
            MedicationName::new("OnceMed").unwrap(),
            Dosage::new(10).unwrap(),
            vec![ScheduledTime::new(8, 0).unwrap()],
            DoseFrequency::OnceDaily,
        )
        .unwrap();
        let id_twice = MedicationId::generate();
        let m_twice = Medication::new(
            id_twice.clone(),
            MedicationName::new("TwiceMed").unwrap(),
            Dosage::new(20).unwrap(),
            vec![
                ScheduledTime::new(8, 0).unwrap(),
                ScheduledTime::new(20, 0).unwrap(),
            ],
            DoseFrequency::TwiceDaily,
        )
        .unwrap();
        let id_thrice = MedicationId::generate();
        let m_thrice = Medication::new(
            id_thrice.clone(),
            MedicationName::new("ThriceMed").unwrap(),
            Dosage::new(30).unwrap(),
            vec![
                ScheduledTime::new(8, 0).unwrap(),
                ScheduledTime::new(14, 0).unwrap(),
                ScheduledTime::new(20, 0).unwrap(),
            ],
            DoseFrequency::ThriceDaily,
        )
        .unwrap();

        let repo = Arc::new(FakeMedicationRepository::with(vec![
            m_once, m_twice, m_thrice,
        ]));
        let svc = GetMedicationService::new(repo, Arc::new(FakeDoseRecordRepository::new()));

        let r1 = svc
            .execute(GetMedicationRequest {
                id: id_once.to_string(),
            })
            .unwrap();
        assert_eq!(r1.medication.dose_frequency, "OnceDaily");
        let r2 = svc
            .execute(GetMedicationRequest {
                id: id_twice.to_string(),
            })
            .unwrap();
        assert_eq!(r2.medication.dose_frequency, "TwiceDaily");
        let r3 = svc
            .execute(GetMedicationRequest {
                id: id_thrice.to_string(),
            })
            .unwrap();
        assert_eq!(r3.medication.dose_frequency, "ThriceDaily");
    }

    #[test]
    fn get_medication_when_repository_fails_returns_storage_error() {
        use bitpill::application::errors::ApplicationError;
        let repo = Arc::new(FakeMedicationRepository::failing_on_find_by_id());
        let service = GetMedicationService::new(repo, Arc::new(FakeDoseRecordRepository::new()));
        let id =
            bitpill::domain::value_objects::medication_id::MedicationId::generate().to_string();

        let res = service.execute(GetMedicationRequest { id });

        assert!(matches!(res, Err(ApplicationError::Storage(_))));
    }
}
