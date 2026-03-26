// Composition root for the presentation layer
use crate::infrastructure::container::Container;
use crate::presentation::rest::server::RestServer;
use crate::presentation::tui::app::App;
use crate::presentation::tui::app_services::AppServices;
use std::sync::Arc;

pub struct PresentationRoot {
    pub tui_app: App,
    pub rest_server: RestServer,
}

impl PresentationRoot {
    pub fn new(container: Arc<Container>) -> Self {
        let services = AppServices::from_container(&container);
        let tui_app = App::new(services);
        let rest_server = RestServer;
        Self {
            tui_app,
            rest_server,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::presentation::tui::screen::Screen;
    use tempfile::tempdir;

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
