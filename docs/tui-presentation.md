# TUI Presentation Layer — Architecture & Guide

## Overview

The TUI presentation layer follows a **web-inspired template pattern** built on
[ratatui](https://ratatui.rs). Each screen is a combination of:

- a **template** (layout chrome — title bar + content slot + help bar), and
- a **presenter** (fills the content slot with screen-specific widgets).

This mirrors the "base template + page extends template" model familiar from web
frameworks (Django, Jinja2, Blade, etc.):

| Web concept | Ratatui equivalent |
|-------------|-------------------|
| Base layout (`base.html`) | `ScreenTemplate` / `FormTemplate` |
| `{% block content %}` | `render_content: FnOnce(&mut Frame, Rect)` closure |
| Page template | `*Presenter` struct + DTO |
| Partial / include | `components/*` functions |
| CSS variables | `styles/mod.rs` constants |

---

## Directory Map

```
src/presentation/tui/
├── app.rs                    — App state, event loop, terminal lifecycle
├── screen/mod.rs             — Screen navigation enum
├── styles/mod.rs             — Centralised colours & sizes
├── ui/mod.rs                 — draw() — routes Screen → Presenter
│
├── templates/                — LAYOUT CHROME (title + help bar)
│   ├── screen_template.rs    — ScreenTemplate: 3-zone base layout
│   └── form_template.rs      — FormTemplate: base + input field rows
│
├── presenters/               — CONTENT FILLERS (screen-specific rendering)
│   ├── medication_list_presenter.rs
│   ├── medication_details_presenter.rs
│   ├── create_medication_presenter.rs
│   └── schedule_result_presenter.rs
│
├── components/               — DUMB WIDGETS (reusable, stateless)
│   ├── title_bar.rs          — "BitPill — Subtitle" header
│   ├── bottom_bar.rs         — keybinding / status footer
│   ├── list.rs               — styled List from MedicationDto slice
│   ├── item.rs               — single "Name — Xmg" list item
│   └── detail.rs             — multi-line medication detail block
│
└── handlers/                 — EVENT ROUTING & MUTATION
    ├── event_handler.rs      — routes KeyEvent → screen handler
    ├── medication_list_handler.rs
    ├── create_medication_handler.rs
    └── schedule_result_handler.rs
```

---

## Templates

### `ScreenTemplate` — 3-zone base layout

Every non-form screen uses this. It owns the chrome; the caller fills the middle.

```
┌──────────────────────────────────────┐
│  Title bar  (TOP_BAR_HEIGHT = 5)     │  rendered by template
├──────────────────────────────────────┤
│                                      │
│  Content  (Min 0)                    │  ← render_content closure fills this
│                                      │
├──────────────────────────────────────┤
│  Help / status bar  (1 line)         │  rendered by template
└──────────────────────────────────────┘
```

**Usage:**

```rust
ScreenTemplate {
    subtitle: "My Screen",
    help: " [q] Quit  [Esc] Back",
}
.render(f, |f, area| {
    f.render_widget(my_widget, area);
});
```

**Fallback behaviour:** if the terminal is shorter than `TOP_BAR_HEIGHT + 2`,
the template paints the help text in the last row of the content zone so it is
always visible.

---

### `FormTemplate` — form variant

Extends `ScreenTemplate` with a dynamic number of labelled input blocks.

```
┌──────────────────────────────────────┐
│  Title bar                           │  rendered by template
├──────────────────────────────────────┤
│ ┌─ Field label ──────────────────┐   │
│ │ value                          │   │  3 lines per field
│ └────────────────────────────────┘   │
│ ┌─ Field label ──────────────────┐   │
│ │ value (focused → bold border)  │   │
│ └────────────────────────────────┘   │
│  …                                   │
│  (remaining space — Min 1)           │
├──────────────────────────────────────┤
│  Help / status bar                   │  rendered by template
└──────────────────────────────────────┘
```

**Usage:**

```rust
FormTemplate {
    subtitle: "Create Medication",
    fields: &[
        FormField { label: "Name",        value: dto.name,       focused: dto.focused_field == 0 },
        FormField { label: "Amount (mg)", value: dto.amount_mg,  focused: dto.focused_field == 1 },
    ],
    help: " [i] Insert  [Tab] Next  [Enter] Submit  [Esc] Cancel",
}
.render(f);
```

The number of fields is determined at runtime — add more `FormField` entries to grow
the form automatically.

---

## Presenters

A presenter is a unit-struct with one public method: `present(&self, f, dto)`.

- It receives a **typed DTO** containing only the data it needs (no `&App`).
- It delegates layout chrome to a **template**.
- It delegates widget construction to **components**.
- It contains **zero business logic**.

### Existing presenters

| Presenter | Template | DTO |
|-----------|----------|-----|
| `MedicationListPresenter` | `ScreenTemplate` | `&[MedicationDto]`, `selected_index`, `status_message` |
| `MedicationDetailsPresenter` | `ScreenTemplate` | `MedicationDetailsInput { medication: Option<&MedicationDto> }` |
| `CreateMedicationPresenter` | `FormTemplate` | `CreateMedicationPresenterDto` |
| `ScheduleResultPresenter` | `ScreenTemplate` | `ScheduleResultInput { created_count }` |

---

## Components

Components are **pure functions** that build ratatui widgets. They take data, apply
styles, and return a widget — they never call `f.render_widget` themselves.

```rust
// Build a widget — no side effects
pub fn medication_list(medications: &[MedicationDto]) -> List<'_> { … }

// Exception: render_title_bar calls f.render_widget because it must paint
// the background block before the text, requiring two widgets in one area.
pub fn render_title_bar(f: &mut Frame, area: Rect, subtitle: &str) { … }
```

---

## Styles

All colours and sizes live in `styles/mod.rs`. Import what you need:

```rust
use crate::presentation::tui::styles::{
    TOP_BAR_HEIGHT,  // u16 — height of the title bar zone
    content_style,   // bg: dark, fg: cream
    bar_style,       // bg: orange, fg: dark
    title_style,     // bar_style + Bold
    highlight_style, // fg: orange + Bold (for selected list items)
    BORDER_COLOR,    // Rgb(214, 93, 14) — orange
    COPY_COLOR,      // Rgb(217, 206, 195) — cream
};
```

---

## Adding a New Screen — Step-by-Step

### 1. Add a variant to `Screen`

```rust
// src/presentation/tui/screen/mod.rs
pub enum Screen {
    // …existing…
    DoseHistory { medication_id: String },
}
```

### 2. Create a DTO and presenter

```rust
// src/presentation/tui/presenters/dose_history_presenter.rs
use crate::presentation::tui::templates::screen_template::ScreenTemplate;
use ratatui::Frame;

pub struct DoseHistoryInput<'a> {
    pub medication_name: &'a str,
    pub doses: &'a [String],  // formatted dose strings
}

pub struct DoseHistoryPresenter;

impl DoseHistoryPresenter {
    pub fn present(&self, f: &mut Frame, input: &DoseHistoryInput) {
        ScreenTemplate {
            subtitle: "Dose History",
            help: " [Esc] Back",
        }
        .render(f, |f, area| {
            // build your widget here and render into `area`
        });
    }
}
```

### 3. Register the presenter module

```rust
// src/presentation/tui/presenters/mod.rs
pub mod dose_history_presenter;
```

### 4. Wire the screen in `ui/mod.rs`

```rust
Screen::DoseHistory { medication_id } => {
    let medication = app.medications.iter().find(|m| &m.id == medication_id);
    DoseHistoryPresenter.present(f, &DoseHistoryInput {
        medication_name: medication.map(|m| m.name.as_str()).unwrap_or(""),
        doses: &[],
    });
}
```

### 5. Add an event handler (optional)

If the screen needs keystroke handling beyond simple navigation, create a handler
in `handlers/` and register it in `handlers/event_handler.rs`.

---

## Data Flow (per frame)

```
Terminal::draw()
  └─ ui::draw(f, &app)
       ├─ render background block
       └─ match app.current_screen
            └─ XxxPresenter.present(f, &dto)
                 └─ ScreenTemplate / FormTemplate
                      ├─ render_title_bar(f, top_area, subtitle)
                      ├─ render_content closure  ← presenter fills this
                      └─ bottom_bar(help)
```

---

## Key Decisions

| Decision | Rationale |
|----------|-----------|
| Template takes a closure, not a trait | Zero-cost, no boxing, no lifetime complexity |
| FormTemplate builds constraints dynamically | Adding a field is a one-liner; no manual index tracking |
| Presenters receive DTOs, not `&App` | Decouples rendering from app state; enables isolated tests |
| Components return widgets, not render | Composable; caller decides where to paint |
| `render_title_bar` is the exception | Needs two layered widgets in the same area |
| All colours in `styles/mod.rs` | Change the whole theme in one file |
