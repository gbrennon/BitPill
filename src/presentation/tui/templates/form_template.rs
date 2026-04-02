use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::presentation::tui::{
    components::{
        bottom_bar::bottom_bar, schedule_time::schedule_time, title_bar::render_title_bar,
    },
    styles::{BORDER_COLOR, COPY_COLOR, TOP_BAR_HEIGHT, content_style},
};

/// A single editable field rendered inside a [`FormTemplate`].
pub struct FormField<'a> {
    /// Label shown as the block title.
    pub label: &'a str,
    /// Current field value (raw string buffer). Empty when choices are provided.
    pub value: &'a str,
    /// Whether this field is currently focused.
    pub focused: bool,
    /// Optional radio choices to render for this field.
    pub choices: Option<&'a [&'a str]>,
    /// Optional selected index when `choices` is Some(...).
    pub selected_choice: Option<usize>,
    /// Number of terminal lines to render for this field.
    pub lines: usize,
    /// Optionally highlight a single line inside a multi-line field (by index)
    pub highlighted_line: Option<usize>,
    /// Optional backing slice of values for multi-slot inputs (e.g., scheduled_time)
    pub values: Option<&'a [String]>,
}

/// Form screen template — extends the base screen layout with typed input fields.
///
/// Renders a vertical stack of labelled input blocks between the title bar and
/// the help bar, mirroring the "form page extends base template" pattern from
/// web frameworks.
///
/// ```text
/// ┌─────────────────────────────────┐
/// │  Title bar                      │
/// ├─────────────────────────────────┤
/// │  ┌─ Field 1 ──────────────────┐ │  ← 3 lines each, focus-highlighted
/// │  └────────────────────────────┘ │
/// │  ┌─ Field 2 ──────────────────┐ │
/// │  └────────────────────────────┘ │
/// │  …                              │
/// │  (remaining space)              │
/// ├─────────────────────────────────┤
/// │  Help / status bar              │
/// └─────────────────────────────────┘
/// ```
///
/// # Example
/// ```ignore
/// FormTemplate {
///     subtitle: "Create Medication",
///     fields: &[
///         FormField { label: "Name", value: dto.name, focused: dto.focused_field == 0 },
///         FormField { label: "Amount (mg)", value: dto.amount_mg, focused: dto.focused_field == 1 },
///     ],
///     help: "[i] Insert  [Tab] Next  [Enter] Submit  [Esc] Cancel",
/// }
/// .render(f);
/// ```
pub struct FormTemplate<'a> {
    /// Screen-specific subtitle shown in the title bar.
    pub subtitle: &'a str,
    /// Ordered list of fields to render, top to bottom.
    pub fields: &'a [FormField<'a>],
    /// Text shown in the bottom bar.
    pub help: &'a str,
    /// Current input mode label (eg. "NORMAL" or "INSERT").
    pub mode: &'a str,
}

impl<'a> FormTemplate<'a> {
    /// Render the full form screen: title bar, all fields, and help bar.
    pub fn render(&self, f: &mut Frame) {
        // Build layout constraints dynamically: TopBar + (field.lines × fields) + Min(1) + Help
        let mut constraints = vec![Constraint::Length(TOP_BAR_HEIGHT)];
        for field in self.fields.iter() {
            // Each field renders inside a Block with borders that consume 2 rows; allocate
            // additional space so the inner content area can fit `field.lines` rows.
            constraints.push(Constraint::Length(field.lines as u16 + 2));
        }
        constraints.push(Constraint::Min(1));
        constraints.push(Constraint::Length(1));

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(f.area());

        render_title_bar(f, chunks[0], self.subtitle);

        for (i, field) in self.fields.iter().enumerate() {
            // All field titles should use the BORDER_COLOR; only the active field title is bold
            let label_style = content_style().fg(BORDER_COLOR);

            let mut block = Block::default()
                .borders(Borders::ALL)
                .border_style(content_style().fg(BORDER_COLOR))
                .style(content_style());
            // When focused, make borders and title bold
            if field.focused {
                block = block.title(Span::styled(
                    field.label,
                    label_style.add_modifier(Modifier::BOLD),
                ));
                block = block.border_style(
                    content_style()
                        .fg(BORDER_COLOR)
                        .add_modifier(Modifier::BOLD),
                );
            } else {
                block = block.title(Span::styled(field.label, label_style));
            }

            // All field values should use BORDER_COLOR; do not bold values (only title is bold)
            let value_text_style = content_style().fg(BORDER_COLOR);

            if let Some(choices) = field.choices {
                let selected = field.selected_choice.unwrap_or(0);
                let parts: Vec<String> = choices
                    .iter()
                    .enumerate()
                    .map(|(idx, c)| {
                        if idx == selected {
                            format!("(*) {}", c)
                        } else {
                            format!("( ) {}", c)
                        }
                    })
                    .collect();
                let content = parts.join("  ");
                let paragraph = Paragraph::new(Line::from(Span::styled(content, value_text_style)))
                    .block(block);
                f.render_widget(paragraph, chunks[i + 1]);
            } else {
                // If the field provides a backing values slice, always render the dedicated schedule_time component
                if let Some(values) = field.values {
                    let selected_idx = field.highlighted_line.unwrap_or(0);
                    // Ensure at least one line is allocated to the widget
                    let count = std::cmp::max(1, field.lines);
                    let widget = schedule_time(count, values, selected_idx, field.focused);
                    f.render_widget(widget, chunks[i + 1]);
                } else if field.lines > 1 {
                    // For multi-line fields without a backing values slice render each line separately so the active slot can be highlighted
                    let lines_vec: Vec<Line> = field
                        .value
                        .split('\n')
                        .enumerate()
                        .map(|(li, l)| {
                            let mut style = if field.focused {
                                content_style().add_modifier(Modifier::BOLD)
                            } else {
                                content_style().fg(COPY_COLOR)
                            };
                            if field.focused && field.highlighted_line == Some(li) {
                                style = style.fg(BORDER_COLOR).add_modifier(Modifier::REVERSED);
                            }
                            Line::from(Span::styled(l.to_string(), style))
                        })
                        .collect();
                    let paragraph = Paragraph::new(lines_vec).block(block);
                    f.render_widget(paragraph, chunks[i + 1]);
                } else {
                    let content = field.value.to_string();
                    let paragraph =
                        Paragraph::new(Line::from(Span::styled(content, value_text_style)))
                            .block(block);
                    f.render_widget(paragraph, chunks[i + 1]);
                }
            }
        }

        let help_index = self.fields.len() + 2;
        let combined = format!("{}  |  MODE: {}", self.help, self.mode);
        f.render_widget(bottom_bar(&combined), chunks[help_index]);
    }
}
