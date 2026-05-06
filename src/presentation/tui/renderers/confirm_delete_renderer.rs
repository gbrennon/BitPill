use ratatui::Frame;

use crate::presentation::tui::{
    app::App, components::modal::render_modal, renderers::ScreenRenderer, screen::Screen,
};

pub struct ConfirmDeleteRenderer;

impl ScreenRenderer for ConfirmDeleteRenderer {
    fn render(&self, f: &mut Frame, app: &App) {
        let Screen::ConfirmDelete { name, .. } = &app.current_screen else {
            return;
        };

        let content = format!("Delete medication '{}'?  (y/N)", name);
        render_modal(f, f.area(), "Confirm Delete", &content);
    }
}

#[cfg(test)]
mod tests {
    use ratatui::prelude::*;

    use super::*;
    use crate::presentation::tui::{app::App, screen::Screen};

    #[test]
    fn test_render_does_not_panic_on_confirm_delete() {
        let renderer = ConfirmDeleteRenderer;
        let mut app = App::default();
        app.current_screen = Screen::ConfirmDelete {
            id: "id".to_string(),
            name: "TestMed".to_string(),
        };

        use ratatui::{Terminal, backend::TestBackend};
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|f| {
                renderer.render(f, &app);
            })
            .unwrap();
    }

    #[test]
    fn test_render_does_nothing_on_wrong_screen() {
        let renderer = ConfirmDeleteRenderer;
        let mut app = App::default();

        use ratatui::{Terminal, backend::TestBackend};
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|f| {
                // Should not panic or do anything
                renderer.render(f, &app);
            })
            .unwrap();
    }
}
