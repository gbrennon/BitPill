use std::sync::Arc;

use bitpill::infrastructure::container::Container;
use bitpill::presentation::rest::server::RestServer;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let container = Arc::new(Container::new());
    RestServer::run(container).await
}
