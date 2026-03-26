use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::Terminal;

use std::sync::Arc;
use tempfile::tempdir;

use bitpill::application::dtos::requests::CreateMedicationRequest;
use bitpill::infrastructure::container::Container;
use bitpill::presentation::tui::app::App;
use bitpill::presentation::tui::draw;

#[test]
fn render_all_screens_does_not_panic_and_draws_something() {
    // Use a test-specific container to avoid touching real files
    let dir = tempdir().unwrap();
    let meds = dir.path().join("meds.json");
    let doses = dir.path().join("doses.json");
    let settings = dir.path().join("settings.json");
    let container = Arc::new(Container::new(meds, doses, settings));

    // create a medication so lists/details have data
    {
        let create_req = CreateMedicationRequest::new("Aspirin", 100, vec![(8, 0)], "OnceDaily");
        container
            .create_medication_service
            .execute(create_req)
            .expect("create should succeed");
    }

    let mut app =
        App::new(bitpill::presentation::tui::app_services::AppServices::from_container(&container));

    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    use bitpill::presentation::tui::screen::Screen;

    let screens = vec![
        Screen::HomeScreen,
        Screen::CreateMedication {
            name: "".into(),
            amount_mg: "".into(),
            selected_frequency: 0,
            scheduled_time: vec![],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: false,
        },
        Screen::EditMedication {
            id: "1".into(),
            name: "".into(),
            amount_mg: "".into(),
            selected_frequency: 0,
            scheduled_time: vec![],
            scheduled_idx: 0,
            focused_field: 0,
            insert_mode: false,
        },
        Screen::MedicationDetails {
            id: app.medications.first().unwrap().id.clone(),
        },
        Screen::MarkDose {
            medication_id: app.medications.first().unwrap().id.clone(),
            records: vec![],
            selected_index: 0,
        },
        Screen::ConfirmDelete {
            id: app.medications.first().unwrap().id.clone(),
            name: app.medications.first().unwrap().name.clone(),
        },
        Screen::ConfirmCancel {
            previous: Box::new(Screen::HomeScreen),
        },
        Screen::Settings { vim_enabled: false },
        Screen::ConfirmQuit {
            previous: Box::new(Screen::HomeScreen),
        },
        Screen::ValidationError {
            message: "err".into(),
            previous: Box::new(Screen::HomeScreen),
        },
    ];

    for s in screens {
        app.current_screen = s.clone();
        terminal
            .draw(|f| draw::draw(f, &app))
            .expect("draw should succeed");
        let backend = terminal.backend_mut();
        let buffer: Buffer = backend.buffer().clone();
        let any = buffer.content.iter().any(|cell| cell.symbol() != " ");
        assert!(any, "expected some rendered content for screen");
    }
}
