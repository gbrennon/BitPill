use std::time::Duration;

use bitpill::presentation::tui::{
    event_source::{EventSource, FakeEventSource},
    input::Key,
};

#[test]
fn event_source_trait_is_object_safe() {
    fn _assert_object_safe(_: &dyn EventSource) {}
}

#[test]
fn fake_event_source_poll_returns_configured_result() {
    let source = FakeEventSource::new().with_poll_result(true);
    let result = source.poll(Duration::from_millis(100));
    assert_eq!(result.unwrap(), true);
}

#[test]
fn fake_event_source_poll_returns_false_by_default() {
    let source = FakeEventSource::new();
    let result = source.poll(Duration::from_millis(100));
    assert_eq!(result.unwrap(), false);
}

#[test]
fn fake_event_source_no_event_returns_poll_false() {
    let source = FakeEventSource::new().no_event();
    let result = source.poll(Duration::from_millis(100));
    assert_eq!(result.unwrap(), false);
}

#[test]
fn fake_event_source_read_key_returns_configured_key() {
    let source = FakeEventSource::new().with_key(Key::Enter);
    let result = source.read_key();
    assert_eq!(result.unwrap(), Key::Enter);
}

#[test]
fn fake_event_source_read_key_returns_error_when_not_configured() {
    let source = FakeEventSource::new();
    let result = source.read_key();
    assert!(result.is_err());
}

#[test]
fn fake_event_source_with_quit_event_poll_returns_true_and_key_esc() {
    let source = FakeEventSource::new().with_quit_event();
    assert_eq!(source.poll(Duration::ZERO).unwrap(), true);
    assert_eq!(source.read_key().unwrap(), Key::Esc);
}

#[test]
fn fake_event_source_with_char_event_sets_char() {
    let source = FakeEventSource::new().with_char_event('q');
    assert_eq!(source.poll(Duration::ZERO).unwrap(), true);
    assert_eq!(source.read_key().unwrap(), Key::Char('q'));
}

#[test]
fn fake_event_source_clone_produces_equal_behaviour() {
    let source = FakeEventSource::new()
        .with_poll_result(true)
        .with_key(Key::Char('c'));
    let cloned = source.clone();
    assert_eq!(
        cloned.poll(Duration::ZERO).unwrap(),
        source.poll(Duration::ZERO).unwrap()
    );
    assert_eq!(cloned.read_key().unwrap(), source.read_key().unwrap());
}

#[test]
fn fake_event_source_implements_event_source_trait() {
    let source: FakeEventSource = FakeEventSource::new();
    let _ = source.poll(Duration::ZERO);
    let _ = source.read_key();
}

#[test]
fn fake_event_source_with_event_returns_configured_key() {
    let source = FakeEventSource::new().with_event(Key::Char('a'));
    let result = source.read_key();
    assert_eq!(result.unwrap(), Key::Char('a'));
}

#[test]
fn fake_event_source_with_key_returns_configured_key() {
    let source = FakeEventSource::new().with_key(Key::Esc);
    let result = source.read_key();
    assert_eq!(result.unwrap(), Key::Esc);
}
