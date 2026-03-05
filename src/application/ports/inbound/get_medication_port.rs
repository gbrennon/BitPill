use crate::application::errors::ApplicationError;

pub struct GetMedicationRequest {
    pub id: String,
}

pub struct GetMedicationResponse {
    pub medication: MedicationDto,
}

pub struct MedicationDto {
    pub id: String,
    pub name: String,
    pub amount_mg: u32,
    pub scheduled_time: Vec<(u32, u32)>,
    pub dose_frequency: String,
}

pub trait GetMedicationPort: Send + Sync {
    fn execute(
        &self,
        request: GetMedicationRequest,
    ) -> Result<GetMedicationResponse, ApplicationError>;
}
