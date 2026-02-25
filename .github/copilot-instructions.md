# BitPill — Copilot Instructions

## Project

BitPill is a medication/pill management application built in Rust. It allows users to track pills, dosages, and schedules.

## Build, Test & Lint

```bash
cargo build
cargo test                          # full suite
cargo test <substring>              # single test by name
cargo test domain::                 # tests scoped to a module
cargo clippy -- -D warnings         # lint (must pass with zero warnings)
cargo fmt                           # format
cargo fmt --check                   # format check (CI)
cargo run                           # run the application
```

If a `justfile` is present, prefer `just <recipe>` over raw `cargo` commands.

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
src/domain/entities/pill.rs          → struct Pill
src/domain/value_objects/dosage.rs   → struct Dosage
src/application/ports/pill_repository.rs → trait PillRepository
src/application/services/create_pill_service.rs → struct CreatePillService
```

### Ports as Traits

Define every external capability as a `trait` inside `application/ports/`. Infrastructure adapters implement these traits — the core never knows the concrete type.

```rust
// src/application/ports/pill_repository.rs
use crate::domain::entities::pill::Pill;

pub trait PillRepository: Send + Sync {
    async fn save(&self, pill: &Pill) -> Result<(), RepositoryError>;
    async fn find_by_id(&self, id: &PillId) -> Result<Option<Pill>, RepositoryError>;
    async fn find_all(&self) -> Result<Vec<Pill>, RepositoryError>;
    async fn delete(&self, id: &PillId) -> Result<(), RepositoryError>;
}
```

### Application Services (Use Cases)

Services in `application/services/` receive port traits via constructor injection and contain **no I/O**.

```rust
// src/application/services/create_pill_service.rs
use std::sync::Arc;
use crate::application::ports::pill_repository::PillRepository;
use crate::domain::entities::pill::Pill;

pub struct CreatePillService {
    repository: Arc<dyn PillRepository>,
}

impl CreatePillService {
    pub fn new(repository: Arc<dyn PillRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, name: String) -> Result<Pill, ServiceError> {
        let pill = Pill::new(name);
        self.repository.save(&pill).await?;
        Ok(pill)
    }
}
```

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
// src/domain/entities/pill.rs
impl Pill {
    pub fn schedule(&mut self, dosage: Dosage, time: ScheduledTime) -> Result<(), DomainError> {
        // domain logic here
    }

    pub fn is_due(&self, now: &DateTime<Utc>) -> bool {
        // ...
    }
}
```

### Composition Root

Wire all concrete adapters in `infrastructure/container.rs`. No other module should instantiate concrete adapters with `new`.

```rust
// src/infrastructure/container.rs
pub struct Container {
    pub create_pill_service: CreatePillService,
}

impl Container {
    pub fn new(db_path: &str) -> Self {
        let repository = Arc::new(SqlitePillRepository::new(db_path));
        Self {
            create_pill_service: CreatePillService::new(repository),
        }
    }
}
```

### Error Types

- Domain errors live in `domain/errors.rs` and use `thiserror`.
- Infrastructure and application layers define their own error types.
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

## Per-Context Instructions

Scoped instruction files in `.github/instructions/` apply automatically to matching files:

| File | Applies to |
|---|---|
| `domain.instructions.md` | `src/domain/**` |
| `application.instructions.md` | `src/application/**` |
| `infrastructure.instructions.md` | `src/infrastructure/**` |
| `tests.instructions.md` | `**/*_test.rs`, `**/tests/**` |
