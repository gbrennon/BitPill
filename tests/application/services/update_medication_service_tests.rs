use std::sync::Arc;

use bitpill::{
    application::{
        dtos::requests::{CreateMedicationRequest, UpdateMedicationRequest},
        errors::ApplicationError,
        ports::{
            inbound::{
                create_medication_port::CreateMedicationPort,
                update_medication_port::UpdateMedicationPort,
            },
            outbound::medication_repository_port::MedicationRepository,
        },
        services::{
            create_medication_service::CreateMedicationService,
            update_medication_service::UpdateMedicationService,
        },
    },
    domain::value_objects::{medication_frequency::DoseFrequency, medication_id::MedicationId},
};

use crate::fakes::FakeMedicationRepository;

#[cfg(test)]
mod tests {
    use super::*;

    fn seed_and_service(repo: Arc<FakeMedicationRepository>) -> (String, UpdateMedicationService) {
        let id = CreateMedicationService::new(repo.clone())
            .execute(CreateMedicationRequest::new(
                "Orig",
                100,
                vec![(8, 0)],
                "OnceDaily",
            ))
            .unwrap()
            .id;
        let svc = UpdateMedicationService::new(repo);
        (id, svc)
    }

    #[test]
    fn update_medication_saves_updated_medication() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let (id, service) = seed_and_service(repo.clone());

        let res = service
            .execute(UpdateMedicationRequest::new(
                &id,
                "Updated",
                200,
                vec![(10, 30)],
                "TwiceDaily",
            ))
            .expect("update should succeed");

        assert_eq!(res.id, id);
        assert!(repo.saved_count() >= 1);
    }

    #[test]
    fn update_medication_save_failure_returns_storage_error() {
        let repo = Arc::new(FakeMedicationRepository::failing());
        let svc = UpdateMedicationService::new(repo);
        let id = MedicationId::generate().to_string();

        let res = svc.execute(UpdateMedicationRequest::new(
            &id,
            "Name",
            100,
            vec![(8, 0)],
            "OnceDaily",
        ));

        assert!(matches!(res, Err(ApplicationError::Storage(_))));
    }

    #[test]
    fn update_medication_invalid_id_returns_error() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let svc = UpdateMedicationService::new(repo);

        let res = svc.execute(UpdateMedicationRequest::new(
            "not-a-uuid",
            "Name",
            100,
            vec![(8, 0)],
            "OnceDaily",
        ));

        assert!(res.is_err());
    }

    #[test]
    fn update_medication_with_invalid_scheduled_time_returns_domain_error() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let svc = UpdateMedicationService::new(repo);
        let id = MedicationId::generate().to_string();

        let res = svc.execute(UpdateMedicationRequest::new(
            &id,
            "Name",
            100,
            vec![(8, 99)],
            "Custom",
        ));

        assert!(matches!(res, Err(ApplicationError::Domain(_))));
    }

    #[test]
    fn update_medication_unknown_freq_defaults_to_oncedaily() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let svc = UpdateMedicationService::new(repo.clone());
        let id = MedicationId::generate().to_string();

        let res = svc
            .execute(UpdateMedicationRequest::new(
                &id,
                "Name",
                100,
                vec![(8, 0)],
                "UnknownValue",
            ))
            .expect("should succeed and default frequency");

        assert_eq!(res.id, id);
        assert!(repo.saved_count() >= 1);
    }

    #[test]
    fn update_medication_invalid_dosage_returns_domain_error() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let svc = UpdateMedicationService::new(repo);
        let id = MedicationId::generate().to_string();

        let res = svc.execute(UpdateMedicationRequest::new(
            &id,
            "Name",
            0,
            vec![(8, 0)],
            "OnceDaily",
        ));

        assert!(matches!(res, Err(ApplicationError::Domain(_))));
    }

    #[test]
    fn update_medication_custom_sets_custom_frequency_in_repo() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let svc = UpdateMedicationService::new(repo.clone());
        let id = MedicationId::generate().to_string();

        svc.execute(UpdateMedicationRequest::new(
            &id,
            "CustomName",
            123,
            vec![(9, 0), (21, 0)],
            "Custom",
        ))
        .expect("should save");

        let uuid = uuid::Uuid::parse_str(&id).unwrap();
        let mid = MedicationId::from(uuid);
        let saved = repo
            .find_by_id(&mid)
            .expect("repo error")
            .expect("not found");
        assert!(matches!(saved.dose_frequency(), DoseFrequency::Custom(_)));
        let times = saved.scheduled_time();
        assert_eq!(times.len(), 2);
        assert_eq!(times[0].hour(), 9);
        assert_eq!(times[1].hour(), 21);
    }

    #[test]
    fn update_medication_with_thrice_daily_frequency() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let svc = UpdateMedicationService::new(repo.clone());
        let id = MedicationId::generate().to_string();

        let res = svc.execute(UpdateMedicationRequest::new(
            &id,
            "Med",
            100,
            vec![(8, 0), (14, 0), (20, 0)],
            "ThriceDaily",
        ));

        assert!(res.is_ok());
    }
}
