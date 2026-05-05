use bitpill::{
    application::dtos::responses::MedicationDto,
    presentation::tui::{
        renderers::{ScreenRenderer, medication_details_renderer::MedicationDetailsRenderer},
        screen::Screen,
    },
};
use ratatui::{Terminal, backend::TestBackend};

use crate::helpers::make_app;

#[test]
fn render_does_not_panic() {
    let mut t = Terminal::new(TestBackend::new(80, 24)).unwrap();
    let mut app = make_app(Screen::MedicationDetails { id: "m1".into() });
    app.medications = vec![MedicationDto {
        id: "m1".into(),
        name: "Aspirin".into(),
        amount_mg: 100,
        scheduled_time: vec![(8, 0)],
        dose_frequency: "OnceDaily".into(),
        taken_today: 0,
        scheduled_today: 1,
    }];
    t.draw(|f| MedicationDetailsRenderer.render(f, &app))
        .unwrap();
}
