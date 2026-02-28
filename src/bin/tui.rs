use std::sync::Arc;

use bitpill::infrastructure::container::Container;
use bitpill::presentation::tui::app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let container = Arc::new(Container::new());
    App::run(container)
}
