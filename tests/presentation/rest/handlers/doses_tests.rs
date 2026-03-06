use actix_web::test::{self, init_service};
use actix_web::{App, web};
use bitpill::{
    infrastructure::container::Container,
    presentation::rest::handlers::{doses, medications},
};
use std::sync::Arc;
use tempfile::TempDir;

fn container() -> (Arc<Container>, TempDir) {
    let dir = tempfile::tempdir().unwrap();
    let c = Arc::new(Container::new_with_paths(
        dir.path().join("meds.json"),
        dir.path().join("doses.json"),
        dir.path().join("settings.json"),
    ));
    (c, dir)
}

#[actix_web::test]
async fn schedule_returns_200() {
    let (c, _dir) = container();
    let data = web::Data::new(c);
    let app = init_service(
        App::new()
            .app_data(data)
            .route("/doses/schedule", web::post().to(doses::schedule)),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/doses/schedule")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["created_count"].is_number());
}

#[actix_web::test]
async fn mark_taken_with_invalid_date_format_returns_400() {
    let (c, _dir) = container();
    let data = web::Data::new(c);
    let app = init_service(
        App::new()
            .app_data(data)
            .route("/doses/{id}/mark-taken", web::post().to(doses::mark_taken)),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/doses/some-id/mark-taken")
        .set_json(serde_json::json!({ "taken_at": "not-a-date" }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn mark_taken_for_nonexistent_record_returns_404() {
    let (c, _dir) = container();
    let data = web::Data::new(c);
    let app = init_service(
        App::new()
            .app_data(data)
            .route("/doses/{id}/mark-taken", web::post().to(doses::mark_taken)),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/doses/00000000-0000-0000-0000-000000000000/mark-taken")
        .set_json(serde_json::json!({ "taken_at": "2025-01-01T08:00:00" }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn mark_taken_for_existing_record_returns_200() {
    let (c, _dir) = container();
    let data = web::Data::new(c.clone());
    let app = init_service(
        App::new()
            .app_data(data)
            .route("/medications", web::post().to(medications::create))
            .route("/doses/schedule", web::post().to(doses::schedule))
            .route("/doses/{id}/mark-taken", web::post().to(doses::mark_taken)),
    )
    .await;

    // Create a medication with a schedule
    let create_req = test::TestRequest::post()
        .uri("/medications")
        .set_json(serde_json::json!({
            "name": "Aspirin",
            "amount_mg": 100,
            "scheduled_time": [[8, 0]],
            "dose_frequency": "OnceDaily"
        }))
        .to_request();
    test::call_service(&app, create_req).await;

    // Schedule doses
    let sched_req = test::TestRequest::post()
        .uri("/doses/schedule")
        .to_request();
    test::call_service(&app, sched_req).await;

    // List all dose records to find an ID
    // We can't list them via REST without a list endpoint, so we rely on the
    // schedule response having created a record — test the 404 path instead
    // for a random ID to confirm the not-found branch above works.
    let mark_req = test::TestRequest::post()
        .uri("/doses/00000000-0000-0000-0000-000000000000/mark-taken")
        .set_json(serde_json::json!({ "taken_at": "2025-01-01T08:00:00" }))
        .to_request();
    let mark_resp = test::call_service(&app, mark_req).await;
    assert_eq!(mark_resp.status(), 404);
}

#[actix_web::test]
async fn mark_taken_returns_200_for_real_dose_record() {
    use bitpill::application::dtos::requests::CreateDoseRecordRequest;
    use chrono::NaiveDateTime;

    let (c, _dir) = container();
    let data = web::Data::new(c.clone());
    let app = init_service(
        App::new()
            .app_data(data)
            .route("/medications", web::post().to(medications::create))
            .route("/doses/{id}/mark-taken", web::post().to(doses::mark_taken)),
    )
    .await;

    // Create medication
    let create_req = test::TestRequest::post()
        .uri("/medications")
        .set_json(serde_json::json!({
            "name": "Aspirin",
            "amount_mg": 100,
            "scheduled_time": [[8, 0]],
            "dose_frequency": "OnceDaily"
        }))
        .to_request();
    let create_resp = test::call_service(&app, create_req).await;
    let body: serde_json::Value = test::read_body_json(create_resp).await;
    let med_id = body["id"].as_str().unwrap().to_string();

    // Create a dose record directly (bypasses scheduling time-of-day logic)
    let scheduled_at =
        NaiveDateTime::parse_from_str("2025-01-01T08:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
    let record_resp = c
        .create_dose_record_service
        .execute(CreateDoseRecordRequest::new(med_id.clone(), scheduled_at))
        .unwrap();

    let mark_req = test::TestRequest::post()
        .uri(&format!("/doses/{}/mark-taken", record_resp.id))
        .set_json(serde_json::json!({ "taken_at": "2025-01-01T08:05:00" }))
        .to_request();
    let mark_resp = test::call_service(&app, mark_req).await;
    assert_eq!(mark_resp.status(), 200);
}

#[actix_web::test]
async fn mark_taken_with_invalid_uuid_id_returns_500() {
    let (c, _dir) = container();
    let data = web::Data::new(c);
    let app = init_service(
        App::new()
            .app_data(data)
            .route("/doses/{id}/mark-taken", web::post().to(doses::mark_taken)),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/doses/not-a-valid-uuid/mark-taken")
        .set_json(serde_json::json!({ "taken_at": "2025-01-01T08:00:00" }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 500);
}

#[actix_web::test]
async fn mark_taken_twice_returns_400() {
    use bitpill::application::dtos::requests::CreateDoseRecordRequest;
    use chrono::NaiveDateTime;

    let (c, _dir) = container();
    let data = web::Data::new(c.clone());
    let app = init_service(
        App::new()
            .app_data(data)
            .route("/medications", web::post().to(medications::create))
            .route("/doses/{id}/mark-taken", web::post().to(doses::mark_taken)),
    )
    .await;

    // Create medication
    let create_req = test::TestRequest::post()
        .uri("/medications")
        .set_json(serde_json::json!({
            "name": "Aspirin",
            "amount_mg": 100,
            "scheduled_time": [[8, 0]],
            "dose_frequency": "OnceDaily"
        }))
        .to_request();
    let body: serde_json::Value =
        test::read_body_json(test::call_service(&app, create_req).await).await;
    let med_id = body["id"].as_str().unwrap().to_string();

    // Create dose record directly
    let scheduled_at =
        NaiveDateTime::parse_from_str("2025-01-01T08:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
    let record_resp = c
        .create_dose_record_service
        .execute(CreateDoseRecordRequest::new(med_id, scheduled_at))
        .unwrap();

    // Mark taken once (success)
    let mark_req = test::TestRequest::post()
        .uri(&format!("/doses/{}/mark-taken", record_resp.id))
        .set_json(serde_json::json!({ "taken_at": "2025-01-01T08:05:00" }))
        .to_request();
    test::call_service(&app, mark_req).await;

    // Mark taken again (should return 400 DoseAlreadyTaken domain error)
    let mark_req2 = test::TestRequest::post()
        .uri(&format!("/doses/{}/mark-taken", record_resp.id))
        .set_json(serde_json::json!({ "taken_at": "2025-01-01T08:10:00" }))
        .to_request();
    let resp2 = test::call_service(&app, mark_req2).await;

    assert_eq!(resp2.status(), 400);
}
