use crate::domain::{
    entities::medication::{CreateMedication, UpdateMedication},
    value_objects::*,
};

pub struct MedicationMapper {}

impl MedicationMapper {
    pub fn from_request<T>(request: T) -> Result<CreateMedication, ApplicationError>
    where
        T: Into<crate::application::dtos::requests::CreateMedicationRequest>,
    {
        let request = request.into();
        
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

        Ok(CreateMedication {
            id: MedicationId::generate(),
            name,
            dosage,
            scheduled_time: times,
            dose_frequency,
        })
    }
}
