use std::convert::TryFrom;

use crate::{
    application::{
        dtos::requests::{CreateMedicationRequest, UpdateMedicationRequest},
        errors::ApplicationError,
        mappers::{
            create_medication_mapper::CreateMedicationMapper,
            update_medication_mapper::UpdateMedicationMapper,
        },
    },
    domain::{
        entities::medication::Medication, ports::mapper::Mapper,
        value_objects::medication_id::MedicationId,
    },
};

impl TryFrom<CreateMedicationRequest> for Medication {
    type Error = ApplicationError;

    fn try_from(request: CreateMedicationRequest) -> Result<Self, Self::Error> {
        CreateMedicationMapper
            .map(request)
            .map_err(|e| ApplicationError::MultipleDomainErrors { errors: vec![e] })
    }
}

impl TryFrom<(UpdateMedicationRequest, MedicationId)> for Medication {
    type Error = ApplicationError;

    fn try_from(src: (UpdateMedicationRequest, MedicationId)) -> Result<Self, Self::Error> {
        UpdateMedicationMapper
            .map(src)
            .map_err(|e| ApplicationError::MultipleDomainErrors { errors: e })
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    fn create_req(name: &str, amount_mg: u32, freq: &str) -> CreateMedicationRequest {
        CreateMedicationRequest::new(name, amount_mg, vec![(8, 0)], freq)
    }

    fn update_req(name: &str, amount_mg: u32, freq: &str) -> UpdateMedicationRequest {
        UpdateMedicationRequest::new(Uuid::nil().to_string(), name, amount_mg, vec![(8, 0)], freq)
    }

    // --- CreateMedicationRequest ---

    #[test]
    fn try_from_create_request_with_valid_data_returns_medication() {
        let result = Medication::try_from(create_req("Aspirin", 500, "OnceDaily"));

        assert!(result.is_ok());
        let med = result.unwrap();
        assert_eq!(med.name().value(), "Aspirin");
        assert_eq!(med.dosage().amount_mg(), 500);
        assert!(!med.id().to_string().is_empty());
    }

    #[test]
    fn try_from_create_request_with_empty_name_returns_error() {
        let result = Medication::try_from(create_req("", 500, "OnceDaily"));

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, ApplicationError::MultipleDomainErrors { .. }));
        if let ApplicationError::MultipleDomainErrors { errors } = err {
            assert!(errors.contains(&crate::domain::errors::DomainError::EmptyMedicationName));
        }
    }

    #[test]
    fn try_from_create_request_with_zero_dosage_returns_error() {
        let result = Medication::try_from(create_req("Aspirin", 0, "OnceDaily"));

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, ApplicationError::MultipleDomainErrors { .. }));
        if let ApplicationError::MultipleDomainErrors { errors } = err {
            assert!(errors.contains(&crate::domain::errors::DomainError::InvalidDosage));
        }
    }

    #[test]
    fn try_from_create_request_with_invalid_scheduled_time_returns_error() {
        let request = CreateMedicationRequest::new("Aspirin", 500, vec![(25, 0)], "OnceDaily");

        let result = Medication::try_from(request);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, ApplicationError::MultipleDomainErrors { .. }));
        if let ApplicationError::MultipleDomainErrors { errors } = err {
            assert!(errors.contains(&crate::domain::errors::DomainError::InvalidScheduledTime));
        }
    }

    #[test]
    fn try_from_create_request_each_call_generates_unique_id() {
        let med_a = Medication::try_from(create_req("Aspirin", 100, "OnceDaily")).unwrap();
        let med_b = Medication::try_from(create_req("Aspirin", 100, "OnceDaily")).unwrap();

        assert_ne!(med_a.id(), med_b.id());
    }

    // --- (UpdateMedicationRequest, MedicationId) ---

    #[test]
    fn try_from_update_tuple_with_valid_data_returns_medication_with_given_id() {
        let id = MedicationId::from(Uuid::nil());
        let request = UpdateMedicationRequest::new(
            Uuid::nil().to_string(),
            "Ibuprofen",
            200,
            vec![(8, 0), (20, 0)],
            "TwiceDaily",
        );

        let result = Medication::try_from((request, id.clone()));

        assert!(result.is_ok());
        let med = result.unwrap();
        assert_eq!(med.id(), &id);
        assert_eq!(med.name().value(), "Ibuprofen");
        assert_eq!(med.dosage().amount_mg(), 200);
    }

    #[test]
    fn try_from_update_tuple_with_empty_name_returns_error() {
        let id = MedicationId::from(Uuid::nil());

        let result = Medication::try_from((update_req("", 200, "OnceDaily"), id));

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, ApplicationError::MultipleDomainErrors { .. }));
        if let ApplicationError::MultipleDomainErrors { errors } = err {
            assert!(errors.contains(&crate::domain::errors::DomainError::EmptyMedicationName));
        }
    }

    #[test]
    fn try_from_update_tuple_with_zero_dosage_returns_error() {
        let id = MedicationId::from(Uuid::nil());

        let result = Medication::try_from((update_req("Aspirin", 0, "OnceDaily"), id));

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, ApplicationError::MultipleDomainErrors { .. }));
        if let ApplicationError::MultipleDomainErrors { errors } = err {
            assert!(errors.contains(&crate::domain::errors::DomainError::InvalidDosage));
        }
    }

    #[test]
    fn try_from_update_tuple_with_invalid_scheduled_time_returns_error() {
        let id = MedicationId::from(Uuid::nil());
        let request = UpdateMedicationRequest::new(
            Uuid::nil().to_string(),
            "Aspirin",
            500,
            vec![(25, 0)],
            "OnceDaily",
        );

        let result = Medication::try_from((request, id));

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, ApplicationError::MultipleDomainErrors { .. }));
        if let ApplicationError::MultipleDomainErrors { errors } = err {
            assert!(errors.contains(&crate::domain::errors::DomainError::InvalidScheduledTime));
        }
    }
}
