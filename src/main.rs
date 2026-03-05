use std::sync::Arc;

use bitpill::infrastructure::container::Container;
use bitpill::presentation::root::PresentationRoot;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let container = Arc::new(Container::new());
    let _presentation = PresentationRoot::new(container.clone());
    let mode = bitpill::runner::parse_mode(&mut std::env::args());
    bitpill::runner::run_app(&mode, container)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_runs_without_error_in_test_environment() {
        main().unwrap();
    }
}
