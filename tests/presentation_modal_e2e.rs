use bitpill::presentation::tui::components::modal::render_modal;
use ratatui::Terminal;
use ratatui::backend::TestBackend;

#[test]
fn modal_renders_e2e() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|f| {
        let area = f.area();
        render_modal(f, area, "My Modal", "Hello Modal\nSecond Line");
    }).unwrap();

    let buffer = terminal.backend().buffer();
    assert!(!buffer.content.is_empty());
    let contains_chars = buffer.content.iter().any(|cell| !cell.symbol().is_empty());
    assert!(contains_chars);
}
