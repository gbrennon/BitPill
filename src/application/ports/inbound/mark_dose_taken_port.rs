use crate::application::dtos::requests::MarkDoseTakenRequest;
use crate::application::dtos::responses::MarkDoseTakenResponse;
use crate::application::errors::ApplicationError;

pub trait MarkDoseTakenPort: Send + Sync {
    fn execute(
        &self,
        request: MarkDoseTakenRequest,
    ) -> Result<MarkDoseTakenResponse, ApplicationError>;
}
