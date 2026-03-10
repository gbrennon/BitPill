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
