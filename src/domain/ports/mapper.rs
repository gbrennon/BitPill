use crate::application::mappers::update_medication_mapper::UpdateMedicationMapper;

pub trait MedicationMapper: Send + Sync {
    type Request;
    
    fn from_request(&self, request: Self::Request, id: Option<MedicationId>) -> Result<Medication, ApplicationError>;
}
