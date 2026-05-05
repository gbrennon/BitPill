use std::sync::Arc;

use bitpill::{
    infrastructure::container::Container,
    presentation::{root::PresentationRoot, tui::screen::Screen},
};
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
