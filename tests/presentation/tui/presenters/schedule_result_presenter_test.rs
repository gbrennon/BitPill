use bitpill::presentation::tui::presenters::schedule_result_presenter::{
    ScheduleResultInput, ScheduleResultPresenter,
};
use ratatui::{Terminal, backend::TestBackend};

#[test]
fn present_renders_scheduled_count() {
    let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
    let input = ScheduleResultInput { created_count: 3 };
    terminal
        .draw(|f| ScheduleResultPresenter.present(f, &input))
        .unwrap();
    let content: String = terminal
        .backend()
        .buffer()
        .content
        .iter()
        .map(|c| c.symbol())
        .collect();
    assert!(content.contains('3'));
}

#[test]
fn present_with_zero_count_does_not_panic() {
    let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
    let input = ScheduleResultInput { created_count: 0 };
    terminal
        .draw(|f| ScheduleResultPresenter.present(f, &input))
        .unwrap();
}
