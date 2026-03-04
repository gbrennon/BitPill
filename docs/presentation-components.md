Presentation components guide

Purpose

This document explains how to create small, testable, dumb presentation components for the TUI, how to center content vertically/horizontally with ratatui, and how to share styles.

Conventions

- One primary component per file. File name matches the component (snake_case).
- Components are dumb: they accept data and rendering options and return a widget (not drive application state).
- Presenters are the composition root for UI rendering: they iterate data, instantiate components, and place them into Layout chunks.
- All styles live in src/presentation/tui/styles/mod.rs and components should call style helpers (e.g. bar_style(), content_style()).

Creating a component

- Responsibility: render a single visual piece. Do not query repositories or mutate app state.
- API: accept the data to render and return a ratatui widget, for example:

  pub fn title_bar(subtitle: &str) -> Paragraph<'_> {
      let line = Line::from(vec![
          Span::styled("BitPill", bar_style().add_modifier(Modifier::BOLD)),
          Span::styled(if subtitle.is_empty() { String::new() } else { format!("  —  {subtitle}") }, bar_style()),
      ]);
      // single Line only; alignment centers horizontally inside its area
      Paragraph::new(line).style(bar_style()).alignment(Alignment::Center)
  }

Centering vertically

Paragraph alignment is horizontal only. To center vertically, render the widget into a Layout area that places it in the middle row. Example in a presenter:

  let chunks = Layout::default()
      .direction(Direction::Vertical)
      .constraints([Constraint::Length(TOP_BAR_HEIGHT), Constraint::Min(1), Constraint::Length(1)])
      .split(f.area());

  // create an inner layout for the top bar that centers its content vertically
  let top_inner = Layout::default()
      .direction(Direction::Vertical)
      .constraints([Constraint::Min(1), Constraint::Length(1), Constraint::Min(1)])
      .split(chunks[0]);

  f.render_widget(title_bar("Medications"), top_inner[1]);

This places the single-line title in the middle of the TOP_BAR_HEIGHT block.

Styles

- Centralize colors and style helpers in styles/mod.rs. Example:

  pub fn bar_style() -> Style { Style::default().bg(BORDER_COLOR).fg(BAR_TEXT_COLOR) }

- Components should not define colors inline; call style helpers instead so the whole UI can be themed easily.

Testing components

- Prefer unit tests in the component module. Create small tests that exercise creation and formatting of widgets.
- For integration/presentation tests, render the UI to a buffer and assert on layout or output slices.

Examples & Patterns

- Presenter: only composes components, owns Layout decisions, and maps domain DTOs into component inputs.
- Component: pure rendering of given data and styles.

Follow these rules to keep the presentation layer simple, testable, and consistent.
