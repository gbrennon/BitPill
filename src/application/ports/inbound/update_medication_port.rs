use crate::application::errors::ApplicationError;

pub struct UpdateMedicationRequest {
    pub id: String,
    pub name: String,
    pub amount_mg: u32,
    pub scheduled_time: Vec<(u32, u32)>,
    pub dose_frequency: String,
}

pub struct UpdateMedicationResponse {
    pub id: String,
}

pub trait UpdateMedicationPort: Send + Sync {
    fn execute(
        &self,
        request: UpdateMedicationRequest,
    ) -> Result<UpdateMedicationResponse, ApplicationError>;
}
