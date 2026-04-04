use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Text},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::presentation::tui::styles::{bar_style, content_style};

#[derive(Debug)]
struct InternalPadding {
    x: u16,
    y: u16,
}

fn wrap_text(content: &str, max_width: u16) -> Text<'_> {
    let mut lines = Vec::new();
    for paragraph in content.split("\n\n") {
        for line in paragraph.lines() {
            if line.is_empty() {
                lines.push(Line::from(""));
                continue;
            }
            let words: Vec<&str> = line.split_whitespace().collect();
            let mut current_line = String::new();
            for word in words {
                let potential = if current_line.is_empty() {
                    word.to_string()
                } else {
                    format!("{} {}", current_line, word)
                };
                if potential.len() > max_width as usize && !current_line.is_empty() {
                    lines.push(Line::from(current_line.clone()));
                    current_line = word.to_string();
                } else {
                    current_line = potential;
                }
            }
            if !current_line.is_empty() {
                lines.push(Line::from(current_line));
            }
        }
        lines.push(Line::from(""));
    }
    Text::from(lines)
}

pub fn render_modal(f: &mut Frame, area: Rect, title: &str, content: &str) {
    let width = area.width.saturating_sub(4).clamp(40, 80);
    let height = area.height.saturating_sub(4).clamp(6, 20);
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    let r = Rect::new(x, y, width, height);

    f.render_widget(Clear, r);
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(bar_style());
    f.render_widget(block, r);

    let padding = InternalPadding { x: 1, y: 1 };
    let inner_w_raw = width
        .saturating_sub(2)
        .saturating_sub(padding.x.saturating_mul(2));
    let inner_h_raw = height
        .saturating_sub(2)
        .saturating_sub(padding.y.saturating_mul(2));

    let inner_x = x + 1 + padding.x;
    let inner_y = y + 1 + padding.y;

    let inner_w = if inner_w_raw == 0 { 1 } else { inner_w_raw };
    let inner_h = if inner_h_raw == 0 { 1 } else { inner_h_raw };
    let inner = Rect::new(inner_x, inner_y, inner_w, inner_h);

    let wrapped = wrap_text(content, inner_w);
    let p = Paragraph::new(wrapped).style(content_style());
    f.render_widget(p, inner);
}
