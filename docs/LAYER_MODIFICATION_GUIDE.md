BitPill — Layer Modification Guide

Purpose

This document explains how to safely and consistently modify each layer of the BitPill codebase (domain, application, infrastructure, presentation, tests). Follow the conventions below to preserve the hexagonal architecture, testability, and maintainability.

Repository structure (high level)

- src/
  - domain/           — pure domain objects (entities, value objects, domain errors)
  - application/      — ports (traits) and use-case services that orchestrate domain
  - infrastructure/   — concrete adapters (persistence, clocks, notifications) and container.rs (composition root)
  - presentation/     — UI adapters (TUI, REST, CLI). Keep presentation code thin; inject services and ports.
- tests/              — integration and end-to-end tests for the whole application
- docs/               — guides and other documentation

Core principles to follow

- Hexagonal (Ports & Adapters): inner layers (domain, application) must not depend on outer layers.
- Single Responsibility: change one concern per commit/PR when possible.
- Ports as traits: express external capabilities as trait definitions under application/ports.
- Dependency injection via container.rs: only infrastructure/container.rs wires concrete implementations into services.
- Tests location: unit tests live next to the code under src/... using #[cfg(test)]; integration and E2E tests must live in the top-level tests/ directory.

Layer-specific guidance

1) Domain (src/domain)

- Purpose: Hold entities, value objects and pure domain logic (no I/O, no system calls).
- Files: one primary type per file; filename matches the type (snake_case).
- Validation: perform all input validation inside constructors (e.g., fn new -> Result<Self, DomainError>).
- Errors: domain/errors.rs contains domain-specific errors using thiserror.
- Tests: add unit tests as #[cfg(test)] modules at the bottom of the file.

When modifying domain:
- Add a new entity: create src/domain/entities/<name>.rs, implement behavior methods here.
- Add a value object: create src/domain/value_objects/<name>.rs and derive Clone/PartialEq/Eq/Hash when appropriate.
- Avoid adding any dependency on application, infrastructure, or presentation.

2) Application (src/application)

- Purpose: Define port traits (application/ports/) and implement use-case services (application/services/).
- Ports: every external capability (repository, clock, notification) must be a trait in application/ports. Name them with _port.rs suffix.
- Services: services in application/services/ should be pure orchestrators: validate input, construct domain objects, call ports, and return ApplicationError.
- Error handling: use application/errors.rs or a shared ApplicationError type for service return errors.

When modifying application:
- To add a new use case: create a port trait in application/ports/<use_case>_port.rs (define Request/Response DTOs), then implement service in application/services/<use_case>_service.rs.
- Inject ports via constructor (Arc<dyn Trait>) and keep services synchronous (no async unless project adopts runtime).
- Tests: add unit tests under application/ for services using fakes in application/ports/fakes.rs.

3) Infrastructure (src/infrastructure)

- Purpose: Implement concrete adapters that satisfy application ports (persistence, system clock, notification adapters), and provide a single composition root: infrastructure/container.rs.
- Container: only container.rs should instantiate concrete adapters and assemble Arc<dyn Trait> into services.
- Persistence adapters: place in infrastructure/persistence/, one file per adapter.

When modifying infrastructure:
- Implement/extend adapter: create new file in infrastructure/persistence or other appropriate subfolder, implement required trait(s) from application/ports.
- Ensure adapters do not leak their concrete error types to upper layers—map them to StorageError/ApplicationError as needed.
- Be careful with blocking I/O and locking: never hold Mutex guards while performing potentially slow file operations (serialize the data, drop the lock, then write to disk).
- Update container.rs to wire the new adapter into the composition root.
- Tests: unit tests for adapters may live under tests/ (integration-style) or under src/infrastructure with #[cfg(test)] if they are pure and fast. Prefer top-level tests/ for integration file-I/O tests.

4) Presentation (src/presentation)

- Purpose: UI adapters that translate user events into application port calls. Keep presentation code thin and side-effect-light.
- Structure: separate presenters, handlers, and dumb components. Handlers and presenters should be defined by ports (traits) so they are easily testable.
- Styles and components: centralize styles (e.g., src/presentation/tui/tui_styles.rs) and make components dumb: accept data+handlers, do not call services directly.

When modifying presentation:
- Add/modify presenter: create a presenters/ submodule with a presenter_port.rs trait and concrete implementation that handles UI rendering logic.
- Add/modify handler: create handlers/ with a handler port trait and move event handling into handler impls.
- Update UI components: components should accept content and handler callbacks; avoid global state.
- Tests: unit tests for presenters/handlers live next to implementation in src/presentation (#[cfg(test)]). E2E tests for UI flows belong in top-level tests/ as integration/E2E tests using TestBackend or other harnesses.

5) Tests (tests/)

- Integration/E2E tests must live in the repository root tests/ directory. They run with cargo test --tests and have access to the public crate API (use crate root as the crate name, e.g., bitpill::...).
- Naming: tests should reflect the path they exercise (e.g., tests/presentation_e2e.rs for presentation-level E2E tests). Use clear test names describing behaviour.
- Avoid placing integration or E2E tests inside src/ modules (presentation should not contain tests/ subfolder for E2E).

Common work flow for changes

1. Write or modify code following layer rules and SOLID guidelines.
2. Add unit tests next to the code under src/ for small-unit verification.
3. Add or update integration/E2E tests in tests/ to cover cross-layer behaviour.
4. Run the full test suite:
   - just fmt-check    # ensure formatting
   - just lint         # clippy -D warnings
   - just test         # runs all tests with coverage

Example: adding a medication repository

1. Define the port trait (if missing) in application/ports/medication_repository_port.rs.
2. Add an in-memory fake in application/ports/fakes.rs for unit tests.
3. Implement a JSON adapter in infrastructure/persistence/json_medication_repository.rs. Ensure you:
   - Limit mutex lock scope: serialize/save outside the lock
   - Map adapter errors to StorageError
4. Wire adapter in infrastructure/container.rs so services receive Arc<dyn MedicationRepository>.
5. Add unit tests for service logic under application/services using the fake repo.
6. Add integration test under tests/ to verify end-to-end persistence flow (create -> list -> read).

Notes & gotchas

- Never import application/infrastructure/presentation into domain files.
- Keep domain and application layers free of I/O and system APIs to maintain testability.
- When writing persistence adapters, avoid holding locks across file I/O to prevent UI or thread blocking.
- Follow naming conventions: one primary type per file; port files end with _port.rs.
- Place E2E/integration tests in tests/ to ensure they are executed as integration tests and can reference the public crate API.

Commands

- just fmt / just fmt-check
- just lint
- just build
- just test
- just run (where available)

If unsure, ask: provide the file you plan to change and a 1–2 sentence summary of the intended change (purpose + expected behavioural change). A short PR with focused changes is preferred over a large monolith change.

