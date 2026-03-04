use crate::presentation::tui::styles::{TOP_BAR_HEIGHT, bar_style, content_style, BORDER_COLOR, COPY_COLOR};

// External crates
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::text::{Line, Span};

pub struct CreateMedicationForm;

impl CreateMedicationForm {
    /// Render the CreateMedicationForm component from supplied data (keeps presenter free of App state).
    /// This is used by both Create and Edit screens, so it doesn't include the title bar or help/status line (which are handled by the presenter).
    /// Focused field is used to apply different styles to the currently focused field, and insert_mode is used to optionally show an indicator in the title bar (handled by presenter).
    pub fn present_with_data(&self, f: &mut Frame, name: &str, amount_mg: &str, scheduled_time: &str, focused_field: u8, insert_mode: bool) {
        let fields = [
            ("Name", name, 0u8),
            ("Amount (mg)", amount_mg, 1u8),
            ("Scheduled times (HH:MM,...)", scheduled_time, 2u8),
        ];

        for (i, (label, value, field_idx)) in fields.iter().enumerate() {
            let is_focused = *field_idx == focused_field;
            let title_style = if is_focused {
                content_style().fg(BORDER_COLOR).add_modifier(ratatui::style::Modifier::BOLD)
            } else {
                content_style().fg(COPY_COLOR)
            };
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(content_style().fg(BORDER_COLOR))
                .style(content_style())
                .title(Span::styled(*label, title_style));
            let paragraph = Paragraph::new(Line::from(Span::styled(*value, content_style()))).block(block);
            f.render_widget(paragraph, chunks[i + 1]);
        }

        if insert_mode {
            // The form itself doesn't render the insert mode indicator, but it can still indicate insert mode by changing the field styles (e.g. making them bold) to provide a visual cue that we're in insert mode.
            // The presenter can handle any additional visual indicators in the title bar area if desired, but the focused field styling should be sufficient to indicate when we're in insert mode.
        }

    }
}
