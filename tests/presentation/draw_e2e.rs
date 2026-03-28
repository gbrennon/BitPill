use bitpill::infrastructure::container::Container;
use bitpill::presentation::tui::app::App;
use bitpill::presentation::tui::app_services::AppServices;
use bitpill::presentation::tui::draw;
use ratatui::Terminal;
use ratatui::backend::TestBackend;
use tempfile::tempdir;

#[test]
fn draw_renders_home_screen_without_panic() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let dir = tempdir().unwrap();
    let meds = dir.path().join("meds.json");
    let doses = dir.path().join("doses.json");
    let settings = dir.path().join("settings.json");
    let container = Container::new(meds, doses, settings);

    let app = App::new(AppServices::from_container(&container));
    terminal
        .draw(|f| draw::draw(f, &app))
        .expect("draw should not panic");
    let backend = terminal.backend_mut();
    let buffer = backend.buffer().clone();
    let any = buffer.content.iter().any(|cell| cell.symbol() != " ");
    assert!(any, "expected some rendered content");
}
