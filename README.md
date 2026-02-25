# BitPill 💊

> **Work in progress.** A personal medication management application built in Rust.
> *(More context about the health condition that motivated this project will be added soon.)*

BitPill helps individuals manage their daily medications — tracking pills, dosages, and schedules in one place. It is being built with a focus on reliability and correctness, because when it comes to medication, errors matter.

---

## Prerequisites

- [Rust](https://rustup.rs/) (edition 2024, stable toolchain)
- [just](https://github.com/casey/just) — task runner (`cargo install just` or via your package manager)

Install all other dev tools with:

```bash
just tools
```

---

## Running the Application

> ⚠️ The application is in early development. Running it currently demonstrates the core domain working end-to-end.

```bash
just run
# or: cargo run
```

---

## Running Tests

Run the full test suite:

```bash
just test
# or: cargo test
```

Run a single test by name (substring match):

```bash
just test-one <name>
# e.g.: just test-one new_with_zero_amount_returns_error
```

### Coverage

The **default `just` recipe** runs formatting checks, linting, and tests with coverage:

```bash
just
```

To run coverage on its own:

```bash
just coverage
```

### Other tasks

```bash
just lint        # cargo clippy -- -D warnings
just fmt         # cargo fmt
just fmt-check   # formatting check (used in CI)
just build       # cargo build
just clean       # cargo clean
```

---

## Architecture

BitPill follows **Hexagonal Architecture** (Ports & Adapters).

The core domain has no knowledge of how data is stored, displayed, or delivered — those are pluggable adapters.

```
┌──────────────────────────────────────────┐
│            Presentation Layer            │
│         (CLI, TUI, REST, etc.)           │
├──────────────────────────────────────────┤
│          Infrastructure Layer            │
│   (Persistence, External APIs, I/O)      │
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
| **Domain** | Core business rules — `Pill`, `Dosage`, `PillName`, `PillId`. No I/O, no framework dependencies. |
| **Application** | Use-case services (e.g. `CreatePillService`). Defines port traits that infrastructure must implement. |
| **Infrastructure** | Concrete adapters: persistence (`InMemoryPillRepository`, future SQLite). Wired together in `container.rs`. |
| **Presentation** | Delivery mechanism — currently a minimal CLI. Will grow into a TUI or other interface. |

### Key rules

- `domain` and `application` never import from `infrastructure` or `presentation`.
- All concrete adapters are instantiated in exactly one place: `infrastructure/container.rs`.
- Port traits (`application/ports/`) are the only coupling point between the application core and the outside world.
