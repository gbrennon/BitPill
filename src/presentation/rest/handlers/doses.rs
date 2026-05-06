use std::sync::Arc;

use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};

use crate::{
    application::{
        dtos::requests::{MarkDoseTakenRequest, ScheduleDoseRequest},
        errors::{ApplicationError, NotFoundError},
    },
    infrastructure::container::Container,
};

#[derive(Deserialize)]
pub struct MarkTakenBody {
    pub notes: Option<String>,
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

pub async fn schedule(data: web::Data<Arc<Container>>) -> HttpResponse {
    match data.schedule_dose_service.execute(ScheduleDoseRequest) {
        Ok(resp) => HttpResponse::Ok().json(ScheduleResponseBody {
            created_count: resp.created.len(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            error: e.to_string(),
        }),
    }
}

pub async fn mark_taken(
    data: web::Data<Arc<Container>>,
    path: web::Path<String>,
    _body: web::Json<MarkTakenBody>,
) -> HttpResponse {
    let id = path.into_inner();

    let request = MarkDoseTakenRequest::new(id);
    match data.mark_dose_taken_service.execute(request) {
        Ok(resp) => HttpResponse::Ok().json(MarkTakenResponseBody {
            record_id: resp.record_id,
        }),
        Err(ApplicationError::NotFound(NotFoundError)) => {
            HttpResponse::NotFound().json(ErrorBody {
                error: "dose record not found".into(),
            })
        }
        Err(ApplicationError::InvalidInput(e)) => {
            HttpResponse::BadRequest().json(ErrorBody { error: e })
        }
        Err(ApplicationError::Domain(e)) => HttpResponse::BadRequest().json(ErrorBody {
            error: e.to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ErrorBody {
            error: e.to_string(),
        }),
    }
}
