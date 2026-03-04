use std::future::Future;
use std::sync::Arc;

use actix_web::{App, HttpServer, web};

use crate::infrastructure::container::Container;
use crate::presentation::rest::handlers::{doses, medications};

pub struct RestServer;

impl RestServer {
    /// Run the REST server on the default address 0.0.0.0:8080
    pub fn run(container: Arc<Container>) -> impl Future<Output = std::io::Result<()>> {
        Self::run_with_addr(container, "0.0.0.0:8080")
    }

    /// Run the REST server bound to the provided address (e.g. "127.0.0.1:8080").
    /// Returns a Future that resolves when the server exits or an IO error occurs while binding.
    pub fn run_with_addr(
        container: Arc<Container>,
        addr: &str,
    ) -> impl Future<Output = std::io::Result<()>> {
        let data = web::Data::new(container);
        let addr = addr.to_string();

        async move {
            let server = HttpServer::new(move || {
                App::new()
                    .app_data(data.clone())
                    .route("/medications", web::get().to(medications::list_all))
                    .route("/medications", web::post().to(medications::create))
                    .route("/medications/{id}", web::get().to(medications::get_by_id))
                    .route("/medications/{id}", web::put().to(medications::update))
                    .route("/doses/schedule", web::post().to(doses::schedule))
                    .route("/doses/{id}/mark-taken", web::post().to(doses::mark_taken))
            })
            .bind(addr)?;

            server.run().await
        }
    }
}
