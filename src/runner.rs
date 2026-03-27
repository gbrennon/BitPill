use std::sync::Arc;

use crate::infrastructure::container::Container;

pub fn parse_mode(args: &mut impl Iterator<Item = String>) -> String {
    args.nth(1).unwrap_or_else(|| "tui".to_string())
}

pub fn run_app(mode: &str, container: Arc<Container>) -> Result<(), Box<dyn std::error::Error>> {
    match mode {
        "api" => start_api(container),
        _ => start_tui(container),
    }
}

#[cfg(not(test))]
fn start_tui(container: Arc<Container>) -> Result<(), Box<dyn std::error::Error>> {
    crate::presentation::tui::app::App::run(container)
}

#[cfg(test)]
fn start_tui(_: Arc<Container>) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[cfg(all(not(test), feature = "rest-api"))]
fn start_api(container: Arc<Container>) -> Result<(), Box<dyn std::error::Error>> {
    tokio::runtime::Runtime::new()
        .expect("failed to create tokio runtime")
        .block_on(async move {
            crate::presentation::rest::RestServer::run_with_addr(container, "0.0.0.0:8080")
                .await
                .expect("REST server error");
        });
    Ok(())
}

#[cfg(all(not(test), not(feature = "rest-api")))]
fn start_api(_: Arc<Container>) -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("REST API not enabled. Compile with --features rest-api to enable.");
    Ok(())
}

#[cfg(test)]
fn start_api(_: Arc<Container>) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use tempfile::tempdir;

    use crate::infrastructure::container::Container;

    use super::*;

    fn make_container() -> (Arc<Container>, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let container = Arc::new(Container::new(
            dir.path().join("meds.json"),
            dir.path().join("doses.json"),
            dir.path().join("settings.json"),
        ));
        (container, dir)
    }

    #[test]
    fn parse_mode_returns_second_argument() {
        let args = vec!["binary".to_string(), "tui".to_string()];
        assert_eq!(parse_mode(&mut args.into_iter()), "tui");
    }

    #[test]
    fn parse_mode_defaults_to_tui_when_no_second_argument() {
        let args = vec!["binary".to_string()];
        assert_eq!(parse_mode(&mut args.into_iter()), "tui");
    }

    #[test]
    fn run_app_with_tui_mode_returns_ok() {
        let (container, _dir) = make_container();
        run_app("tui", container).unwrap();
    }

    #[test]
    fn run_app_with_api_mode_returns_ok_when_feature_disabled() {
        let (container, _dir) = make_container();
        let result = run_app("api", container);
        assert!(result.is_ok());
    }

    #[test]
    fn run_app_with_default_mode_returns_tui() {
        let (container, _dir) = make_container();
        run_app("default", container).unwrap();
    }

    #[test]
    fn run_app_with_unknown_mode_falls_through_to_tui() {
        let (container, _dir) = make_container();
        run_app("unknown", container).unwrap();
    }
}
