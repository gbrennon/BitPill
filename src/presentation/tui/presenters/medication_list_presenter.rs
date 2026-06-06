use ratatui::{Frame, widgets::ListState};

use crate::{
    application::dtos::responses::MedicationDto,
    presentation::tui::{
        components::table::medication_table, keybindings,
        templates::screen_template::ScreenTemplate,
    },
};

pub struct MedicationListPresenter;

impl MedicationListPresenter {
    pub fn present(
        &self,
        f: &mut Frame,
        medications: &[MedicationDto],
        selected_index: usize,
        status_message: Option<&String>,
    ) {
        let help_text = status_message
            .map(|s| s.as_str())
            .unwrap_or_else(|| keybindings::home_screen_help(!medications.is_empty()));

        let mut state = ListState::default();
        if !medications.is_empty() {
            state.select(Some(selected_index));
        }

        ScreenTemplate {
            subtitle: "Medications",
            help: help_text,
            mode: "NORMAL",
        }
        .render(f, |f, area| {
            let selected = if medications.is_empty() {
                None
            } else {
                Some(selected_index)
            };
            if medications.is_empty() {
                f.render_widget(
                    medication_table("", &["Name", "mg"], medications, selected),
                    area,
                );
            } else {
                f.render_widget(
                    medication_table(
                        "",
                        &["Name", "mg", "Taken", "Actions"],
                        medications,
                        selected,
                    ),
                    area,
                );
            }
        });
    }
}
