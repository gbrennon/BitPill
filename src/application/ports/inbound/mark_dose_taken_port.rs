use crate::application::{
    dtos::{requests::MarkDoseTakenRequest, responses::MarkDoseTakenResponse},
    errors::ApplicationError,
};

pub trait MarkDoseTakenPort: Send + Sync {
    fn execute(
        &self,
        request: MarkDoseTakenRequest,
    ) -> Result<MarkDoseTakenResponse, ApplicationError>;
}
