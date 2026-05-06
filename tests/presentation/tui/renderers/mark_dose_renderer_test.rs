use bitpill::{
    application::dtos::responses::DoseRecordDto,
    presentation::tui::{
        renderers::{ScreenRenderer, mark_dose_renderer::MarkDoseRenderer},
        screen::Screen,
    },
};
use ratatui::{Terminal, backend::TestBackend};

use crate::helpers::make_app;

#[test]
fn render_does_not_panic() {
    let mut t = Terminal::new(TestBackend::new(80, 24)).unwrap();
    let app = make_app(Screen::MarkDose {
        medication_id: "m1".into(),
        records: vec![DoseRecordDto {
            id: "r1".into(),
            medication_id: "m1".into(),
            scheduled_at: chrono::NaiveDate::from_ymd_opt(2025, 1, 1)
                .unwrap()
                .and_hms_opt(8, 0, 0)
                .unwrap(),
            taken_at: None,
        }],
        selected_index: 0,
    });
    t.draw(|f| MarkDoseRenderer.render(f, &app)).unwrap();
}
