use bitpill::{
    infrastructure::container::Container,
    presentation::tui::{
        app::App,
        app_services::AppServices,
        handlers::{
            port::{Handler, HandlerResult},
            schedule_result_handler::ScheduleResultHandler,
        },
        input::Key,
        screen::Screen,
    },
};
use tempfile::tempdir;

#[test]
fn default_constructs() {
    let _handler = ScheduleResultHandler::default();
}

#[test]
fn handle_returns_to_home_and_reloads() {
    let mut handler = ScheduleResultHandler::default();
    let dir = tempdir().unwrap();
    let meds = dir.path().join("meds.json");
    let doses = dir.path().join("doses.json");
    let settings = dir.path().join("settings.json");

    std::fs::write(&meds, "[]").unwrap();
    std::fs::write(&doses, "[]").unwrap();
    std::fs::write(&settings, r#"{"vim_enabled":false}"#).unwrap();

    let container = Container::new(meds, doses, settings);
    let mut app = App::new(AppServices::from_container(&container));
    app.current_screen = Screen::ConfirmQuit {
        previous: Box::new(Screen::HomeScreen),
    };

    let result = handler.handle(&mut app, Key::Enter);
    assert!(matches!(result, HandlerResult::Continue));
    assert!(matches!(app.current_screen, Screen::HomeScreen));
}
