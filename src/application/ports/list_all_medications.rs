use thiserror::Error;

struct ListAllMedicationsRequest {}

struct ListAllMedicationsResponse {
    medications: Vec<MedicationDto>,
}

trait ListAllMedicationsPort {
    fn execute(
        &self,
        request: ListAllMedicationsRequest,
    ) -> Result<ListAllMedicationsResponse, ListAllMedicationsError>;
}
