use crate::application::errors::ApplicationError;

pub struct CreateMedicationRequest {
    pub name: String,
    pub amount_mg: u32,
    pub scheduled_times: Vec<(u32, u32)>,
}

impl CreateMedicationRequest {
    pub fn new(name: impl Into<String>, amount_mg: u32, scheduled_times: Vec<(u32, u32)>) -> Self {
        Self {
            name: name.into(),
            amount_mg,
            scheduled_times,
        }
    }
}

pub struct CreateMedicationResponse {
    pub id: String,
}

pub trait CreateMedicationPort: Send + Sync {
    fn execute(
        &self,
        request: CreateMedicationRequest,
    ) -> Result<CreateMedicationResponse, ApplicationError>;
}
