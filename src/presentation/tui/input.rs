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

#[cfg(test)]
mod tests {
    use crossterm::event::KeyCode;

    use super::*;

    #[test]
    fn from_code_char_returns_key_char() {
        assert_eq!(from_code(KeyCode::Char('a')), Key::Char('a'));
        assert_eq!(from_code(KeyCode::Char('z')), Key::Char('z'));
        assert_eq!(from_code(KeyCode::Char('1')), Key::Char('1'));
    }

    #[test]
    fn from_code_esc_returns_key_esc() {
        assert_eq!(from_code(KeyCode::Esc), Key::Esc);
    }

    #[test]
    fn from_code_enter_returns_key_enter() {
        assert_eq!(from_code(KeyCode::Enter), Key::Enter);
    }

    #[test]
    fn from_code_up_returns_key_up() {
        assert_eq!(from_code(KeyCode::Up), Key::Up);
    }

    #[test]
    fn from_code_down_returns_key_down() {
        assert_eq!(from_code(KeyCode::Down), Key::Down);
    }

    #[test]
    fn from_code_left_returns_key_left() {
        assert_eq!(from_code(KeyCode::Left), Key::Left);
    }

    #[test]
    fn from_code_right_returns_key_right() {
        assert_eq!(from_code(KeyCode::Right), Key::Right);
    }

    #[test]
    fn from_code_tab_returns_key_tab() {
        assert_eq!(from_code(KeyCode::Tab), Key::Tab);
    }

    #[test]
    fn from_code_backspace_returns_key_backspace() {
        assert_eq!(from_code(KeyCode::Backspace), Key::Backspace);
    }

    #[test]
    fn from_code_unknown_returns_other() {
        assert_eq!(from_code(KeyCode::F(1)), Key::Other);
        assert_eq!(from_code(KeyCode::Home), Key::Other);
        assert_eq!(from_code(KeyCode::Delete), Key::Other);
        assert_eq!(from_code(KeyCode::PageUp), Key::Other);
    }

    #[test]
    fn key_enum_derives_clone_debug_partialeq_eq() {
        let key = Key::Char('a');
        let cloned = key.clone();
        assert_eq!(key, cloned);
        let formatted = format!("{:?}", key);
        assert_eq!(formatted, "Char('a')");
    }
}
