pub mod create_medication_state;
pub mod edit_medication_state;
pub mod mark_dose_state;
pub mod settings_state;

pub use create_medication_state::CreateMedicationState;
pub use edit_medication_state::EditMedicationState;
pub use mark_dose_state::MarkDoseState;
pub use settings_state::SettingsState;

use crate::{application::dtos::responses::DoseRecordDto, presentation::tui::screen::Screen};

/// Central enum representing UI view state (struct-like variants for compatibility)
#[derive(Clone, Debug)]
pub enum ViewState {
    // legacy name kept for compatibility with existing code
    HomeScreen,
    CreateMedication {
        name: String,
        amount_mg: String,
        selected_frequency: usize,
        scheduled_time: Vec<String>,
        scheduled_idx: usize,
        focused_field: u8,
        insert_mode: bool,
    },
    EditMedication {
        id: String,
        name: String,
        amount_mg: String,
        selected_frequency: usize,
        scheduled_time: Vec<String>,
        scheduled_idx: usize,
        focused_field: u8,
        insert_mode: bool,
    },
    MedicationDetails {
        id: String,
    },
    MarkDose {
        medication_id: String,
        records: Vec<DoseRecordDto>,
        selected_index: usize,
    },
    /// Confirmation modal for deleting a medication
    ConfirmDelete {
        id: String,
        name: String,
    },
    /// Confirmation modal for cancelling an in-progress form (create/edit).
    ConfirmCancel {
        previous: Box<ViewState>,
    },
    /// Settings screen
    Settings {
        vim_enabled: bool,
        selected_index: usize,
    },
    /// Settings help modal - shows mode-specific navigation instructions
    SettingsHelp {
        vim_enabled: bool,
        selected_index: usize,
        help_text: String,
        previous: Box<Screen>,
    },
    /// Confirmation modal for quitting the application. Holds the previous view to return to on cancel.
    ConfirmQuit {
        previous: Box<ViewState>,
    },
    /// Validation/modal to show input errors; holds the previous view to return to when dismissed.
    ValidationError {
        messages: Vec<String>,
        previous: Box<ViewState>,
    },
}
