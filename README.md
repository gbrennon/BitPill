# BitPill

> **Work in progress.** A personal medication management application built in Rust.
> *(More context about my health condition that motivated this project will be added soon.)*

BitPill helps individuals manage their daily medications — tracking pills, dosages, and schedules in one place.
It is being built with a focus on reliability and correctness, because when it comes to medication, errors matter.

---

## Prerequisites

- [Rust](https://rustup.rs/) (edition 2024, stable toolchain)
- [just](https://github.com/casey/just) — task runner (`cargo install just` or via your package manager)

Install all other dev tools with:

```bash
just tools
```

---

## Starting the Server

BitPill ships a REST API built with [actix-web](https://actix.rs/). By default `just run` starts it on **port 8080**.

```bash
just run         # REST server only  (http://localhost:8080)
just run-tui     # Terminal UI only
just run-both    # REST server in background + TUI in foreground
```

### REST API

#### Medications

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/medications` | List all medications |
| `POST` | `/medications` | Create a medication |

**List all medications**

```bash
curl http://localhost:8080/medications
```

```json
[
  {
    "id": "019535c4-...",
    "name": "Ibuprofen",
    "amount_mg": 400,
    "scheduled_times": [[8, 0], [20, 0]]
  }
]
```

**Create a medication**

`scheduled_times` is an array of `[hour, minute]` pairs (24-hour clock).

```bash
curl -X POST http://localhost:8080/medications \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Ibuprofen",
    "amount_mg": 400,
    "scheduled_times": [[8, 0], [20, 0]]
  }'
```

```json
{ "id": "019535c4-..." }
```

#### Doses

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/doses/schedule` | Schedule doses for the current minute |
| `POST` | `/doses/{id}/mark-taken` | Mark a dose record as taken |

**Schedule doses** (call this on a cron/timer each minute)

```bash
curl -X POST http://localhost:8080/doses/schedule
```

```json
{ "created_count": 2 }
```

**Mark a dose as taken**

`taken_at` must be in `YYYY-MM-DDTHH:MM:SS` format and cannot be in the future.

```bash
curl -X POST http://localhost:8080/doses/019535c4-.../mark-taken \
  -H "Content-Type: application/json" \
  -d '{ "taken_at": "2025-06-01T08:00:00" }'
```

```json
{ "record_id": "019535c4-..." }
```

#### Error responses

All error responses share the same shape:

```json
{ "error": "description of what went wrong" }
```

| Status | Meaning |
|--------|---------|
| `400` | Invalid input (bad dosage, empty name, bad datetime format) |
| `404` | Dose record not found |
| `500` | Unexpected server error |

---

## Terminal UI (TUI)

BitPill also ships a terminal interface built with [ratatui](https://ratatui.rs/).
To launch it instead of the REST server, replace `main.rs` with:

```rust
use std::sync::Arc;
use bitpill::infrastructure::container::Container;
use bitpill::presentation::tui::app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let container = Arc::new(Container::new());
    App::run(container)
}
```

### TUI keyboard shortcuts

| Screen | Key | Action |
|--------|-----|--------|
| Medication list | `c` | Open the create-medication form |
| Medication list | `s` | Run a scheduling tick |
| Medication list | `j` / `↓` | Move selection down |
| Medication list | `k` / `↑` | Move selection up |
| Medication list | `q` | Quit |
| Create form | `Tab` | Cycle between fields (Name → Amount → Times) |
| Create form | `Enter` | Submit the form |
| Create form | `Esc` | Cancel and go back |
| Schedule result | any key | Dismiss and go back |

`scheduled_times` in the form accepts comma-separated `HH:MM` entries, e.g. `08:00,20:00`.

---

## Development

### Running tests

```bash
just test        # full suite with coverage (cargo llvm-cov)
just test-one <name>  # single test by name substring
# e.g.: just test-one new_with_zero_amount_returns_error
```

### Default recipe (CI-equivalent)

Runs formatting check, lint, and tests with coverage in one command:

```bash
just
```

### All tasks

```bash
just build       # cargo build
just run         # REST server (http://localhost:8080)
just run-tui     # Terminal UI
just run-both    # REST server (background) + TUI (foreground)
just test        # tests + coverage report
just lint        # cargo clippy -- -D warnings
just fmt         # cargo fmt
just fmt-check   # formatting check only
just clean       # cargo clean
just tools       # install rustfmt, clippy, cargo-llvm-cov
```

---

## Architecture

BitPill follows **Hexagonal Architecture** (Ports & Adapters). Dependencies always point inward — outer layers know about inner layers, never the reverse.

```
┌──────────────────────────────────────────┐
│            Presentation Layer            │
│            (TUI, REST API)               │
├──────────────────────────────────────────┤
│          Infrastructure Layer            │
│   (Persistence, Clock, Notifications)    │
├──────────────────────────────────────────┤
│           Application Layer              │
│        (Use-Case Services, Ports)        │
├──────────────────────────────────────────┤
│              Domain Layer                │
│        (Entities, Value Objects)         │
└──────────────────────────────────────────┘
         ↑ Dependencies point inward ↑
```

### Layer responsibilities

| Layer | Responsibility |
|---|---|
| **Domain** | Core business rules — `Medication`, `DoseRecord`, `Dosage`, `ScheduledTime`, etc. Zero external dependencies; pure logic only. |
| **Application** | Use-case services (`CreateMedicationService`, `MarkDoseTakenService`, `ScheduleDoseService`, `ListAllMedicationsService`). Defines port traits that infrastructure implements. |
| **Infrastructure** | Concrete adapters: `InMemoryMedicationRepository`, `InMemoryDoseRecordRepository`, `SystemClock`, `ConsoleNotificationAdapter`. Wired together in `container.rs`. |
| **Presentation** | Delivery adapters — `rest/` (actix-web HTTP API) and `tui/` (ratatui terminal UI). |

### Module layout

```
src/
├── domain/
│   ├── entities/          # Medication, DoseRecord
│   └── value_objects/     # Dosage, MedicationId, ScheduledTime, TakenAt, …
├── application/
│   ├── ports/             # Trait definitions + fakes/ (test doubles)
│   └── services/          # Use-case implementations
├── infrastructure/
│   ├── clock/             # SystemClock, SystemScheduledTimeSupplier
│   ├── notifications/     # ConsoleNotificationAdapter
│   ├── persistence/       # InMemoryMedicationRepository, InMemoryDoseRecordRepository
│   └── container.rs       # Composition root
└── presentation/
    ├── rest/              # actix-web server + handlers
    └── tui/               # ratatui app + screens + event handling
```

### Dependency rule

| Allowed | Forbidden |
|---|---|
| `presentation` → `application` ✅ | `domain` → anything outer ❌ |
| `presentation` → `domain` ✅ | `application` → `infrastructure` ❌ |
| `infrastructure` → `application` ✅ | `application` → `presentation` ❌ |
| `application` → `domain` ✅ | `infrastructure` → `presentation` ❌ |

---

## Contributing

### Before you start

1. Make sure `just` and a stable Rust toolchain are installed.
2. Install dev tools: `just tools`
3. Confirm everything passes before touching any code: `just`

### Adding a new use case

1. **Define the port** — create `src/application/ports/my_action_port.rs` with a `Request`, `Response`, and a `trait MyActionPort: Send + Sync`.
2. **Implement the service** — create `src/application/services/my_action_service.rs`. Inject dependencies via `Arc<dyn SomePort>` in `new()`. No I/O allowed here.
3. **Add a fake** — add `src/application/ports/fakes/fake_my_repo.rs` if the service needs a new repository port. Re-export it from `src/application/ports/fakes/mod.rs`.
4. **Wire the container** — add the concrete adapter (if new) under `src/infrastructure/`, then add the service to `src/infrastructure/container.rs`.
5. **Expose in presentation** — add a REST handler in `src/presentation/rest/handlers/` and/or a TUI screen action.

### Code conventions

- **One primary type per file.** File name = type name in `snake_case`.
- **Unit tests** go in a `#[cfg(test)]` block at the bottom of the file under test. Use fakes from `crate::application::ports::fakes`, never real I/O.
- **Integration tests** go in `tests/` at the crate root and may use real infrastructure adapters.
- **No magic numbers or strings** — use named constants.
- **No `Box<dyn Error>`** in domain or application signatures — use typed error enums with `thiserror`.
- **Domain stays pure** — no `chrono`, no `uuid`, no `async`, no I/O inside `src/domain/`.

### Running the full check suite

This is equivalent to what CI runs:

```bash
just          # fmt-check + lint + test with coverage
```

All of these must pass before a contribution is considered complete.
