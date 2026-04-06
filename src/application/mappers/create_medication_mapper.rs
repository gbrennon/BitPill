use crate::{
    application::dtos::requests::CreateMedicationRequest,
    domain::{
        entities::medication::Medication,
        errors::DomainError,
        ports::mapper::Mapper,
        value_objects::{
            dosage::Dosage, medication_frequency::DoseFrequency, medication_id::MedicationId,
            medication_name::MedicationName, scheduled_time::ScheduledTime,
        },
    },
};

/// Mapper that produces a `Medication` from a `CreateMedicationRequest`.
///
/// Responsibility: parse and validate DTO fields and construct a fully-formed
/// domain `Medication`. This lives in the application layer because it depends on
/// DTO types and is an adapter around pure domain constructors and value objects.
///
/// The mapper consumes the request by-value and returns a `DomainError` on validation failure.
pub struct CreateMedicationMapper;

impl Mapper<Medication> for CreateMedicationMapper {
    type Source = CreateMedicationRequest;

    fn map(&self, request: CreateMedicationRequest) -> Result<Medication, DomainError> {
        let name = MedicationName::new(request.name)?;
        let dosage = Dosage::new(request.amount_mg)?;
        let times = request
            .scheduled_time
            .into_iter()
            .map(|(h, m)| ScheduledTime::new(h, m))
            .collect::<Result<Vec<_>, _>>()?;

        let dose_frequency = match request.dose_frequency.as_str() {
            "OnceDaily" => DoseFrequency::OnceDaily,
            "TwiceDaily" => DoseFrequency::TwiceDaily,
            "ThriceDaily" => DoseFrequency::ThriceDaily,
            "Custom" => DoseFrequency::Custom(times.clone()),
            _ => DoseFrequency::OnceDaily,
        };

        Medication::new(
            MedicationId::generate(),
            name,
            dosage,
            times,
            dose_frequency,
        )
        .map_err(|errors| {
            errors
                .into_iter()
                .next()
                .unwrap_or(DomainError::EmptyMedicationName)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_request(name: &str, amount_mg: u32, freq: &str) -> CreateMedicationRequest {
        CreateMedicationRequest::new(name, amount_mg, vec![(8, 0)], freq)
    }

    #[test]
    fn map_with_valid_request_returns_medication() {
        let mapper = CreateMedicationMapper;
        let request = make_request("Aspirin", 500, "OnceDaily");

        let result = mapper.map(request);

        assert!(result.is_ok());
        let med = result.unwrap();
        assert_eq!(med.name().value(), "Aspirin");
        assert_eq!(med.dosage().amount_mg(), 500);
    }

    #[test]
    fn map_with_empty_name_returns_domain_error() {
        let mapper = CreateMedicationMapper;
        let request = make_request("", 500, "OnceDaily");

        let result = mapper.map(request);

        assert!(matches!(result, Err(DomainError::EmptyMedicationName)));
    }

    #[test]
    fn map_with_zero_dosage_returns_domain_error() {
        let mapper = CreateMedicationMapper;
        let request = make_request("Aspirin", 0, "OnceDaily");

        let result = mapper.map(request);

        assert!(matches!(result, Err(DomainError::InvalidDosage)));
    }

    #[test]
    fn map_with_invalid_time_returns_domain_error() {
        let mapper = CreateMedicationMapper;
        let request = CreateMedicationRequest::new("Aspirin", 500, vec![(25, 0)], "OnceDaily");

        let result = mapper.map(request);

        assert!(matches!(result, Err(DomainError::InvalidScheduledTime)));
    }

    #[test]
    fn map_parses_twice_daily_frequency() {
        let mapper = CreateMedicationMapper;
        let request =
            CreateMedicationRequest::new("Ibuprofen", 200, vec![(8, 0), (20, 0)], "TwiceDaily");

        let med = mapper.map(request).unwrap();

        assert_eq!(med.dose_frequency(), &DoseFrequency::TwiceDaily);
    }

    #[test]
    fn map_unknown_frequency_defaults_to_once_daily() {
        let mapper = CreateMedicationMapper;
        let request = make_request("Ibuprofen", 200, "Unknown");

        let med = mapper.map(request).unwrap();

        assert_eq!(med.dose_frequency(), &DoseFrequency::OnceDaily);
    }
}
