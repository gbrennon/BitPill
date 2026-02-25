---
applyTo: "src/domain/**"
---

# Domain Layer Rules

- Files in `src/domain/` must have **zero** dependencies on `application`, `infrastructure`, or `presentation` modules.
- Each file defines exactly one primary `struct`, `enum`, or `trait`. File name matches the type name in `snake_case`.
- **Entities** (`domain/entities/`) have an `id` field and carry behaviour — no anemic structs with only getters/setters.
- **Value Objects** (`domain/value_objects/`) are immutable. Derive `Clone`, `PartialEq`, `Eq`, `Hash` as appropriate. Never expose `&mut self` methods.
- All domain validation belongs in the constructor (`fn new(...) -> Result<Self, DomainError>`), not in the caller.
- Domain errors live in `domain/errors.rs` and use `thiserror`. Never use `Box<dyn Error>` in domain signatures.
- No `use std::io`, no file I/O, no network, no async runtime — pure logic only.
- Avoid generic names (`Manager`, `Handler`, `Helper`). Use the ubiquitous language of the domain (e.g., `Pill`, `Dosage`, `Schedule`).
