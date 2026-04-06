use std::sync::Arc;

use uuid::Uuid;

use crate::{
    application::{
        dtos::{requests::EditMedicationRequest, responses::EditMedicationResponse},
        errors::ApplicationError,
        ports::{
            inbound::edit_medication_port::EditMedicationPort,
            outbound::medication_repository_port::MedicationRepository,
        },
    },
    domain::{
        entities::medication::Medication,
        value_objects::{
            dosage::Dosage, medication_frequency::DoseFrequency, medication_id::MedicationId,
            medication_name::MedicationName, scheduled_time::ScheduledTime,
        },
    },
};

pub struct EditMedicationService {
    repository: Arc<dyn MedicationRepository>,
}

impl EditMedicationService {
    pub fn new(repository: Arc<dyn MedicationRepository>) -> Self {
        Self { repository }
    }
}

impl EditMedicationPort for EditMedicationService {
    fn execute(
        &self,
        request: EditMedicationRequest,
    ) -> Result<EditMedicationResponse, ApplicationError> {
        let uuid = Uuid::parse_str(&request.id)
            .map_err(|_| ApplicationError::InvalidInput("invalid id".into()))?;
        let id = MedicationId::from(uuid);

        let mut errors = Vec::new();

        let name = match MedicationName::new(request.name) {
            Ok(n) => n,
            Err(e) => {
                errors.push(e);
                return Err(ApplicationError::MultipleDomainErrors { errors });
            }
        };

        let dosage = match Dosage::new(request.amount_mg) {
            Ok(d) => d,
            Err(e) => {
                errors.push(e);
                return Err(ApplicationError::MultipleDomainErrors { errors });
            }
        };

        let scheduled_times: Vec<ScheduledTime> = request
            .scheduled_time
            .into_iter()
            .filter_map(|(h, m)| match ScheduledTime::new(h, m) {
                Ok(st) => Some(st),
                Err(e) => {
                    errors.push(e);
                    None
                }
            })
            .collect();

        if !errors.is_empty() {
            return Err(ApplicationError::MultipleDomainErrors { errors });
        }

        let dose_frequency = match request.dose_frequency.as_str() {
            "TwiceDaily" => DoseFrequency::TwiceDaily,
            "ThriceDaily" => DoseFrequency::ThriceDaily,
            "Custom" => DoseFrequency::Custom(scheduled_times.clone()),
            _ => DoseFrequency::OnceDaily,
        };

        match Medication::with_id(id, name, dosage, scheduled_times, dose_frequency) {
            Ok(medication) => {
                self.repository.save(&medication)?;
                Ok(EditMedicationResponse { id: request.id })
            }
            Err(es) => Err(ApplicationError::MultipleDomainErrors { errors: es }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::fakes::FakeMedicationRepository;

    fn make_service(repo: std::sync::Arc<FakeMedicationRepository>) -> EditMedicationService {
        EditMedicationService::new(repo)
    }

    #[test]
    fn execute_with_invalid_uuid_returns_invalid_input() {
        let repo = std::sync::Arc::new(FakeMedicationRepository::new());
        let service = make_service(repo);
        let req = EditMedicationRequest::new("not-a-uuid", "Name", 100, vec![(8, 0)], "OnceDaily");

        let res = service.execute(req);
        assert!(matches!(res, Err(ApplicationError::InvalidInput(_))));
    }

    #[test]
    fn execute_with_empty_name_returns_domain_error() {
        let repo = std::sync::Arc::new(FakeMedicationRepository::new());
        let service = make_service(repo);
        let id = uuid::Uuid::now_v7().to_string();
        let req = EditMedicationRequest::new(id, "", 100, vec![(8, 0)], "OnceDaily");

        let res = service.execute(req);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert!(matches!(err, ApplicationError::MultipleDomainErrors { .. }));
    }

    #[test]
    fn execute_with_zero_dosage_returns_domain_error() {
        let repo = std::sync::Arc::new(FakeMedicationRepository::new());
        let service = make_service(repo);
        let id = uuid::Uuid::now_v7().to_string();
        let req = EditMedicationRequest::new(id, "Test", 0, vec![(8, 0)], "OnceDaily");

        let res = service.execute(req);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert!(matches!(err, ApplicationError::MultipleDomainErrors { .. }));
    }

    #[test]
    fn execute_with_invalid_scheduled_time_returns_domain_error() {
        let repo = std::sync::Arc::new(FakeMedicationRepository::new());
        let service = make_service(repo);
        let id = uuid::Uuid::now_v7().to_string();
        let req = EditMedicationRequest::new(id, "Test", 100, vec![(25, 0)], "OnceDaily");

        let res = service.execute(req);
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert!(matches!(err, ApplicationError::MultipleDomainErrors { .. }));
    }

    #[test]
    fn execute_with_twice_daily_frequency_maps_correctly() {
        let repo = std::sync::Arc::new(FakeMedicationRepository::new());
        let service = make_service(repo.clone());
        let id = uuid::Uuid::now_v7().to_string();
        let req = EditMedicationRequest::new(id, "Test", 100, vec![(8, 0), (20, 0)], "TwiceDaily");

        let res = service.execute(req);

        assert!(res.is_ok());
        assert_eq!(repo.saved_count(), 1);
    }

    #[test]
    fn execute_with_thrice_daily_frequency_maps_correctly() {
        let repo = std::sync::Arc::new(FakeMedicationRepository::new());
        let service = make_service(repo.clone());
        let id = uuid::Uuid::now_v7().to_string();
        let req = EditMedicationRequest::new(
            id,
            "Test",
            100,
            vec![(8, 0), (14, 0), (20, 0)],
            "ThriceDaily",
        );

        let res = service.execute(req);

        assert!(res.is_ok());
        assert_eq!(repo.saved_count(), 1);
    }

    #[test]
    fn execute_with_repository_error_returns_storage_error() {
        let repo = std::sync::Arc::new(FakeMedicationRepository::failing());
        let service = make_service(repo);
        let id = uuid::Uuid::now_v7().to_string();
        let req = EditMedicationRequest::new(id, "Test", 100, vec![(8, 0)], "OnceDaily");

        let res = service.execute(req);

        assert!(matches!(res, Err(ApplicationError::Storage(_))));
    }
}
