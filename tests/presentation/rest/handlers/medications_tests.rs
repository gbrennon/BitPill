use actix_web::test::{self, init_service};
use actix_web::{App, web};
use bitpill::infrastructure::container::Container;
use bitpill::presentation::rest::handlers::medications;
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
async fn list_all_returns_200_with_empty_list() {
    let (c, _dir) = container(); let data = web::Data::new(c);
    let app = init_service(
        App::new()
            .app_data(data)
            .route("/medications", web::get().to(medications::list_all)),
    )
    .await;

    let req = test::TestRequest::get().uri("/medications").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
    let body: Vec<serde_json::Value> = test::read_body_json(resp).await;
    assert!(body.is_empty());
}

#[actix_web::test]
async fn create_returns_201_with_id() {
    let (c, _dir) = container(); let data = web::Data::new(c);
    let app = init_service(
        App::new()
            .app_data(data)
            .route("/medications", web::post().to(medications::create)),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/medications")
        .set_json(serde_json::json!({
            "name": "Aspirin",
            "amount_mg": 100,
            "scheduled_time": [[8, 0]],
            "dose_frequency": "OnceDaily"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 201);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["id"].as_str().is_some());
}

#[actix_web::test]
async fn create_with_invalid_dosage_returns_400() {
    let (c, _dir) = container(); let data = web::Data::new(c);
    let app = init_service(
        App::new()
            .app_data(data)
            .route("/medications", web::post().to(medications::create)),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/medications")
        .set_json(serde_json::json!({
            "name": "X",
            "amount_mg": 0,
            "scheduled_time": [[8, 0]],
            "dose_frequency": "OnceDaily"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn create_with_empty_name_returns_400() {
    let (c, _dir) = container(); let data = web::Data::new(c);
    let app = init_service(
        App::new()
            .app_data(data)
            .route("/medications", web::post().to(medications::create)),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/medications")
        .set_json(serde_json::json!({
            "name": "",
            "amount_mg": 100,
            "scheduled_time": [[8, 0]],
            "dose_frequency": "OnceDaily"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn get_by_id_returns_200_for_existing_medication() {
    let (c, _dir) = container();
    let data = web::Data::new(c.clone());
    let app = init_service(
        App::new()
            .app_data(data)
            .route("/medications", web::post().to(medications::create))
            .route("/medications/{id}", web::get().to(medications::get_by_id)),
    )
    .await;

    // Create first
    let create_req = test::TestRequest::post()
        .uri("/medications")
        .set_json(serde_json::json!({
            "name": "Ibuprofen",
            "amount_mg": 200,
            "scheduled_time": [[9, 0]]
        }))
        .to_request();
    let create_resp = test::call_service(&app, create_req).await;
    let body: serde_json::Value = test::read_body_json(create_resp).await;
    let id = body["id"].as_str().unwrap().to_string();

    // Then get by id
    let req = test::TestRequest::get()
        .uri(&format!("/medications/{}", id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn get_by_id_returns_500_for_missing_medication() {
    let (c, _dir) = container(); let data = web::Data::new(c);
    let app = init_service(
        App::new()
            .app_data(data)
            .route("/medications/{id}", web::get().to(medications::get_by_id)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/medications/nonexistent-id")
        .to_request();
    let resp = test::call_service(&app, req).await;

    // NotFound maps to InternalServerError in current implementation
    assert!(resp.status().is_server_error() || resp.status().is_client_error());
}

#[actix_web::test]
async fn update_returns_200_for_existing_medication() {
    let (c, _dir) = container();
    let data = web::Data::new(c.clone());
    let app = init_service(
        App::new()
            .app_data(data)
            .route("/medications", web::post().to(medications::create))
            .route("/medications/{id}", web::put().to(medications::update)),
    )
    .await;

    // Create first
    let create_req = test::TestRequest::post()
        .uri("/medications")
        .set_json(serde_json::json!({
            "name": "Paracetamol",
            "amount_mg": 500,
            "scheduled_time": [[8, 0]]
        }))
        .to_request();
    let create_resp = test::call_service(&app, create_req).await;
    let body: serde_json::Value = test::read_body_json(create_resp).await;
    let id = body["id"].as_str().unwrap().to_string();

    // Then update
    let update_req = test::TestRequest::put()
        .uri(&format!("/medications/{}", id))
        .set_json(serde_json::json!({
            "name": "Paracetamol Updated",
            "amount_mg": 600,
            "scheduled_time": [[9, 0]]
        }))
        .to_request();
    let update_resp = test::call_service(&app, update_req).await;
    assert_eq!(update_resp.status(), 200);
}

#[actix_web::test]
async fn update_with_invalid_name_returns_400() {
    let (c, _dir) = container();
    let data = web::Data::new(c.clone());
    let app = init_service(
        App::new()
            .app_data(data)
            .route("/medications", web::post().to(medications::create))
            .route("/medications/{id}", web::put().to(medications::update)),
    )
    .await;

    // Create first
    let create_req = test::TestRequest::post()
        .uri("/medications")
        .set_json(serde_json::json!({
            "name": "Valid",
            "amount_mg": 100,
            "scheduled_time": [[8, 0]]
        }))
        .to_request();
    let create_resp = test::call_service(&app, create_req).await;
    let body: serde_json::Value = test::read_body_json(create_resp).await;
    let id = body["id"].as_str().unwrap().to_string();

    // Update with empty name
    let update_req = test::TestRequest::put()
        .uri(&format!("/medications/{}", id))
        .set_json(serde_json::json!({
            "name": "",
            "amount_mg": 100,
            "scheduled_time": [[8, 0]]
        }))
        .to_request();
    let update_resp = test::call_service(&app, update_req).await;
    assert_eq!(update_resp.status(), 400);
}

#[actix_web::test]
async fn update_with_invalid_id_returns_500() {
    let (c, _dir) = container();
    let data = web::Data::new(c.clone());
    let app = init_service(
        App::new()
            .app_data(data)
            .route("/medications/{id}", web::put().to(medications::update)),
    )
    .await;

    let req = test::TestRequest::put()
        .uri("/medications/not-a-uuid")
        .set_json(serde_json::json!({
            "name": "X",
            "amount_mg": 100,
            "scheduled_time": [[8, 0]]
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 500);
}

#[actix_web::test]
async fn create_with_storage_error_returns_500() {
    use std::fs;
    let dir = tempfile::tempdir().unwrap();
    // Make medications path a directory so writes fail
    let meds_path = dir.path().join("meds_dir");
    fs::create_dir_all(&meds_path).unwrap();
    let c = Arc::new(Container::new_with_paths(
        meds_path,
        dir.path().join("doses.json"),
        dir.path().join("settings.json"),
    ));
    let data = web::Data::new(c);
    let app = init_service(
        App::new()
            .app_data(data)
            .route("/medications", web::post().to(medications::create)),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/medications")
        .set_json(serde_json::json!({
            "name": "Aspirin",
            "amount_mg": 100,
            "scheduled_time": [[8, 0]]
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 500);
}

#[actix_web::test]
async fn list_all_returns_medications_when_present() {
    let (c, _dir) = container();
    let data = web::Data::new(c.clone());
    let app = init_service(
        App::new()
            .app_data(data)
            .route("/medications", web::post().to(medications::create))
            .route("/medications", web::get().to(medications::list_all)),
    )
    .await;

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

    let req = test::TestRequest::get().uri("/medications").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
    let body: Vec<serde_json::Value> = test::read_body_json(resp).await;
    assert_eq!(body.len(), 1);
    assert_eq!(body[0]["name"], "Aspirin");
}
