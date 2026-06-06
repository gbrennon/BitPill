use bitpill::presentation::tui::keybindings;

#[test]
fn home_screen_empty_help() {
    let help = keybindings::home_screen_help(false);
    assert_eq!(help, " [c] Create  [s] Settings  [q] Quit");
}

#[test]
fn home_screen_with_medications_help() {
    let help = keybindings::home_screen_help(true);
    assert_eq!(
        help,
        " [c] Create  [Enter] Details  [m] Mark Taken  [e] Edit  [d] Delete  [s] Settings  [q] Quit"
    );
}

#[test]
fn medication_details_help_shows_m_not_s() {
    let help = keybindings::medication_details_help();
    // The handler expects 'm' — verify the help text matches
    assert!(
        help.contains("[m] Mark scheduled slot"),
        "medication_details_help should show [m], got: {help}"
    );
    assert!(
        !help.contains("[s]"),
        "medication_details_help should NOT show [s], got: {help}"
    );
}

#[test]
fn medication_details_help_format() {
    let help = keybindings::medication_details_help();
    assert_eq!(help, " [e] Edit  [m] Mark scheduled slot  [Esc] Back");
}

#[test]
fn mark_dose_help() {
    let help = keybindings::mark_dose_help();
    assert_eq!(help, " [Enter] Mark as taken  [j/k] Navigate  [Esc] Back");
}

#[test]
fn medication_form_help() {
    let help = keybindings::medication_form_help();
    assert_eq!(
        help,
        " [i] Insert  [Tab] Next field  [Enter] Submit  [Esc] Cancel"
    );
}

#[test]
fn settings_help() {
    let help = keybindings::settings_help();
    assert_eq!(
        help,
        " [?] Help  [Space/j/l] Toggle  [s] Save  [Esc] Cancel"
    );
}
