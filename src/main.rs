use std::sync::Arc;

use bitpill::infrastructure::config::app_paths::AppPaths;
use bitpill::infrastructure::container::Container;
use bitpill::presentation::root::PresentationRoot;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
