# BitPill — Copilot Instructions

## Build, Test & Lint

- Use `just` for all major tasks:
  - `just` — runs formatting check, lint, and tests with coverage
  - `just build` — builds the project
  - `just run` — runs the default binary (both REST + TUI via CLI args)
  - `just run-api` — starts the REST API server only (http://localhost:8080)
  - `just run-tui` — launches the terminal UI only
  - `just run-both` — alias for `just run`
  - `just test` — runs all tests with coverage
  - `just test-one <name_substring>` — runs a single test by name substring
  - `just lint` — runs clippy with warnings as errors
  - `just fmt` — formats code
  - `just fmt-check` — checks formatting
  - `just clean` — cleans build artifacts
  - `just tools` — installs dev tools (rustfmt, clippy, cargo-llvm-cov)
- Raw `cargo` equivalents work, but prefer `just`.

## High-Level Architecture

BitPill uses Hexagonal Architecture (Ports & Adapters):
- **Presentation Layer**: TUI (ratatui) and REST API (actix-web)
- **Infrastructure Layer**: JSON file-based persistence, Clock, Notifications; all concrete adapters wired in `infrastructure/container.rs`
- **Application Layer**: Use-case services and port traits (e.g., `CreateMedicationService`, `MarkDoseTakenService`)
- **Domain Layer**: Pure business logic (entities, value objects, error types)
- Dependencies always point inward; outer layers know about inner layers, never the reverse.

### Ports are split into two sub-namespaces

- `application/ports/inbound/` — use-case interfaces (e.g., `CreateMedicationPort`); implemented by services, called by presentation.
- `application/ports/outbound/` — infrastructure interfaces (e.g., `MedicationRepository`, `ClockPort`); implemented by adapters in `infrastructure/`, called by services.

### TUI internal pattern

The TUI (`presentation/tui/`) uses an MVP-like pattern:
- `App::current_screen` (a `ViewState` / `Screen` enum) is the single source of truth for which screen is active and its inline field state.
- `Handler` trait (`handlers/port.rs`) — one handler per screen; receives `KeyEvent` and mutates `App`. Must be imported explicitly: `use …::tui::handlers::port::Handler`.
- `Presenter` trait (`presenters/presenter_port.rs`) — one presenter per screen; renders to a `ratatui::Frame`.
- `draw.rs` dispatches to the correct `Presenter` based on `app.current_screen`.

## Key Conventions

- **One primary type per file**: Each file defines one main struct, enum, or trait; file name matches type name in `snake_case`.
- **Ports as traits**: All external capabilities are defined as traits in `application/ports/`, suffixed with `_port.rs`. Infrastructure implements these traits; core never knows concrete types.
- **Services**: Application services receive dependencies via constructor injection (`Arc<dyn PortTrait>`), never instantiate adapters directly, and contain no I/O.
- **Test fakes are `#[cfg(test)]`-gated**: Shared fakes live in `src/application/ports/fakes/` but are compiled only under `#[cfg(test)]`. **Integration tests in `tests/` cannot import them** — use the real infrastructure with a tempdir and `BITPILL_DOSE_RECORDS_FILE` / `BITPILL_MEDICATIONS_FILE` env vars instead.
- **`DoseRecord::new()` creates an untaken record** (`taken_at = None`). Call `.mark_taken(datetime)?` explicitly before saving to persist it as taken.
- **Rust 2024 edition**: `std::env::set_var` and `remove_var` are unsafe — wrap them in `unsafe {}` blocks.
- **Error handling**: Domain errors use `thiserror` in `domain/errors.rs`. Application errors are in `application/errors.rs` as `ApplicationError` and subtypes (`StorageError`, `NotFoundError`, `DeliveryError`). Never use `Box<dyn Error>` in domain/application signatures.
- **No magic numbers/strings**: Use named constants or newtype wrappers in domain/application code.
- **Per-context instructions**: Scoped rules in `.github/instructions/` apply automatically to matching files.

## Summary

- This file consolidates build/test/lint commands, the high-level Hexagonal Architecture used by BitPill, and repository-specific conventions to help future Copilot sessions work effectively.
- If you'd like additions (CI workflows, MCP servers, or more granular conventions), say what to add and changes will be made.
