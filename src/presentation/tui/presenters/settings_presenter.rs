use ratatui::{
    Frame,
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::presentation::tui::{styles::content_style, templates::screen_template::ScreenTemplate};

pub struct SettingsPresenter;

impl SettingsPresenter {
    pub fn present(&self, f: &mut Frame, vim_enabled: bool) {
        let help = "[Space] Toggle  [s] Save  [Esc] Cancel";
        ScreenTemplate {
            subtitle: "Settings",
            help,
            mode: "NORMAL",
        }
        .render(f, |f, area| {
            let checked = if vim_enabled { "[x]" } else { "[ ]" };
            let lines = vec![Line::from(Span::styled(
                format!("{} Vim navigation", checked),
                content_style(),
            ))];
            let p = Paragraph::new(lines).block(
                Block::default()
                    .title("Preferences")
                    .borders(ratatui::widgets::Borders::ALL)
                    .style(content_style()),
            );
            f.render_widget(p, area);
        });
    }
}
