// Composition root for the presentation layer
use std::sync::Arc;

use crate::{
    infrastructure::container::Container,
    presentation::tui::{app::App, app_services::AppServices},
};

pub struct PresentationRoot {
    pub tui_app: App,
}

impl PresentationRoot {
    pub fn new(container: Arc<Container>) -> Self {
        let services = AppServices::from_container(&container);
        let tui_app = App::new(services);
        Self { tui_app }
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;
    use crate::presentation::tui::screen::Screen;

    #[test]
    fn new_creates_presentation_root_with_home_screen() {
        let dir = tempdir().unwrap();
        let container = Arc::new(Container::new(
            dir.path().join("meds.json"),
            dir.path().join("doses.json"),
            dir.path().join("settings.json"),
        ));
        let root = PresentationRoot::new(container);
        assert!(matches!(root.tui_app.current_screen, Screen::HomeScreen));
    }
}
