use crate::presentation::tui::templates::form_template::{FormField, FormTemplate};
use ratatui::Frame;

pub fn render_medication_form<'a>(
    f: &mut Frame,
    subtitle: &str,
    name: &'a str,
    amount_mg: &'a str,
    scheduled_time: &'a [String],
    scheduled_idx: usize,
    focused_field: u8,
    insert_mode: bool,
    status_message: Option<&'a str>,
    frequency_options: &'a [&'a str],
    selected_frequency: usize,
) {
    let help = status_message
        .unwrap_or(" [i] Insert  [Tab] Next field  [Enter] Submit  [Esc] Cancel");

    let mode = if insert_mode { "INSERT" } else { "NORMAL" };

    // Build per-slot display values, showing placeholder when slot empty.
    // Ensure number of rendered slots is driven by frequency but at least matches any existing values.
    let base_slots = match selected_frequency {
        0 => 1,
        1 => 2,
        2 => 3,
        _ => 3,
    };
    let scheduled_lines = std::cmp::max(base_slots, scheduled_time.len());
    let mut display_slots: Vec<String> = Vec::new();
    for i in 0..scheduled_lines {
        let s = scheduled_time.get(i).map(|s| s.as_str()).unwrap_or("");
        if s.trim().is_empty() {
            display_slots.push("HH:MM:SS".to_string());
        } else {
            display_slots.push(s.to_string());
        }
    }
    let scheduled_value = display_slots.join("\n");

    FormTemplate {
        subtitle,
        fields: &[
            FormField {
                label: "Name",
                value: name,
                focused: focused_field == 0,
                choices: None,
                selected_choice: None,
                lines: 3,
                highlighted_line: None,
                values: None,
            },
            FormField {
                label: "Amount (mg)",
                value: amount_mg,
                focused: focused_field == 1,
                choices: None,
                selected_choice: None,
                lines: 3,
                highlighted_line: None,
                values: None,
            },
            FormField {
                label: "Dose Frequency",
                value: "",
                focused: focused_field == 2,
                choices: Some(frequency_options),
                selected_choice: Some(selected_frequency),
                lines: 3,
                highlighted_line: None,
                values: None,
            },
            FormField {
                label: "Scheduled time(s) (HH:MM:SS)",
                value: &scheduled_value,
                focused: focused_field == 3,
                choices: None,
                selected_choice: None,
                lines: scheduled_lines,
                highlighted_line: if focused_field == 3 { Some(scheduled_idx) } else { None },
                values: Some(scheduled_time),
            },
        ],
        help,
        mode,
    }
    .render(f);
}
