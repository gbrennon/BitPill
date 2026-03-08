use crate::application::dtos::requests::UpdateMedicationRequest;
use crate::domain::entities::medication::Medication;
use crate::domain::errors::DomainError;
use crate::domain::ports::mapper::Mapper;
use crate::domain::value_objects::{
    dosage::Dosage, medication_frequency::DoseFrequency, medication_id::MedicationId,
    medication_name::MedicationName, scheduled_time::ScheduledTime,
};

/// Mapper that produces a `Medication` from an `(UpdateMedicationRequest, MedicationId)` tuple.
///
/// Responsibility: validate and map incoming update DTO fields into the existing
/// domain `Medication` identity supplied by `MedicationId`. Keeps mapping logic
/// centralized so services remain thin. Consumes the input tuple by-value and
/// returns `DomainError` on validation failure.
pub struct UpdateMedicationMapper;

impl Mapper<Medication> for UpdateMedicationMapper {
    type Source = (UpdateMedicationRequest, MedicationId);

    fn map(&self, src: (UpdateMedicationRequest, MedicationId)) -> Result<Medication, DomainError> {
        let (request, id) = src;
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

        Ok(Medication::new(id, name, dosage, times, dose_frequency))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn make_request(name: &str, amount_mg: u32, freq: &str) -> UpdateMedicationRequest {
        UpdateMedicationRequest::new(Uuid::nil().to_string(), name, amount_mg, vec![(8, 0)], freq)
    }

    fn make_id() -> MedicationId {
        MedicationId::from(Uuid::nil())
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

        assert!(matches!(result, Err(DomainError::EmptyMedicationName)));
    }

    #[test]
    fn map_with_zero_dosage_returns_domain_error() {
        let mapper = UpdateMedicationMapper;
        let request = make_request("Aspirin", 0, "OnceDaily");

        let result = mapper.map((request, make_id()));

        assert!(matches!(result, Err(DomainError::InvalidDosage)));
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

        assert!(matches!(result, Err(DomainError::InvalidScheduledTime)));
    }

    #[test]
    fn map_unknown_frequency_defaults_to_once_daily() {
        let mapper = UpdateMedicationMapper;
        let request = make_request("Ibuprofen", 200, "Unknown");

        let med = mapper.map((request, make_id())).unwrap();

        assert_eq!(med.dose_frequency(), &DoseFrequency::OnceDaily);
    }
}
