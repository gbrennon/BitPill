use std::env;
use std::sync::Arc;

use bitpill::infrastructure::container::Container;
use bitpill::presentation::root::PresentationRoot;

/// Application entrypoint: delegates presentation to PresentationRoot which
/// composes the TUI and REST entrypoints. The REST server can be run in the
/// background (default) or the app can run in tui-only or api-only modes via
/// a positional argument: `tui`, `api`, or `both` (default).
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mode = env::args().nth(1).unwrap_or_else(|| "both".to_string());

    let container = Arc::new(Container::new());
    let _presentation = PresentationRoot::new(container.clone());

    match mode.as_str() {
        "tui" => {
            // Run only the TUI in foreground
            bitpill::presentation::tui::app::App::run(container)
        }
        "api" => {
            // Run only the REST server on a dedicated tokio runtime (blocking)
            tokio::runtime::Runtime::new()
                .expect("failed to create tokio runtime")
                .block_on(async move {
                    bitpill::presentation::rest::RestServer::run_with_addr(
                        container,
                        "0.0.0.0:8080",
                    )
                    .await
                    .expect("REST server error");
                });
            Ok(())
        }
        _ => {
            // Default: run both; REST server in background thread and TUI in foreground
            let rest_container = container.clone();
            std::thread::spawn(move || {
                tokio::runtime::Runtime::new()
                    .expect("failed to create tokio runtime")
                    .block_on(async move {
                        bitpill::presentation::rest::RestServer::run_with_addr(
                            rest_container,
                            "0.0.0.0:8080",
                        )
                        .await
                        .expect("REST server error");
                    });
            });

            bitpill::presentation::tui::app::App::run(container)
        }
    }
}
