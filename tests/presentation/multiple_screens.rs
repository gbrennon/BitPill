use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use std::sync::Arc;
use tempfile::tempdir;

use bitpill::application::dtos::responses::MedicationDto;
use bitpill::infrastructure::container::Container;
use bitpill::presentation::tui::app::App;
use bitpill::presentation::tui::draw;
use bitpill::presentation::tui::screen::Screen;

fn draw_with_app(app: &App) -> Buffer {
    let backend = TestBackend::new(80, 24);
    let mut term = Terminal::new(backend).unwrap();
    term.draw(|f| draw::draw(f, app)).expect("draw failed");
    term.backend().buffer().clone()
}

#[test]
fn render_create_and_edit_and_settings_and_confirm_modals() {
    let dir = tempdir().unwrap();
    let meds = dir.path().join("meds.json");
    let doses = dir.path().join("doses.json");
    let settings = dir.path().join("settings.json");
    let container = Arc::new(Container::new_with_paths(meds, doses, settings));
    let mut app =
        App::new(bitpill::presentation::tui::app_services::AppServices::from_container(&container));

    // CreateMedication
    app.current_screen = Screen::CreateMedication {
        name: "".into(),
        amount_mg: "".into(),
        selected_frequency: 0,
        scheduled_time: vec!["08:00".into()],
        scheduled_idx: 0,
        focused_field: 0,
        insert_mode: false,
    };
    let buf = draw_with_app(&app);
    assert!(buf.content.iter().any(|cell| cell.symbol() != " "));

    // EditMedication
    app.current_screen = Screen::EditMedication {
        id: "id".into(),
        name: "A".into(),
        amount_mg: "100".into(),
        selected_frequency: 1,
        scheduled_time: vec!["08:00".into(), "20:00".into()],
        scheduled_idx: 0,
        focused_field: 0,
        insert_mode: false,
    };
    let buf = draw_with_app(&app);
    assert!(buf.content.iter().any(|cell| cell.symbol() != " "));

    // Settings
    app.current_screen = Screen::Settings { vim_enabled: true };
    let buf = draw_with_app(&app);
    assert!(buf.content.iter().any(|cell| cell.symbol() != " "));

    // ConfirmQuit overlay
    app.current_screen = Screen::ConfirmQuit {
        previous: Box::new(Screen::HomeScreen),
    };
    let buf = draw_with_app(&app);
    assert!(buf.content.iter().any(|cell| cell.symbol() != " "));
}

#[test]
fn render_medication_details_and_mark_dose_and_validation_error() {
    let dir = tempdir().unwrap();
    let meds = dir.path().join("meds.json");
    let doses = dir.path().join("doses.json");
    let settings = dir.path().join("settings.json");
    let container = Arc::new(Container::new_with_paths(
        meds.clone(),
        doses.clone(),
        settings,
    ));
    let mut app =
        App::new(bitpill::presentation::tui::app_services::AppServices::from_container(&container));

    // Add a medication dto to app so details renderer finds it
    let med = MedicationDto {
        id: "med1".to_string(),
        name: "TestMed".to_string(),
        amount_mg: 10,
        scheduled_time: vec![(8, 0)],
        dose_frequency: "OnceDaily".to_string(),
    };
    app.medications.push(med);

    app.current_screen = Screen::MedicationDetails { id: "med1".into() };
    let buf = draw_with_app(&app);
    assert!(buf.content.iter().any(|cell| cell.symbol() != " "));

    // MarkDose screen requires DoseRecordDto; leave records empty works
    app.current_screen = Screen::MarkDose {
        medication_id: "med1".into(),
        records: vec![],
        selected_index: 0,
    };
    let buf = draw_with_app(&app);
    assert!(buf.content.iter().any(|cell| cell.symbol() != " "));

    // ValidationError overlay
    app.current_screen = Screen::ValidationError {
        message: "Bad field".into(),
        previous: Box::new(Screen::HomeScreen),
    };
    let buf = draw_with_app(&app);
    assert!(buf.content.iter().any(|cell| cell.symbol() != " "));
}
