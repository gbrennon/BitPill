use crate::presentation::tui::app::App;
use ratatui::Frame;

pub trait Presenter {
    fn present(&self, f: &mut Frame, app: &App);
}
