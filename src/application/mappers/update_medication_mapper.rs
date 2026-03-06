use super::*;
use crate::domain::ports::mapper::MedicationMapper;

pub struct UpdateMedicationMapper;

impl MedicationMapper for UpdateMedicationMapper {
    type Request = UpdateMedicationRequest;
    
    fn from_request(&self, request: Self::Request, id: Option<MedicationId>) -> Result<Medication, ApplicationError> {
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

        Ok(Medication::new(
            id.unwrap(), // We must have an ID for update
            name,
            dosage,
            times,
            dose_frequency,
        ))
    }
}
