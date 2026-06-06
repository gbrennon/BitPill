/// A single keybinding definition.
pub struct Keybinding {
    pub key: &'static str,
    pub description: &'static str,
}

impl Keybinding {
    pub const fn new(key: &'static str, description: &'static str) -> Self {
        Self { key, description }
    }

    /// Formats as ` [key] description`.
    pub fn format(&self) -> String {
        format!(" [{}] {}", self.key, self.description)
    }
}

/// Builds a help string from a list of keybindings followed by optional navigation hints.
pub fn build_help(actions: &[Keybinding], navigation: &str) -> String {
    let mut s = String::new();
    for a in actions {
        s.push_str(&a.format());
    }
    if !navigation.is_empty() {
        s.push_str("  ");
        s.push_str(navigation);
    }
    s
}

// ─── HomeScreen ──────────────────────────────────────────────────────────────

pub fn home_screen_help(has_medications: bool) -> &'static str {
    if has_medications {
        HOME_HELP_WITH_MEDS
    } else {
        HOME_HELP_EMPTY
    }
}

static HOME_HELP_EMPTY: &str = " [c] Create  [s] Settings  [q] Quit";
static HOME_HELP_WITH_MEDS: &str =
    " [c] Create  [Enter] Details  [m] Mark Taken  [e] Edit  [d] Delete  [s] Settings  [q] Quit";

// ─── MedicationDetails ───────────────────────────────────────────────────────

pub fn medication_details_help() -> &'static str {
    MEDICATION_DETAILS_HELP
}

static MEDICATION_DETAILS_HELP: &str = " [e] Edit  [m] Mark scheduled slot  [Esc] Back";

// ─── MarkDose ────────────────────────────────────────────────────────────────

pub fn mark_dose_help() -> &'static str {
    MARK_DOSE_HELP
}

static MARK_DOSE_HELP: &str = " [Enter] Mark as taken  [j/k] Navigate  [Esc] Back";

// ─── Medication Form (Create / Edit) ─────────────────────────────────────────

pub fn medication_form_help() -> &'static str {
    MEDICATION_FORM_HELP
}

static MEDICATION_FORM_HELP: &str = " [i] Insert  [Tab] Next field  [Enter] Submit  [Esc] Cancel";

// ─── Settings ────────────────────────────────────────────────────────────────

pub fn settings_help() -> &'static str {
    SETTINGS_HELP
}

static SETTINGS_HELP: &str = " [?] Help  [Space/j/l] Toggle  [s] Save  [Esc] Cancel";
