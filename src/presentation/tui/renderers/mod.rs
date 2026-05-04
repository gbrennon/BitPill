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
        Screen::ValidationError { messages, previous } => {
            // render underlying view then dim overlay and modal
            render_view(f, app, previous);
            use ratatui::style::Color;
            // dim the background by drawing a semi-transparent overlay (simulated)
            let dim = Block::default().style(content_style().fg(Color::DarkGray));
            f.render_widget(dim, f.area());
            let content = messages.join("\n");
            crate::presentation::tui::components::modal::render_modal(
                f,
                f.area(),
                "Validation error",
                &content,
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
        Screen::SettingsHelp {
            help_text,
            previous,
            ..
        } => {
            render_view(f, app, previous);
            let content = format!("{}\n\nPress any key to close", help_text);
            crate::presentation::tui::components::modal::render_modal(
                f,
                f.area(),
                "Navigation Help",
                &content,
            );
        }
    }
}
