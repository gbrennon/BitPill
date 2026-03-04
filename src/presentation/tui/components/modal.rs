use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, Paragraph, Clear};

use crate::presentation::tui::styles::{content_style, bar_style};

/// Internal struct to hold padding values for modal content. Not strictly necessary but improves readability.
/// Could be expanded in the future if we want to support different padding for x/y or more complex layouts.
/// Currently just a simple wrapper around two u16 values, but it makes the code more self-documenting and easier to adjust in one place if needed.
/// If we wanted to get fancier, we could even implement methods on this struct for calculating inner dimensions based on the overall modal size, but for now it's just a simple container.
/// Using a struct here is a bit of an overkill for just two values, but it helps clarify the intent and keeps related values together. It also makes it easier to extend in the future if we want to add more padding options or different types of padding (e.g. separate horizontal/vertical padding).
/// Overall, this is a small design choice that prioritizes readability and maintainability over minimalism, which I think is appropriate in this case since it makes the code clearer and easier to work with in the long run.
/// If we wanted to be more concise, we could just use two separate variables for pad_x and pad_y, but I think this struct approach is cleaner and more scalable. It also allows us to easily pass around padding values as a single unit if needed in the future, rather than having to manage multiple separate variables. So while it may seem like a bit of an over-engineering for just two values, I think it pays off in terms of code clarity and future flexibility.
/// In summary, this InternalPadding struct is a simple way to encapsulate the padding values for the modal content, making the code more readable and maintainable while also allowing for future extensibility if we want to add more complex padding options down the line.
#[derive(Debug)]
struct InternalPadding{
    x: u16,
    y: u16,
}

/// Renders a centered modal overlay styled to match the application.
/// Uses the application's bar style for the modal block (title/border) and
/// content_style for the message text so it visually matches the rest of the app.
/// A small inner padding is applied so text doesn't touch the borders.
pub fn render_modal(f: &mut Frame, area: Rect, title: &str, content: &str) {
    // Constrain modal size to reasonable default relative to full area
    let width = std::cmp::min(60u16, area.width.saturating_sub(10));
    let height = std::cmp::min(10u16, area.height.saturating_sub(6));
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    let r = Rect::new(x, y, width, height);

    // Clear the area behind the modal and render bordered block
    f.render_widget(Clear, r);
    let block = Block::default().title(title).borders(Borders::ALL).style(bar_style());
    f.render_widget(block, r);

    // Small padding inside the block so content doesn't touch borders.
    // Account for the 1-cell border on each side: reduce available inner size by 2
    // then apply requested padding so the block size stays the same while content
    // moves inward.

    let padding = InternalPadding { x: 1, y: 1 };
    // subtract 2 for the left+right borders, then subtract padding on both sides
    let inner_w_raw = width.saturating_sub(2).saturating_sub(padding.x.saturating_mul(2));
    let inner_h_raw = height.saturating_sub(2).saturating_sub(padding.y.saturating_mul(2));

    // compute inner origin inside the block: x + 1 for left border, + padding
    let inner_x = x + 1 + padding.x;
    let inner_y = y + 1 + padding.y;

    // Ensure inner rect is at least 1x1
    let inner_w = if inner_w_raw == 0 { 1 } else { inner_w_raw };
    let inner_h = if inner_h_raw == 0 { 1 } else { inner_h_raw };
    let inner = Rect::new(inner_x, inner_y, inner_w, inner_h);

    let p = Paragraph::new(content).style(content_style());
    f.render_widget(p, inner);
}
