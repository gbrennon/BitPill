use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style, Modifier};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use ratatui::Frame;

use crate::presentation::tui::app::App;
use crate::presentation::tui::screen::Screen;
use crate::presentation::tui::components::title_bar::title_bar;

const BACKGROUND_COLOR: Color = Color::Rgb(28, 27, 26);
const COPY_COLOR: Color = Color::Rgb(217, 206, 195);
const BORDER_COLOR: Color = Color::Rgb(214, 93, 14);
const TOP_BAR_HEIGHT: u16 = 3;

fn bar_style() -> Style {
    Style::default().bg(BORDER_COLOR)
}

fn content_style() -> Style {
    Style::default().bg(BACKGROUND_COLOR).fg(COPY_COLOR)
}

pub fn draw(f: &mut Frame, app: &App) {
    f.render_widget(Block::default().style(content_style()), f.area());

    match &app.current_screen {
        Screen::MedicationList => draw_medication_list(f, app),
        Screen::CreateMedication {
            name,
            amount_mg,
            scheduled_times,
            focused_field,
        } => draw_create_medication(f, name, amount_mg, scheduled_times, *focused_field),
        Screen::ScheduleResult { created_count } => draw_schedule_result(f, *created_count),
    }
}

fn draw_medication_list(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(TOP_BAR_HEIGHT),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(f.area());

    f.render_widget(title_bar("Medications"), chunks[0]);

    let items: Vec<ListItem> = app
        .medications
        .iter()
        .map(|m| {
            let label = format!("{} — {}mg", m.name, m.amount_mg);
            ListItem::new(label).style(content_style())
        })
        .collect();

    let mut state = ListState::default();
    if !app.medications.is_empty() {
        state.select(Some(app.selected_index));
    }

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::RIGHT)
                .border_style(Style::default().fg(BORDER_COLOR))
                .style(content_style()),
        )
        .highlight_style(
            Style::default()
                .fg(BORDER_COLOR)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    f.render_stateful_widget(list, chunks[1], &mut state);

    let help = Paragraph::new(" [c] Create  [s] Schedule  [q] Quit").style(bar_style());
    f.render_widget(help, chunks[2]);
}

fn draw_create_medication(
    f: &mut Frame,
    name: &str,
    amount_mg: &str,
    scheduled_times: &str,
    focused_field: u8,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(TOP_BAR_HEIGHT),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(f.area());

    f.render_widget(title_bar("Create Medication"), chunks[0]);

    let fields = [
        ("Name", name, 0u8),
        ("Amount (mg)", amount_mg, 1u8),
        ("Scheduled times (HH:MM,...)", scheduled_times, 2u8),
    ];

    for (i, (label, value, field_idx)) in fields.iter().enumerate() {
        let is_focused = *field_idx == focused_field;
        let title_style = if is_focused {
            Style::default().fg(BORDER_COLOR).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(COPY_COLOR)
        };
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(BORDER_COLOR))
            .style(content_style())
            .title(Span::styled(*label, title_style));
        let paragraph =
            Paragraph::new(Line::from(Span::styled(*value, content_style()))).block(block);
        f.render_widget(paragraph, chunks[i + 1]);
    }

    let help =
        Paragraph::new(" [Tab] Next field  [Enter] Submit  [Esc] Cancel").style(bar_style());
    f.render_widget(help, chunks[5]);
}

fn draw_schedule_result(f: &mut Frame, created_count: usize) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(TOP_BAR_HEIGHT),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(f.area());

    f.render_widget(title_bar("Schedule Result"), chunks[0]);

    let msg = format!("  {created_count} dose(s) scheduled. Press any key to return.");
    let paragraph = Paragraph::new(msg).style(content_style());
    f.render_widget(paragraph, chunks[1]);

    let help = Paragraph::new(" [any key] Back").style(bar_style());
    f.render_widget(help, chunks[2]);
}
