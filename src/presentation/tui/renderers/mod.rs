use ratatui::{Frame, widgets::Block};

use crate::presentation::tui::{app::App, styles::content_style};

mod confirm_cancel_renderer;
mod confirm_delete_renderer;
mod confirm_quit_renderer;
mod create_medication_renderer;
mod edit_medication_renderer;
mod home_screen_renderer;
mod mark_dose_renderer;
mod medication_details_renderer;
mod settings_renderer;

use confirm_cancel_renderer::ConfirmCancelRenderer;
use confirm_delete_renderer::ConfirmDeleteRenderer;
use confirm_quit_renderer::ConfirmQuitRenderer;
use create_medication_renderer::CreateMedicationRenderer;
use edit_medication_renderer::EditMedicationRenderer;
use home_screen_renderer::HomeScreenRenderer;
use mark_dose_renderer::MarkDoseRenderer;
use medication_details_renderer::MedicationDetailsRenderer;
use settings_renderer::SettingsRenderer;

use crate::presentation::tui::screen::Screen;

/// All screen renderers implement this trait.
/// Each renderer has exactly one reason to change: the screen it serves (SRP).
/// Adding a new screen requires only a new renderer file — this dispatcher stays closed
/// to modification (OCP).
pub trait ScreenRenderer {
    fn render(&self, f: &mut Frame, app: &App);
}

/// Top-level render entry point called by `draw::draw`.
fn render_view(f: &mut Frame, app: &App, view: &Screen) {
    match view {
        Screen::HomeScreen => HomeScreenRenderer.render(f, app),
        Screen::CreateMedication { .. } => CreateMedicationRenderer.render(f, app),
        Screen::EditMedication { .. } => EditMedicationRenderer.render(f, app),
        Screen::MedicationDetails { .. } => MedicationDetailsRenderer.render(f, app),
        Screen::MarkDose { .. } => MarkDoseRenderer.render(f, app),
        Screen::ConfirmDelete { .. } => ConfirmDeleteRenderer.render(f, app),
        Screen::ConfirmCancel { .. } => ConfirmCancelRenderer.render(f, app),
        Screen::Settings { .. } => SettingsRenderer.render(f, app),
        Screen::ConfirmQuit { .. } => ConfirmQuitRenderer.render(f, app),
        // fallback for new variants
        _ => HomeScreenRenderer.render(f, app),
    }
}

pub fn render(f: &mut Frame, app: &App) {
    f.render_widget(Block::default().style(content_style()), f.area());

    match &app.current_screen {
        Screen::ValidationError { message, previous } => {
            // render underlying view then dim overlay and modal
            render_view(f, app, previous);
            use ratatui::style::Color;
            // dim the background by drawing a semi-transparent overlay (simulated)
            let dim = Block::default().style(content_style().fg(Color::DarkGray));
            f.render_widget(dim, f.area());
            crate::presentation::tui::components::modal::render_modal(
                f,
                f.area(),
                "Validation error",
                message,
            );
        }
        Screen::ConfirmQuit { previous } => {
            render_view(f, app, previous);
            crate::presentation::tui::components::modal::render_modal(
                f,
                f.area(),
                "Confirm Quit",
                "Quit application?  (y/N)",
            );
        }
        Screen::ConfirmCancel { previous } => {
            render_view(f, app, previous);
            crate::presentation::tui::components::modal::render_modal(
                f,
                f.area(),
                "Confirm",
                "Discard changes?  (y/N)",
            );
        }
        Screen::HomeScreen => HomeScreenRenderer.render(f, app),
        Screen::CreateMedication { .. } => CreateMedicationRenderer.render(f, app),
        Screen::EditMedication { .. } => EditMedicationRenderer.render(f, app),
        Screen::MedicationDetails { .. } => MedicationDetailsRenderer.render(f, app),
        Screen::MarkDose { .. } => MarkDoseRenderer.render(f, app),
        Screen::ConfirmDelete { .. } => {
            // confirm delete doesn't track previous; just dim and show modal
            HomeScreenRenderer.render(f, app);
            crate::presentation::tui::components::modal::render_modal(
                f,
                f.area(),
                "Confirm Delete",
                "Delete this medication?  (y/N)",
            );
        }
        Screen::Settings { .. } => SettingsRenderer.render(f, app),
    }
}

#[cfg(test)]
mod tests {
    use ratatui::{Terminal, backend::TestBackend};

    use super::*;
    use crate::presentation::tui::app_services::AppServices;

    #[test]
    fn render_all_screens_no_panic() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut app = App {
            services: AppServices::fake(),
            current_screen: Screen::HomeScreen,
            medications: Vec::new(),
            selected_index: 0,
            status_message: None,
            status_expires_at: None,
            should_quit: false,
            show_welcome_modal: false,
        };

        let screens = vec![
            Screen::HomeScreen,
            Screen::CreateMedication {
                name: String::new(),
                amount_mg: String::new(),
                selected_frequency: 0,
                scheduled_time: Vec::new(),
                scheduled_idx: 0,
                focused_field: 0,
                insert_mode: false,
            },
            Screen::EditMedication {
                id: String::new(),
                name: String::new(),
                amount_mg: String::new(),
                selected_frequency: 0,
                scheduled_time: Vec::new(),
                scheduled_idx: 0,
                focused_field: 0,
                insert_mode: false,
            },
            Screen::MedicationDetails { id: String::new() },
            Screen::MarkDose {
                medication_id: String::new(),
                records: Vec::new(),
                selected_index: 0,
            },
            Screen::ConfirmDelete {
                id: String::new(),
                name: String::new(),
            },
            Screen::ConfirmCancel {
                previous: Box::new(Screen::HomeScreen),
            },
            Screen::Settings { vim_enabled: false },
            Screen::ConfirmQuit {
                previous: Box::new(Screen::HomeScreen),
            },
            Screen::ValidationError {
                message: String::from("err"),
                previous: Box::new(Screen::HomeScreen),
            },
        ];

        for s in screens {
            app.current_screen = s;
            terminal
                .draw(|f| {
                    render(f, &app);
                })
                .unwrap();
        }
    }

    #[test]
    fn render_view_with_non_home_previous_screens_no_panic() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut app = App {
            services: AppServices::fake(),
            current_screen: Screen::HomeScreen,
            medications: Vec::new(),
            selected_index: 0,
            status_message: None,
            status_expires_at: None,
            should_quit: false,
            show_welcome_modal: false,
        };

        // Exercise render_view with each Screen variant as the `previous` target
        let previous_screens: Vec<Box<Screen>> = vec![
            Box::new(Screen::CreateMedication {
                name: String::new(),
                amount_mg: String::new(),
                selected_frequency: 0,
                scheduled_time: Vec::new(),
                scheduled_idx: 0,
                focused_field: 0,
                insert_mode: false,
            }),
            Box::new(Screen::EditMedication {
                id: String::new(),
                name: String::new(),
                amount_mg: String::new(),
                selected_frequency: 0,
                scheduled_time: Vec::new(),
                scheduled_idx: 0,
                focused_field: 0,
                insert_mode: false,
            }),
            Box::new(Screen::MedicationDetails { id: String::new() }),
            Box::new(Screen::MarkDose {
                medication_id: String::new(),
                records: Vec::new(),
                selected_index: 0,
            }),
            Box::new(Screen::Settings { vim_enabled: false }),
            Box::new(Screen::ConfirmDelete {
                id: String::new(),
                name: String::new(),
            }),
            Box::new(Screen::ConfirmCancel {
                previous: Box::new(Screen::HomeScreen),
            }),
            Box::new(Screen::ConfirmQuit {
                previous: Box::new(Screen::HomeScreen),
            }),
        ];

        for prev in previous_screens {
            app.current_screen = Screen::ValidationError {
                message: "e".into(),
                previous: prev,
            };
            terminal
                .draw(|f| {
                    render(f, &app);
                })
                .unwrap();
        }
    }
}
