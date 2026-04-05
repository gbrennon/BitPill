use std::time::Duration;

use crate::presentation::tui::input::Key;

pub trait EventSource {
    fn poll(&self, duration: Duration) -> Result<bool, std::io::Error>;
    fn read_key(&self) -> Result<Key, std::io::Error>;
}

pub struct RealEventSource;

impl EventSource for RealEventSource {
    fn poll(&self, duration: Duration) -> Result<bool, std::io::Error> {
        crossterm::event::poll(duration)
    }

    fn read_key(&self) -> Result<Key, std::io::Error> {
        match crossterm::event::read()? {
            crossterm::event::Event::Key(k) => Ok(Key::from(k)),
            _ => Err(std::io::Error::other("RealEventSource: non-key event")),
        }
    }
}

#[derive(Clone, Default)]
pub struct FakeEventSource {
    poll_result: Option<bool>,
    read_result: Option<Key>,
}

impl FakeEventSource {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_poll_result(mut self, result: bool) -> Self {
        self.poll_result = Some(result);
        self
    }

    pub fn with_event(mut self, event: Key) -> Self {
        self.read_result = Some(event);
        self
    }

    pub fn with_key(mut self, key: Key) -> Self {
        self.read_result = Some(key);
        self
    }

    pub fn no_event(self) -> Self {
        self.with_poll_result(false)
    }

    pub fn with_quit_event(self) -> Self {
        self.with_poll_result(true).with_key(Key::Esc)
    }

    pub fn with_char_event(self, c: char) -> Self {
        self.with_poll_result(true).with_key(Key::Char(c))
    }
}

impl EventSource for FakeEventSource {
    fn poll(&self, _: Duration) -> Result<bool, std::io::Error> {
        Ok(self.poll_result.unwrap_or(false))
    }

    fn read_key(&self) -> Result<Key, std::io::Error> {
        self.read_result
            .clone()
            .ok_or_else(|| std::io::Error::other("FakeEventSource: no event configured"))
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn fake_event_source_default_is_empty() {
        let source = FakeEventSource::default();
        assert_eq!(source.poll_result, None);
        assert_eq!(source.read_result, None);
    }

    #[test]
    fn fake_event_source_new_is_empty() {
        let source = FakeEventSource::new();
        assert_eq!(source.poll_result, None);
        assert_eq!(source.read_result, None);
    }

    #[test]
    fn fake_event_source_with_poll_result_sets_result() {
        let source = FakeEventSource::new().with_poll_result(true);
        assert_eq!(source.poll_result, Some(true));
    }

    #[test]
    fn fake_event_source_with_event_sets_read_result() {
        let source = FakeEventSource::new().with_event(Key::Char('a'));
        assert_eq!(source.read_result, Some(Key::Char('a')));
    }

    #[test]
    fn fake_event_source_with_key_sets_read_result() {
        let source = FakeEventSource::new().with_key(Key::Esc);
        assert_eq!(source.read_result, Some(Key::Esc));
    }

    #[test]
    fn fake_event_source_no_event_sets_poll_false() {
        let source = FakeEventSource::new().no_event();
        assert_eq!(source.poll_result, Some(false));
    }

    #[test]
    fn fake_event_source_with_quit_event_sets_esc() {
        let source = FakeEventSource::new().with_quit_event();
        assert_eq!(source.poll_result, Some(true));
        assert_eq!(source.read_result, Some(Key::Esc));
    }

    #[test]
    fn fake_event_source_with_char_event_sets_char() {
        let source = FakeEventSource::new().with_char_event('q');
        assert_eq!(source.poll_result, Some(true));
        assert_eq!(source.read_result, Some(Key::Char('q')));
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
    fn fake_event_source_clone_produces_equal_instance() {
        let source = FakeEventSource::new()
            .with_poll_result(true)
            .with_key(Key::Char('c'));
        let cloned = source.clone();
        assert_eq!(cloned.poll_result, source.poll_result);
        assert_eq!(cloned.read_result, source.read_result);
    }

    #[test]
    fn fake_event_source_implements_event_source_trait() {
        let source: FakeEventSource = FakeEventSource::new();
        let _ = source.poll(Duration::ZERO);
        let _ = source.read_key();
    }

    #[test]
    fn event_source_trait_is_object_safe() {
        fn _assert_object_safe(_: &dyn EventSource) {}
    }
}
