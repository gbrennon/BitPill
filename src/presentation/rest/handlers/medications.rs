use std::sync::Arc;

use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};

use crate::{
    application::{
        dtos::requests::{
            CreateMedicationRequest, GetMedicationRequest, ListAllMedicationsRequest,
            UpdateMedicationRequest,
        },
        errors::ApplicationError,
    },
    domain::errors::DomainError,
    infrastructure::container::Container,
};

#[derive(Serialize)]
pub struct MedicationListItem {
    pub id: String,
    pub name: String,
    pub amount_mg: u32,
    pub scheduled_time: Vec<(u32, u32)>,
}

#[derive(Deserialize)]
pub struct CreateMedicationBody {
    pub name: String,
    pub amount_mg: u32,
    pub scheduled_time: Vec<(u32, u32)>,
    pub dose_frequency: Option<String>,
}

#[derive(Serialize)]
pub struct CreateMedicationResponseBody {
    pub id: String,
}

#[derive(Serialize)]
pub struct ErrorBody {
    pub error: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_error_body() {
        let err = ErrorBody {
            error: "fail".to_string(),
        };
        assert_eq!(err.error, "fail");
    }
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
                    scheduled_time: m.scheduled_time,
                })
                .collect();
            HttpResponse::Ok().json(items)
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            error: e.to_string(),
        }),
    }
}

pub async fn get_by_id(data: web::Data<Arc<Container>>, path: web::Path<String>) -> HttpResponse {
    let id = path.into_inner();
    let request = GetMedicationRequest { id };
    match data.get_medication_service.execute(request) {
        Ok(resp) => HttpResponse::Ok().json(MedicationListItem {
            id: resp.medication.id,
            name: resp.medication.name,
            amount_mg: resp.medication.amount_mg,
            scheduled_time: resp.medication.scheduled_time,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            error: e.to_string(),
        }),
    }
}

pub async fn update(
    data: web::Data<Arc<Container>>,
    path: web::Path<String>,
    body: web::Json<CreateMedicationBody>,
) -> HttpResponse {
    let id = path.into_inner();
    let request = UpdateMedicationRequest {
        id,
        name: body.name.clone(),
        amount_mg: body.amount_mg,
        scheduled_time: body.scheduled_time.clone(),
        dose_frequency: "OnceDaily".to_string(),
    };
    match data.update_medication_service.execute(request) {
        Ok(resp) => HttpResponse::Ok().json(CreateMedicationResponseBody { id: resp.id }),
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

pub async fn create(
    data: web::Data<Arc<Container>>,
    body: web::Json<CreateMedicationBody>,
) -> HttpResponse {
    let freq = body
        .dose_frequency
        .clone()
        .unwrap_or_else(|| "OnceDaily".to_string());
    let request = CreateMedicationRequest::new(
        body.name.clone(),
        body.amount_mg,
        body.scheduled_time.clone(),
        freq,
    );

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
