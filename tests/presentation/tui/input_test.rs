use bitpill::presentation::tui::input::{Key, from_code};
use crossterm::event::KeyCode;

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

#[test]
fn from_crossterm_key_event_converts_via_from_code() {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    let event = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
    let key = Key::from(event);
    assert_eq!(key, Key::Esc);

    let event = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE);
    let key = Key::from(event);
    assert_eq!(key, Key::Char('x'));

    let event = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
    let key = Key::from(event);
    assert_eq!(key, Key::Enter);
}
