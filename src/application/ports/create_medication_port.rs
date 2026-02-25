use thiserror::Error;

struct CreateMedicationRequest {
    name: String,
    amount_mg: u32,
    scheduled_times: Vec<(u32, u32)>,
}

struct CreateMedicationResponse {
    id: String,
}

trait CreateMedicationPort {
    fn execute(
        &self,
        request: CreateMedicationRequest,
    ) -> Result<CreateMedicationResponse, Error>;
}
