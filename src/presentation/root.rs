// Composition root for the presentation layer
use crate::infrastructure::container::Container;
use crate::presentation::rest::server::RestServer;
use crate::presentation::tui::app::App;
use std::sync::Arc;

pub struct PresentationRoot {
    pub tui_app: App,
    pub rest_server: RestServer,
}

impl PresentationRoot {
    pub fn new(container: Arc<Container>) -> Self {
        let tui_app = App::new(container.clone());
        let rest_server = RestServer;
        Self {
            tui_app,
            rest_server,
        }
    }
}
