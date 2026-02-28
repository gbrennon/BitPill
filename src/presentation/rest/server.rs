use std::future::Future;
use std::sync::Arc;

use actix_web::{web, App, HttpServer};

use crate::infrastructure::container::Container;
use crate::presentation::rest::handlers::{doses, medications};

pub struct RestServer;

impl RestServer {
    pub fn run(container: Arc<Container>) -> impl Future<Output = std::io::Result<()>> {
        let data = web::Data::new(container);

        HttpServer::new(move || {
            App::new()
                .app_data(data.clone())
                .route("/medications", web::get().to(medications::list_all))
                .route("/medications", web::post().to(medications::create))
                .route("/doses/schedule", web::post().to(doses::schedule))
                .route(
                    "/doses/{id}/mark-taken",
                    web::post().to(doses::mark_taken),
                )
        })
        .bind("0.0.0.0:8080")
        .expect("failed to bind to 0.0.0.0:8080")
        .run()
    }
}
