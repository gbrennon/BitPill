use std::sync::Arc;

use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};

use crate::application::errors::{ApplicationError, NotFoundError};
use crate::application::dtos::requests::MarkDoseTakenRequest;
use crate::infrastructure::container::Container;
use chrono::NaiveDateTime;

#[derive(Deserialize)]
pub struct MarkTakenBody {
    pub taken_at: String,
}

#[derive(Serialize)]
pub struct ScheduleResponseBody {
    pub created_count: usize,
}

#[derive(Serialize)]
pub struct MarkTakenResponseBody {
    pub record_id: String,
}

#[derive(Serialize)]
pub struct ErrorBody {
    pub error: String,
}

pub async fn schedule(data: web::Data<Arc<Container>>) -> HttpResponse {
    match data.schedule_dose_service.execute() {
        Ok(records) => HttpResponse::Ok().json(ScheduleResponseBody {
            created_count: records.len(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            error: e.to_string(),
        }),
    }
}

pub async fn mark_taken(
    data: web::Data<Arc<Container>>,
    path: web::Path<String>,
    body: web::Json<MarkTakenBody>,
) -> HttpResponse {
    let id = path.into_inner();

    let taken_at = match NaiveDateTime::parse_from_str(&body.taken_at, "%Y-%m-%dT%H:%M:%S") {
        Ok(dt) => dt,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorBody {
                error: "invalid taken_at format, expected YYYY-MM-DDTHH:MM:SS".into(),
            });
        }
    };

    let request = MarkDoseTakenRequest::new(id, taken_at);
    match data.mark_dose_taken_service.execute(request) {
        Ok(resp) => HttpResponse::Ok().json(MarkTakenResponseBody {
            record_id: resp.record_id,
        }),
        Err(ApplicationError::NotFound(NotFoundError)) => {
            HttpResponse::NotFound().json(ErrorBody {
                error: "dose record not found".into(),
            })
        }
        Err(ApplicationError::Domain(e)) => HttpResponse::BadRequest().json(ErrorBody {
            error: e.to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            error: e.to_string(),
        }),
    }
}
