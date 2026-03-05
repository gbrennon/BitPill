# BitPill — Maintenance Guide

This document explains how to maintain, debug, and extend the BitPill Rust project (TUI + services). It focuses on developer workflow, conventions, tests, and common pitfalls.

## Quick start
- Ensure Rust toolchain (stable) is installed and up to date (rustup). Use the repository's justfile for common tasks.
- Useful commands:
  - `just` — default: runs fmt-check + lint + test (with coverage)
  - `just run-tui` — run the terminal UI
  - `just test` — run full test suite (uses cargo llvm-cov)
  - `just test-one <name_substring>` — run a single test
  - `just lint` — run clippy (fail on warnings)
  - `just fmt` / `just fmt-check` — formatter

## Architecture overview
- Hexagonal architecture (domain, application, infrastructure, presentation). Dependencies point inward.
- One primary type per file (file name matches the type). Keep domain pure (no I/O).
- Ports are traits under `src/application/ports/` and suffix `_port.rs` for clarity.
- Application services live in `src/application/services/` and implement inbound ports.
- Infrastructure adapters live in `src/infrastructure/` and are wired only in `infrastructure/container.rs` (composition root).
- Presentation (TUI) lives under `src/presentation/tui/` with submodules for `components`, `handlers`, `presenters`, and `tui_styles.rs` for shared styles.

## Conventions to follow when making changes
- Follow SOLID: single responsibility, DI of ports via `Arc<dyn Trait>`, keep logic inside services/presenters, no I/O in domain.
- Use dependency injection; never instantiate infrastructure inside services.
- Components are dumb: inject content and handlers; presenters orchestrate rendering.
- Handlers implement the `Handler` trait in `src/presentation/tui/handlers/port.rs` and live in `src/presentation/tui/handlers/`.
- Presenters define a trait in `presenters/presenter_port.rs` and implement rendering logic in that module.

## Testing
- Unit tests: keep fast and isolated; use in-memory fakes and do not perform I/O. Shared/test helper fakes should live under `tests/fakes/` (or in `src/application/ports/fakes/` when feature-gated) and be available under `#[cfg(any(test, feature = "test-helpers"))]`.
- Integration/E2E tests: put under `tests/` and use `ratatui::backend::TestBackend` for TUI rendering checks.
- Always run `just test` after changes. Use `just test-one` for rapid feedback.
- Tests should follow Arrange/Act/Assert and use descriptive names.

## Debugging the TUI freeze (common pitfall)
Symptom: TUI freezes when creating a medication (UI stops responding) — usually caused by holding a Mutex lock while performing slow/blocking file I/O.

Fix pattern:
- Never hold a mutex across file I/O. Clone or serialize the data while holding the lock, then drop the lock and perform the file write outside the critical section.

Example pattern (pseudocode):
- let data = { let guard = mutex.lock().unwrap(); serialize(&*guard)? };
- write_file(&data)?; // file I/O outside lock

We applied this fix in `src/infrastructure/persistence/json_medication_repository.rs` — prefer this pattern when writing repositories that combine in-memory state and disk persistence.

## How to add a new feature (high-level)
1. Add a port trait in `src/application/ports/` (name_it_port.rs).
2. Implement the service in `src/application/services/` and keep business rules there.
3. Add a fake implementation in `src/application/ports/fakes.rs` for tests.
4. Add a concrete adapter (if needed) in `src/infrastructure/` and wire it in `infrastructure/container.rs`.
5. If it affects UI, add presenters/handlers under `src/presentation/tui/` and tests.
6. Add unit tests for services and handler tests for presentation logic.

## How to add a new handler
- Create a file in `src/presentation/tui/handlers/` with the handler struct.
- Implement the `Handler` trait from `handlers/port.rs`.
- Add a unit test that constructs an `App` via `App::new(Arc::new(Container::new()))` and asserts state changes.

## Style & UI
- All TUI colors and styles live in `src/presentation/tui/tui_styles.rs`. Update colors there and reference via `content_style()`, `bar_style()`, etc.
- Components (e.g., `item.rs`, `list.rs`) should accept content and style via parameters and avoid internal logic.

## Committing and code hygiene
- Follow repository's formatting and lint rules: run `just fmt` and `just lint` before opening PRs.
- Keep changes minimal and targeted (single responsibility per PR).

## Useful debugging tips
- Reproduce TUI freezes with `just run-tui` and use a small test sequence.
- Add temporary logs (or use dbg!) in services and repository code to trace blocking operations.
- Use unit tests/fakes to iterate quickly without touching file system.

## Contacts and further reading
- Review README.md and docs/ for feature-specific notes.
- Consult the `justfile` for available recipes.

---
Keep this guide updated as the project evolves. If unsure about architecture decisions, open an issue or ask before making broad changes.