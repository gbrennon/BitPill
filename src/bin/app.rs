/// Starts both the REST server (on a background thread) and the TUI (on the
/// main thread). The REST server is shut down automatically when the TUI exits.
use std::sync::Arc;

use bitpill::infrastructure::container::Container;
use bitpill::presentation::rest::server::RestServer;
use bitpill::presentation::tui::app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let container = Arc::new(Container::new());

    let rest_container = container.clone();
    std::thread::spawn(move || {
        tokio::runtime::Runtime::new()
            .expect("failed to create tokio runtime")
            .block_on(RestServer::run(rest_container))
            .expect("REST server error");
    });

    App::run(container)
}
