use std::sync::Arc;
use uuid::Uuid;

use crate::application::errors::{ApplicationError, NotFoundError};
use crate::application::ports::inbound::get_medication_port::{
    GetMedicationPort, GetMedicationRequest, GetMedicationResponse, MedicationDto,
};
use crate::application::ports::outbound::medication_repository_port::MedicationRepository;
use crate::domain::value_objects::medication_id::MedicationId;

pub struct GetMedicationService {
    repository: Arc<dyn MedicationRepository>,
}

impl GetMedicationService {
    pub fn new(repository: Arc<dyn MedicationRepository>) -> Self {
        Self { repository }
    }
}


impl GetMedicationPort for GetMedicationService {
    fn execute(
        &self,
        request: GetMedicationRequest,
    ) -> Result<GetMedicationResponse, ApplicationError> {
        let uuid = Uuid::parse_str(&request.id)
            .map_err(|_| ApplicationError::InvalidInput("invalid id".into()))?;
        let id = MedicationId::from(uuid);
        match self.repository.find_by_id(&id)? {
            Some(m) => Ok(GetMedicationResponse {
                medication: MedicationDto {
                    id: m.id().to_string(),
                    name: m.name().value().to_string(),
                    amount_mg: m.dosage().amount_mg(),
                    scheduled_time: m
                        .scheduled_time()
                        .iter()
                        .map(|t| (t.hour(), t.minute()))
                        .collect(),
                    dose_frequency: match m.dose_frequency() {
                        crate::domain::value_objects::medication_frequency::DoseFrequency::OnceDaily => "OnceDaily".into(),
                        crate::domain::value_objects::medication_frequency::DoseFrequency::TwiceDaily => "TwiceDaily".into(),
                        crate::domain::value_objects::medication_frequency::DoseFrequency::ThriceDaily => "ThriceDaily".into(),
                        crate::domain::value_objects::medication_frequency::DoseFrequency::Custom(_) => "Custom".into(),
                        crate::domain::value_objects::medication_frequency::DoseFrequency::EveryXHours(_) => "EveryXHours".into(),
                    },
                },
            }),
            None => Err(ApplicationError::NotFound(NotFoundError)),
        }
    }
}

// Unit tests for GetMedicationService placed in same file as impl
#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::ports::fakes::FakeMedicationRepository;
    use crate::domain::entities::medication::Medication as DomainMedication;
    use crate::domain::value_objects::{
        medication_name::MedicationName,
        dosage::Dosage,
        scheduled_time::ScheduledTime,
        medication_frequency::DoseFrequency,
    };

    #[test]
    fn get_medication_handles_custom_and_everyxhours_frequency() {
        let id1 = MedicationId::generate();
        let custom_times = vec![ScheduledTime::new(9, 0).unwrap(), ScheduledTime::new(21, 0).unwrap()];
        let med_custom = DomainMedication::new(
            id1.clone(),
            MedicationName::new("CustomMed").unwrap(),
            Dosage::new(50).unwrap(),
            custom_times.clone(),
            DoseFrequency::Custom(custom_times.clone()),
        );

        let id2 = MedicationId::generate();
        let med_every = DomainMedication::new(
            id2.clone(),
            MedicationName::new("EveryMed").unwrap(),
            Dosage::new(25).unwrap(),
            vec![],
            DoseFrequency::EveryXHours(6),
        );

        let repo = std::sync::Arc::new(FakeMedicationRepository::with(vec![med_custom.clone(), med_every.clone()]));
        let svc = GetMedicationService::new(repo);

        let req_custom = GetMedicationRequest { id: id1.to_string() };
        let res_custom = svc.execute(req_custom).expect("should return custom med");
        assert_eq!(res_custom.medication.dose_frequency, "Custom");

        let req_every = GetMedicationRequest { id: id2.to_string() };
        let res_every = svc.execute(req_every).expect("should return every med");
        assert_eq!(res_every.medication.dose_frequency, "EveryXHours");
    }

    #[test]
    fn get_medication_fixed_frequencies_map_to_strings() {
        let id_once = MedicationId::generate();
        let m_once = DomainMedication::new(
            id_once.clone(),
            MedicationName::new("OnceMed").unwrap(),
            Dosage::new(10).unwrap(),
            vec![ScheduledTime::new(8, 0).unwrap()],
            DoseFrequency::OnceDaily,
        );

        let id_twice = MedicationId::generate();
        let m_twice = DomainMedication::new(
            id_twice.clone(),
            MedicationName::new("TwiceMed").unwrap(),
            Dosage::new(20).unwrap(),
            vec![ScheduledTime::new(8, 0).unwrap(), ScheduledTime::new(20, 0).unwrap()],
            DoseFrequency::TwiceDaily,
        );

        let id_thrice = MedicationId::generate();
        let m_thrice = DomainMedication::new(
            id_thrice.clone(),
            MedicationName::new("ThriceMed").unwrap(),
            Dosage::new(30).unwrap(),
            vec![ScheduledTime::new(8, 0).unwrap(), ScheduledTime::new(14, 0).unwrap(), ScheduledTime::new(20, 0).unwrap()],
            DoseFrequency::ThriceDaily,
        );

        let repo = std::sync::Arc::new(FakeMedicationRepository::with(vec![m_once.clone(), m_twice.clone(), m_thrice.clone()]));
        let svc = GetMedicationService::new(repo);

        let r1 = svc.execute(GetMedicationRequest { id: id_once.to_string() }).unwrap();
        assert_eq!(r1.medication.dose_frequency, "OnceDaily");
        let r2 = svc.execute(GetMedicationRequest { id: id_twice.to_string() }).unwrap();
        assert_eq!(r2.medication.dose_frequency, "TwiceDaily");
        let r3 = svc.execute(GetMedicationRequest { id: id_thrice.to_string() }).unwrap();
        assert_eq!(r3.medication.dose_frequency, "ThriceDaily");
    }

    // Tests moved from additional_tests.rs
    #[test]
    fn get_medication_returns_medication_dto_when_found() {
        let med = DomainMedication::new(
            MedicationId::generate(),
            MedicationName::new("TestMed").unwrap(),
            Dosage::new(150).unwrap(),
            vec![ScheduledTime::new(9, 0).unwrap()],
            DoseFrequency::OnceDaily,
        );
        let repo = std::sync::Arc::new(FakeMedicationRepository::with(vec![med.clone()]));
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
        let repo = std::sync::Arc::new(FakeMedicationRepository::new());
        let service = GetMedicationService::new(repo);
        let req = GetMedicationRequest { id: MedicationId::generate().to_string() };
        let res = service.execute(req);
        assert!(matches!(res, Err(crate::application::errors::ApplicationError::NotFound(_))));
    }

    #[test]
    fn get_medication_invalid_id_returns_invalid_input() {
        let repo = std::sync::Arc::new(FakeMedicationRepository::new());
        let service = GetMedicationService::new(repo);
        let req = GetMedicationRequest { id: "not-a-uuid".into() };
        let res = service.execute(req);
        assert!(matches!(res, Err(crate::application::errors::ApplicationError::InvalidInput(_))));
    }
}
