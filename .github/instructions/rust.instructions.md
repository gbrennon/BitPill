---
applyTo: "src/**"
---

# Rust Code Rules

## Import Placement

- All `use` declarations must appear at the **top of the file**, before any `mod`, `struct`, `enum`, `trait`, `impl`, or `fn` items.
- Group imports in this order, separated by a blank line:
  1. `std` / `core` library imports
  2. External crate imports (e.g. `chrono`, `serde`, `uuid`, `ratatui`)
  3. Internal crate imports (`crate::…`)
- Never scatter `use` statements inside function bodies or `impl` blocks, except inside `#[cfg(test)]` modules where local imports are acceptable for test-only dependencies.

```rust
// 1. std
use std::sync::Arc;

// 2. external crates
use chrono::NaiveDateTime;
use ratatui::Frame;

// 3. internal
use crate::application::ports::medication_repository_port::MedicationRepository;
use crate::domain::entities::medication::Medication;
```

---

## SOLID Principles

### Single Responsibility (S)
- Each file defines **one** primary `struct`, `enum`, or `trait`. File name matches the type in `snake_case`.
- Each function or method does one thing. Split a function when it operates at more than one level of abstraction.
- Handlers, services, and components must not mix concerns: a service orchestrates, a repository persists, a handler reacts to input.

### Open / Closed (O)
- Extend behaviour through new trait implementations, not by adding `match` arms or `if` chains on concrete types.
- Prefer passing `Arc<dyn Trait>` over passing concrete structs so callers are open to new implementations without modification.
- Avoid `match typeof(x)` patterns; dispatch through trait methods instead.

### Liskov Substitution (L)
- Every type that implements a trait must honour the full contract of that trait.
- Never implement a trait method by panicking, returning a hard-coded error, or silently doing nothing unless the trait contract explicitly allows it.
- Fakes and stubs in tests must behave like valid implementations — they may be simplified but must not violate invariants.

### Interface Segregation (I)
- Port traits must be **narrow**: define only the methods that a single consumer actually needs.
- If two callers need different subsets of a type's behaviour, define two separate traits rather than one fat trait.
- Avoid adding convenience methods to a trait; put them in a helper function or a default-method extension trait instead.

### Dependency Inversion (D)
- Application services depend on port **traits** (`Arc<dyn PortTrait>`), never on concrete infrastructure types.
- `infrastructure/container.rs` is the **only** place that instantiates concrete adapters. No other module may call `new` on an infrastructure type.
- Pass dependencies through constructors; never reach into global state, environment variables, or singletons from inside domain or application code.
