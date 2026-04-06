use crate::{
    application::dtos::requests::UpdateMedicationRequest,
    domain::{
        entities::medication::Medication,
        errors::DomainError,
        value_objects::{
            dosage::Dosage, medication_frequency::DoseFrequency, medication_id::MedicationId,
            medication_name::MedicationName, scheduled_time::ScheduledTime,
        },
    },
};

pub struct UpdateMedicationMapper;

impl UpdateMedicationMapper {
    pub fn map(
        &self,
        src: (UpdateMedicationRequest, MedicationId),
    ) -> Result<Medication, Vec<DomainError>> {
        let mut errors = Vec::new();
        let (request, id) = src;

        let name = match MedicationName::new(request.name) {
            Ok(n) => n,
            Err(e) => {
                errors.push(e);
                return Err(errors);
            }
        };

        let dosage = match Dosage::new(request.amount_mg) {
            Ok(d) => d,
            Err(e) => {
                errors.push(e);
                return Err(errors);
            }
        };

        let times: Vec<ScheduledTime> = request
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

        if errors.is_empty() {
            let dose_frequency = match request.dose_frequency.as_str() {
                "OnceDaily" => DoseFrequency::OnceDaily,
                "TwiceDaily" => DoseFrequency::TwiceDaily,
                "ThriceDaily" => DoseFrequency::ThriceDaily,
                "Custom" => DoseFrequency::Custom(times.clone()),
                _ => DoseFrequency::OnceDaily,
            };

            return Medication::new(id, name, dosage, times, dose_frequency);
        }

        let dose_frequency = match request.dose_frequency.as_str() {
            "OnceDaily" => DoseFrequency::OnceDaily,
            "TwiceDaily" => DoseFrequency::TwiceDaily,
            "ThriceDaily" => DoseFrequency::ThriceDaily,
            "Custom" => DoseFrequency::Custom(times.clone()),
            _ => DoseFrequency::OnceDaily,
        };

        if let Err(mut scheduled_errors) = Medication::new(id, name, dosage, times, dose_frequency)
        {
            errors.append(&mut scheduled_errors);
        }

        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    fn make_request(name: &str, amount_mg: u32, freq: &str) -> UpdateMedicationRequest {
        UpdateMedicationRequest::new(Uuid::nil().to_string(), name, amount_mg, vec![(8, 0)], freq)
    }

    fn make_id() -> MedicationId {
        MedicationId::from(Uuid::nil())
    }

    fn assert_error_contains(errors: Vec<DomainError>, expected: DomainError) {
        assert!(
            errors.contains(&expected),
            "expected errors to contain {:?}, got {:?}",
            expected,
            errors
        );
    }

    #[test]
    fn map_with_valid_request_returns_medication_with_given_id() {
        let mapper = UpdateMedicationMapper;
        let id = make_id();
        let request = make_request("Aspirin", 500, "OnceDaily");

        let result = mapper.map((request, id.clone()));

        assert!(result.is_ok());
        let med = result.unwrap();
        assert_eq!(med.id(), &id);
        assert_eq!(med.name().value(), "Aspirin");
        assert_eq!(med.dosage().amount_mg(), 500);
    }

    #[test]
    fn map_with_empty_name_returns_domain_error() {
        let mapper = UpdateMedicationMapper;
        let request = make_request("", 500, "OnceDaily");

        let result = mapper.map((request, make_id()));

        assert!(result.is_err());
        assert_error_contains(result.unwrap_err(), DomainError::EmptyMedicationName);
    }

    #[test]
    fn map_with_zero_dosage_returns_domain_error() {
        let mapper = UpdateMedicationMapper;
        let request = make_request("Aspirin", 0, "OnceDaily");

        let result = mapper.map((request, make_id()));

        assert!(result.is_err());
        assert_error_contains(result.unwrap_err(), DomainError::InvalidDosage);
    }

    #[test]
    fn map_with_invalid_time_returns_domain_error() {
        let mapper = UpdateMedicationMapper;
        let request = UpdateMedicationRequest::new(
            Uuid::nil().to_string(),
            "Aspirin",
            500,
            vec![(25, 0)],
            "OnceDaily",
        );

        let result = mapper.map((request, make_id()));

        assert!(result.is_err());
        assert_error_contains(result.unwrap_err(), DomainError::InvalidScheduledTime);
    }

    #[test]
    fn map_unknown_frequency_defaults_to_once_daily() {
        let mapper = UpdateMedicationMapper;
        let request = make_request("Ibuprofen", 200, "Unknown");

        let med = mapper.map((request, make_id())).unwrap();

        assert_eq!(med.dose_frequency(), &DoseFrequency::OnceDaily);
    }

    #[test]
    fn map_twice_daily_without_required_times_returns_error() {
        let mapper = UpdateMedicationMapper;
        let request = UpdateMedicationRequest::new(
            Uuid::nil().to_string(),
            "Test",
            100,
            vec![],
            "TwiceDaily",
        );

        let result = mapper.map((request, make_id()));

        assert!(result.is_err());
        assert_error_contains(result.unwrap_err(), DomainError::InvalidScheduledTimesCount);
    }

    #[test]
    fn map_with_duplicate_times_returns_error() {
        let mapper = UpdateMedicationMapper;
        let request = UpdateMedicationRequest::new(
            Uuid::nil().to_string(),
            "Test",
            100,
            vec![(8, 0), (8, 0)],
            "TwiceDaily",
        );

        let result = mapper.map((request, make_id()));

        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.contains(&DomainError::DuplicateScheduledTime));
    }

    #[test]
    fn map_once_daily_with_multiple_times_returns_error() {
        let mapper = UpdateMedicationMapper;
        let request = UpdateMedicationRequest::new(
            Uuid::nil().to_string(),
            "Test",
            100,
            vec![(8, 0), (20, 0)],
            "OnceDaily",
        );

        let result = mapper.map((request, make_id()));

        assert!(result.is_err());
        assert_error_contains(result.unwrap_err(), DomainError::InvalidScheduledTimesCount);
    }

    #[test]
    fn map_with_empty_name_and_zero_dosage_returns_aggregated_errors() {
        let mapper = UpdateMedicationMapper;
        let request =
            UpdateMedicationRequest::new(Uuid::nil().to_string(), "", 0, vec![(8, 0)], "OnceDaily");

        let result = mapper.map((request, make_id()));

        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.contains(&DomainError::EmptyMedicationName));
    }
}
