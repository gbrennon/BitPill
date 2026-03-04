// Internal imports first
use crate::presentation::tui::templates::screen_template::ScreenTemplate;

// External crates
use ratatui::Frame;
use ratatui::widgets::Paragraph;

use crate::presentation::tui::styles::content_style;

pub struct ScheduleResultInput {
    pub created_count: usize,
}

pub struct ScheduleResultPresenter;

impl ScheduleResultPresenter {
    pub fn present(&self, f: &mut Frame, input: &ScheduleResultInput) {
        let msg = format!(
            "  {} dose(s) scheduled. Press any key to return.",
            input.created_count
        );

        ScreenTemplate {
            subtitle: "Schedule Result",
            help: " [any key] Back",
            mode: "NORMAL",
        }
        .render(f, |f, area| {
            f.render_widget(Paragraph::new(msg).style(content_style()), area);
        });
    }
}
