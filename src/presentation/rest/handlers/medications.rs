use std::sync::Arc;

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::application::errors::ApplicationError;
use crate::application::ports::create_medication_port::{
    CreateMedicationPort, CreateMedicationRequest,
};
use crate::application::ports::list_all_medications_port::{
    ListAllMedicationsPort, ListAllMedicationsRequest,
};
use crate::domain::errors::DomainError;
use crate::infrastructure::container::Container;

#[derive(Serialize)]
pub struct MedicationListItem {
    pub id: String,
    pub name: String,
    pub amount_mg: u32,
    pub scheduled_times: Vec<(u32, u32)>,
}

#[derive(Deserialize)]
pub struct CreateMedicationBody {
    pub name: String,
    pub amount_mg: u32,
    pub scheduled_times: Vec<(u32, u32)>,
}

#[derive(Serialize)]
pub struct CreateMedicationResponseBody {
    pub id: String,
}

#[derive(Serialize)]
pub struct ErrorBody {
    pub error: String,
}

pub async fn list_all(data: web::Data<Arc<Container>>) -> HttpResponse {
    match data
        .list_all_medications_service
        .execute(ListAllMedicationsRequest)
    {
        Ok(resp) => {
            let items: Vec<MedicationListItem> = resp
                .medications
                .into_iter()
                .map(|m| MedicationListItem {
                    id: m.id,
                    name: m.name,
                    amount_mg: m.amount_mg,
                    scheduled_times: m.scheduled_times,
                })
                .collect();
            HttpResponse::Ok().json(items)
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            error: e.to_string(),
        }),
    }
}

pub async fn create(
    data: web::Data<Arc<Container>>,
    body: web::Json<CreateMedicationBody>,
) -> HttpResponse {
    let request =
        CreateMedicationRequest::new(body.name.clone(), body.amount_mg, body.scheduled_times.clone());

    match data.create_medication_service.execute(request) {
        Ok(resp) => HttpResponse::Created().json(CreateMedicationResponseBody { id: resp.id }),
        Err(ApplicationError::Domain(
            DomainError::InvalidDosage
            | DomainError::EmptyMedicationName
            | DomainError::InvalidScheduledTime,
        )) => HttpResponse::BadRequest().json(ErrorBody {
            error: "invalid input".into(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            error: e.to_string(),
        }),
    }
}
