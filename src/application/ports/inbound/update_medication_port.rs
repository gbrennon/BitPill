use crate::application::errors::ApplicationError;

pub struct UpdateMedicationRequest {
    pub id: String,
    pub name: String,
    pub amount_mg: u32,
    pub scheduled_time: Vec<(u32, u32)>,
    pub dose_frequency: String,
}

impl UpdateMedicationRequest {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        amount_mg: u32,
        scheduled_time: Vec<(u32, u32)>,
        dose_frequency: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            amount_mg,
            scheduled_time,
            dose_frequency: dose_frequency.into(),
        }
    }
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
