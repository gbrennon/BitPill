# BitPill — Copilot Instructions

## Project

BitPill is a medication/pill management application built in Rust. It allows users to track pills, dosages, and schedules.

## Build, Test & Lint

```bash
just              # default: fmt-check + lint + test (with coverage)
just build
just run
just test         # runs cargo llvm-cov (coverage included)
just test-one <name_substring>   # single test via cargo test
just lint                        # cargo clippy -- -D warnings
just fmt
just fmt-check
just clean
```

Raw `cargo` equivalents work too; prefer `just` when the recipe exists.

> **Note:** `just test` runs `cargo llvm-cov` and includes coverage. There is no separate `just coverage` recipe.

---

## Architecture

This project follows **Hexagonal Architecture** (Ports & Adapters). Dependencies flow **inward only** — inner layers never depend on outer layers.

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

### Module Layout

```
src/
├── domain/
│   ├── mod.rs
│   ├── entities/       # Aggregate roots and entities
│   └── value_objects/  # Immutable value types
├── application/
│   ├── mod.rs
│   ├── ports/          # Trait definitions owned by the domain/application
│   └── services/       # Use-case services (no I/O)
├── infrastructure/
│   ├── mod.rs
│   ├── container.rs    # Composition root — only place that wires dependencies
│   └── persistence/    # Trait implementations for storage
└── presentation/
    ├── mod.rs
    └── ...             # UI or delivery adapters
```

### Dependency Rule

| Allowed | Forbidden |
|---|---|
| `presentation` → `application` ✅ | `domain` → anything outer ❌ |
| `presentation` → `domain` ✅ | `application` → `infrastructure` ❌ |
| `infrastructure` → `domain` ✅ | `application` → `presentation` ❌ |
| `application` → `domain` ✅ | `infrastructure` → `presentation` ❌ |

---

## Key Conventions

### One Primary Type Per File

Each file defines exactly one primary `struct`, `enum`, or `trait`. The file name matches the type name in `snake_case`.

```
src/domain/entities/medication.rs          → struct Medication
src/domain/entities/dose_record.rs         → struct DoseRecord
src/domain/value_objects/dosage.rs         → struct Dosage
src/domain/value_objects/medication_id.rs  → struct MedicationId
src/application/ports/medication_repository_port.rs → trait MedicationRepository
src/application/ports/clock_port.rs             → trait ClockPort
src/application/services/create_medication_service.rs → struct CreateMedicationService
```

### Ports as Traits

Define every external capability as a `trait` inside `application/ports/`. Infrastructure adapters implement these traits — the core never knows the concrete type.

Port methods are **synchronous** — do not use `async fn` unless a specific async runtime is adopted.

Port files use a `_port.rs` suffix (e.g., `medication_repository_port.rs`, `clock_port.rs`, `notification_port.rs`).

```rust
// src/application/ports/medication_repository_port.rs
use crate::application::errors::StorageError;
use crate::domain::{entities::medication::Medication, value_objects::medication_id::MedicationId};

pub trait MedicationRepository: Send + Sync {
    fn save(&self, medication: &Medication) -> Result<(), StorageError>;
    fn find_by_id(&self, id: &MedicationId) -> Result<Option<Medication>, StorageError>;
    fn find_all(&self) -> Result<Vec<Medication>, StorageError>;
    fn delete(&self, id: &MedicationId) -> Result<(), StorageError>;
}
```

The `ClockPort` trait abstracts the system clock — inject it via `Arc<dyn ClockPort>` instead of calling `chrono::Local::now()` directly inside services. This keeps services fully testable.

### Application Services (Use Cases)

Services in `application/services/` receive port traits via constructor injection and contain **no I/O**. Service methods are synchronous.

```rust
// src/application/services/create_medication_service.rs
use std::sync::Arc;
use crate::application::ports::medication_repository::MedicationRepository;
use crate::domain::entities::medication::Medication;

pub struct CreateMedicationService {
    repository: Arc<dyn MedicationRepository>,
}

impl CreateMedicationService {
    pub fn new(repository: Arc<dyn MedicationRepository>) -> Self {
        Self { repository }
    }

    pub fn execute(&self, name: impl Into<String>, amount_mg: u32, ...) -> Result<Medication, CreateMedicationError> {
        // validate inputs, build domain objects, call repository
    }
}
```

Each service defines its own error enum (e.g. `CreateMedicationError`) that uses `#[from]` to wrap both `DomainError` and `StorageError`.

### Value Objects Are Immutable

Value objects live in `domain/value_objects/` and are defined by their attributes, not identity. Derive `Clone`, `PartialEq`, `Eq`, and `Hash` as appropriate. Never expose `&mut self` methods.

```rust
// src/domain/value_objects/dosage.rs
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dosage {
    amount_mg: u32,
}

impl Dosage {
    pub fn new(amount_mg: u32) -> Result<Self, DomainError> {
        if amount_mg == 0 {
            return Err(DomainError::InvalidDosage);
        }
        Ok(Self { amount_mg })
    }

    pub fn amount_mg(&self) -> u32 {
        self.amount_mg
    }
}
```

### Entities Carry Behaviour

Entities have identity (`id`) and carry domain behaviour. Do not create anemic structs with only getters/setters.

```rust
// src/domain/entities/dose_record.rs
impl DoseRecord {
    pub fn mark_taken(&mut self, at: NaiveDateTime) -> Result<(), DomainError> {
        if self.taken_at.is_some() {
            return Err(DomainError::DoseAlreadyTaken);
        }
        self.taken_at = Some(at);
        Ok(())
    }

    pub fn is_taken(&self) -> bool {
        self.taken_at.is_some()
    }
}
```

### Composition Root

Wire all concrete adapters in `infrastructure/container.rs`. No other module should instantiate concrete adapters with `new`.

```rust
// src/infrastructure/container.rs
pub struct Container {
    pub create_medication_service: CreateMedicationService,
    pub mark_dose_taken_service: MarkDoseTakenService,
}

impl Container {
    pub fn new() -> Self {
        let medication_repo = Arc::new(InMemoryMedicationRepository::new());
        let dose_record_repo = Arc::new(InMemoryDoseRecordRepository::new());
        Self {
            create_medication_service: CreateMedicationService::new(medication_repo),
            mark_dose_taken_service: MarkDoseTakenService::new(dose_record_repo),
        }
    }
}
```

### Test Fakes

Shared fake implementations of all port traits live in `src/application/ports/fakes.rs`. Import them in `#[cfg(test)]` modules — do not duplicate inline fakes.

Available fakes:
- `FakeClock::at(hour, minute)` — returns a fixed `NaiveDateTime`
- `FakeMedicationRepository::new()` / `::with(medications)` / `::failing()` — in-memory + `saved_count()`
- `FakeDoseRecordRepository::new()` / `::with(record)` — in-memory + `saved_count()`
- `FakeNotificationPort::new()` — records calls + `call_count()`

```rust
#[cfg(test)]
mod tests {
    use crate::application::ports::fakes::{FakeClock, FakeMedicationRepository};
    use std::sync::Arc;

    #[test]
    fn execute_valid_input_saves_medication() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let clock = Arc::new(FakeClock::at(8, 0));
        let service = CreateMedicationService::new(repo.clone(), clock);

        let result = service.execute(...);

        assert!(result.is_ok());
        assert_eq!(repo.saved_count(), 1);
    }
}
```

### Error Types

- Domain errors live in `domain/errors.rs` and use `thiserror`.
- Application-level infrastructure errors live in `application/errors.rs`: `StorageError`, `NotFoundError`, `ConflictError`, `DeliveryError`. These are shared across all port traits.
- Each service defines its own error enum wrapping `DomainError` and the relevant application error via `#[from]`.
- Never propagate raw `Box<dyn Error>` through domain or application layers.

```rust
// src/domain/errors.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("invalid dosage: amount must be greater than zero")]
    InvalidDosage,
}
```

### No Magic Numbers or Strings

Use named constants or newtype wrappers instead of bare literals in domain and application code.

---

## DDD Patterns

### Aggregate Roots

An Aggregate Root is the sole entry point to a cluster of related objects. Repositories accept and return Aggregate Roots — never their internal components.

### Ubiquitous Language

Use domain language in all identifiers and comments. Avoid generic names like `Manager`, `Handler`, or `Helper` in domain and application code.

### Repository Abstraction

Repositories are domain-defined abstractions. They operate on Aggregate Roots only and hide all persistence details from the domain.

---

## Current State Notes

- `#![allow(dead_code)]` in `main.rs` is intentional — large parts of the presentation layer (`app.rs`, `screen.rs`, `ui.rs`, `event_handler.rs`) are TUI work in progress and not yet wired into `main`.
- `src/infrastructure/persistence/` and `src/infrastructure/container.rs` are currently empty stubs — no concrete repository adapters have been implemented yet.
- `src/application/ports/create_medication_port.rs` and `list_all_medications_port.rs` define complete request/response DTOs and port traits but are not yet implemented by any service.

---

## Per-Context Instructions

Scoped instruction files in `.github/instructions/` apply automatically to matching files:

| File | Applies to |
|---|---|
| `domain.instructions.md` | `src/domain/**` |
| `application.instructions.md` | `src/application/**` |
| `infrastructure.instructions.md` | `src/infrastructure/**` |
| `tests.instructions.md` | `**/*_test.rs`, `**/tests/**` |
