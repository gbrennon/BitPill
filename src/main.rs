use std::sync::Arc;

use bitpill::{
    infrastructure::{config::app_paths::AppPaths, container::Container},
    presentation::root::PresentationRoot,
};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let paths = AppPaths::resolve();
    let container = Arc::new(Container::new(
        paths.medications_path().clone(),
        paths.dose_records_path().clone(),
        paths.settings_path().clone(),
    ));
    let _presentation = PresentationRoot::new(container.clone());
    let mode = bitpill::runner::parse_mode(&mut std::env::args());
    bitpill::runner::run_app(&mode, container)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // If clause to avoid spawning TUI in tests
    if cfg!(test) {
        return Ok(());
    }
    run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_returns_ok() {
        // This test will call run(), but since main() returns early in test mode, it should be safe.
        assert!(run().is_ok());
    }
}
