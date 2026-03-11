use crossterm::event::KeyEvent as CrosstermKeyEvent;

/// Simplified, presentation-layer key type to decouple handlers/tests from crossterm.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Esc,
    Enter,
    Up,
    Down,
    Left,
    Right,
    Tab,
    Backspace,
    // Fallback for keys we don't care about in tests/handlers
    Other,
}

impl From<CrosstermKeyEvent> for Key {
    fn from(key: CrosstermKeyEvent) -> Self {
        from_code(key.code)
    }
}

/// Converts a crossterm `KeyCode` into the presentation `Key` enum.
/// Use this in tests and helpers instead of constructing `crossterm::event::KeyEvent`.
pub fn from_code(code: crossterm::event::KeyCode) -> Key {
    use crossterm::event::KeyCode;
    match code {
        KeyCode::Char(c) => Key::Char(c),
        KeyCode::Esc => Key::Esc,
        KeyCode::Enter => Key::Enter,
        KeyCode::Up => Key::Up,
        KeyCode::Down => Key::Down,
        KeyCode::Left => Key::Left,
        KeyCode::Right => Key::Right,
        KeyCode::Tab => Key::Tab,
        KeyCode::Backspace => Key::Backspace,
        _ => Key::Other,
    }
}
