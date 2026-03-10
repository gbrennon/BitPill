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
    fn from(k: CrosstermKeyEvent) -> Self {
        use crossterm::event::KeyCode;
        match k.code {
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
}
