# Domain-Driven Design in Rust

A practical guide to the core DDD building blocks and how they map to Rust's type system,
ownership model, and module structure. Examples are drawn from the BitPill domain wherever
possible.

---

## Table of Contents

1. [Value Object](#1-value-object)
2. [Entity](#2-entity)
3. [Aggregate Root](#3-aggregate-root)
4. [Domain Event](#4-domain-event)
5. [Repository](#5-repository)
6. [Domain Service](#6-domain-service)
7. [Factory](#7-factory)
8. [Bounded Context](#8-bounded-context)
9. [Ubiquitous Language](#9-ubiquitous-language)
10. [Summary](#10-summary)

---

## 1. Value Object

> An object that has no conceptual identity. Two value objects are equal if all their
> attributes are equal.

Value objects are the cheapest and most powerful modelling tool in DDD. They replace
primitive types (`String`, `u32`, `i64`) with domain-meaningful types that self-validate.

### Characteristics

- **Immutable** — no `&mut self` methods.
- **Equality by value** — derive `PartialEq`, `Eq`.
- **Self-validating** — invariants enforced in `fn new(...) -> Result<Self, DomainError>`.
- **No identity** — two instances with identical data are interchangeable.

### Example — `Dosage`

```rust
// src/domain/value_objects/dosage.rs

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dosage {
    amount_mg: u32,   // private: callers cannot construct an invalid Dosage
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

impl std::fmt::Display for Dosage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}mg", self.amount_mg)
    }
}
```

`Dosage` cannot be created with `0` mg. Any code that holds a `Dosage` can trust it is
valid — the constructor is the only guard needed.

### Why not just `u32`?

```rust
// ❌ Primitive obsession — what unit? Is 0 valid? Is 10_000 valid?
fn prescribe(amount: u32) { /* ... */ }

// ✅ Self-documenting, self-validating
fn prescribe(dosage: Dosage) { /* ... */ }
```

### Value object checklist

- Fields are private; expose read-only accessors only.
- Derive `Clone`, `PartialEq`, `Eq` (and `Hash` when used in maps/sets).
- Never expose `&mut self` methods.
- All validation in `new` — never in the caller.

---

## 2. Entity

> An object defined by its identity, not its attributes. Two entities with the same `id`
> are the same object even if all other fields differ.

### Characteristics

- Has an **`id` field** (typically a newtype wrapping `Uuid`).
- **Mutable state** — entities transition through a lifecycle.
- **Identity equality** — two entities are the same iff their `id` matches.
- **Carries behaviour** — not an anemic bag of data.

### Example — `DoseRecord`

```rust
// src/domain/entities/dose_record.rs

pub struct DoseRecord {
    id: DoseRecordId,
    medication_id: MedicationId,
    scheduled_at: NaiveDateTime,
    taken_at: Option<NaiveDateTime>,   // ← mutable lifecycle state
}

impl DoseRecord {
    pub fn new(medication_id: MedicationId, scheduled_at: NaiveDateTime) -> Self {
        Self {
            id: DoseRecordId::generate(),
            medication_id,
            scheduled_at,
            taken_at: None,
        }
    }

    /// State transition: pending → taken. Enforces the "only once" invariant.
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

`mark_taken` is not a setter — it is a **domain command** that enforces the business rule
"a dose can only be marked taken once."

### Identity newtype

```rust
// src/domain/value_objects/dose_record_id.rs

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DoseRecordId(Uuid);

impl DoseRecordId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }
}
```

Wrapping `Uuid` in a newtype prevents accidentally passing a `MedicationId` where a
`DoseRecordId` is expected — the compiler enforces it.

---

## 3. Aggregate Root

> A cluster of domain objects treated as a single unit for the purpose of data changes.
> The root is the only entry point — external code never holds direct references to
> inner objects.

### Characteristics

- **Single entry point** — all mutations go through the root.
- **Owns its boundaries** — child entities and value objects are private or exposed
  read-only.
- **Consistency boundary** — a repository loads and saves the entire aggregate atomically.
- **Guards invariants across the cluster**.

### Example — `Medication` as Aggregate Root

```rust
// src/domain/entities/medication.rs

pub struct Medication {
    id: MedicationId,
    name: MedicationName,
    dosage: Dosage,
    scheduled_time: Vec<ScheduledTime>,   // child value objects
}

impl Medication {
    pub fn new(
        id: MedicationId,
        name: MedicationName,
        dosage: Dosage,
        scheduled_time: Vec<ScheduledTime>,
    ) -> Self { /* ... */ }

    // Read-only access to children — callers cannot mutate them directly
    pub fn scheduled_time(&self) -> &[ScheduledTime] {
        &self.scheduled_time
    }

    // All mutations go through the root
    pub fn reschedule(&mut self, times: Vec<ScheduledTime>) {
        self.scheduled_time = times;
    }

    pub fn update_dosage(&mut self, dosage: Dosage) {
        self.dosage = dosage;
    }
}
```

### Aggregate boundary rules

```rust
// ❌ Bypass the root — breaks the consistency boundary
let times = &mut medication.scheduled_time; // field is private, compiler rejects this

// ✅ All changes via the root
medication.reschedule(vec![ScheduledTime::new(9, 0).unwrap()]);
```

### References between aggregates — IDs only

```rust
// ❌ Holding a reference to another aggregate breaks the boundary
pub struct DoseRecord {
    medication: Arc<Medication>,   // wrong
}

// ✅ Reference by identity only
pub struct DoseRecord {
    medication_id: MedicationId,   // correct — load the aggregate when needed
}
```

---

## 4. Domain Event

> A record that something meaningful happened in the domain. Events are immutable facts
> in the past tense.

### Characteristics

- **Immutable** — an event describes something that already happened.
- **Named in past tense** — `DoseTaken`, `MedicationCreated`.
- **Collected, then dispatched** — aggregate methods push events onto an internal queue;
  the application service dispatches them after saving.

### Defining events

```rust
// src/domain/events.rs

#[derive(Debug, Clone)]
pub enum DomainEvent {
    MedicationCreated {
        medication_id: MedicationId,
        name: String,
    },
    DoseTaken {
        dose_record_id: DoseRecordId,
        medication_id: MedicationId,
        taken_at: NaiveDateTime,
    },
    DoseMissed {
        dose_record_id: DoseRecordId,
        medication_id: MedicationId,
        scheduled_at: NaiveDateTime,
    },
}
```

### Collecting events in the aggregate root

```rust
pub struct Medication {
    id: MedicationId,
    name: MedicationName,
    dosage: Dosage,
    scheduled_time: Vec<ScheduledTime>,
    events: Vec<DomainEvent>,   // internal event queue
}

impl Medication {
    pub fn new(id: MedicationId, name: MedicationName, dosage: Dosage, scheduled_time: Vec<ScheduledTime>) -> Self {
        let mut med = Self { id, name, dosage, scheduled_time, events: vec![] };
        med.events.push(DomainEvent::MedicationCreated {
            medication_id: med.id.clone(),
            name: med.name.value().to_owned(),
        });
        med
    }

    /// Drains and returns all pending domain events.
    pub fn drain_events(&mut self) -> Vec<DomainEvent> {
        std::mem::take(&mut self.events)
    }
}
```

### Dispatching from the application service

```rust
// application/services/create_medication.rs

pub async fn execute(&self, cmd: CreateMedicationCommand) -> Result<MedicationId, ApplicationError> {
    let mut medication = Medication::new(/* ... */);
    let id = medication.id().clone();

    self.repository.save(&medication).await?;

    // Dispatch only after a successful save
    for event in medication.drain_events() {
        self.event_bus.publish(event).await?;
    }

    Ok(id)
}
```

---

## 5. Repository

> A collection-like interface for loading and storing aggregates. The domain defines the
> contract; infrastructure provides the implementation.

### The port (domain/application layer)

```rust
// src/application/ports/medication_repository_port.rs

#[async_trait::async_trait]
pub trait MedicationRepositoryPort: Send + Sync {
    async fn save(&self, medication: &Medication) -> Result<(), RepositoryError>;
    async fn find_by_id(&self, id: &MedicationId) -> Result<Option<Medication>, RepositoryError>;
    async fn find_all(&self) -> Result<Vec<Medication>, RepositoryError>;
    async fn delete(&self, id: &MedicationId) -> Result<(), RepositoryError>;
}
```

### Key rules

- One repository per **aggregate root** — never per entity or value object.
- Returns domain types (`Medication`), not persistence models (`MedicationRow`).
- The trait lives in `application/ports/` — it belongs to the domain/application side,
  not infrastructure.

### The adapter (infrastructure layer)

```rust
// src/infrastructure/persistence/json_medication_repository.rs

pub struct JsonMedicationRepository {
    path: PathBuf,
}

#[async_trait::async_trait]
impl MedicationRepositoryPort for JsonMedicationRepository {
    async fn save(&self, medication: &Medication) -> Result<(), RepositoryError> {
        // map Medication → JSON, write to disk
        // infrastructure concern — the domain never knows about files
    }

    async fn find_by_id(&self, id: &MedicationId) -> Result<Option<Medication>, RepositoryError> {
        // read JSON → map to Medication
    }
}
```

### In-memory fake for tests

```rust
// src/application/ports/fakes.rs

pub struct FakeMedicationRepository {
    store: Mutex<HashMap<MedicationId, Medication>>,
}

#[async_trait::async_trait]
impl MedicationRepositoryPort for FakeMedicationRepository {
    async fn save(&self, medication: &Medication) -> Result<(), RepositoryError> {
        self.store.lock().unwrap().insert(medication.id().clone(), medication.clone());
        Ok(())
    }
    // ...
}
```

The fake satisfies the same contract as the real adapter, enabling fully in-memory unit
tests.

---

## 6. Domain Service

> A stateless operation that belongs to the domain but doesn't naturally fit on a single
> entity or value object.

Use a domain service when an operation:

- Involves **multiple aggregates**.
- Has no obvious "home" entity.
- Must remain **free of infrastructure concerns**.

### Example — `DoseScheduler`

```rust
// src/domain/services/dose_scheduler.rs

pub struct DoseScheduler;

impl DoseScheduler {
    /// Generates all DoseRecords due for a Medication on a given day.
    pub fn schedule_for_day(medication: &Medication, date: NaiveDate) -> Vec<DoseRecord> {
        medication
            .scheduled_time()
            .iter()
            .map(|time| {
                let scheduled_at = date.and_hms_opt(time.hour(), time.minute(), 0).unwrap();
                DoseRecord::new(medication.id().clone(), scheduled_at)
            })
            .collect()
    }
}
```

This logic involves both `Medication` and `DoseRecord` without belonging to either.

### Domain service vs application service

| | Domain Service | Application Service |
|---|---|---|
| Layer | `domain/services/` | `application/services/` |
| Dependencies | Domain types only | Ports (repos, clocks, notifiers) |
| State | Stateless | Stateless |
| I/O | None | Via injected ports |
| Purpose | Domain logic | Orchestration |

---

## 7. Factory

> Encapsulates complex creation logic for aggregates or value objects.

When a constructor becomes too complex — especially when involving multiple validation
steps or when creation logic varies by context — extract a factory.

### Simple factory function

```rust
// src/domain/factories/medication_factory.rs

pub struct CreateMedicationInput {
    pub name: String,
    pub amount_mg: u32,
    pub scheduled_time: Vec<(u8, u8)>,  // (hour, minute) pairs
}

pub fn create_medication(input: CreateMedicationInput) -> Result<Medication, DomainError> {
    let name = MedicationName::new(&input.name)?;
    let dosage = Dosage::new(input.amount_mg)?;
    let times = input.scheduled_time
        .into_iter()
        .map(|(h, m)| ScheduledTime::new(h, m))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Medication::new(MedicationId::generate(), name, dosage, times))
}
```

### Builder pattern for complex aggregates

When aggregates have many optional fields, the Builder pattern prevents telescoping
constructors:

```rust
pub struct MedicationBuilder {
    name: Option<MedicationName>,
    dosage: Option<Dosage>,
    scheduled_time: Vec<ScheduledTime>,
}

impl MedicationBuilder {
    pub fn new() -> Self {
        Self { name: None, dosage: None, scheduled_time: vec![] }
    }

    pub fn name(mut self, name: MedicationName) -> Self {
        self.name = Some(name);
        self
    }

    pub fn dosage(mut self, dosage: Dosage) -> Self {
        self.dosage = Some(dosage);
        self
    }

    pub fn scheduled_time(mut self, time: ScheduledTime) -> Self {
        self.scheduled_time.push(time);
        self
    }

    pub fn build(self) -> Result<Medication, DomainError> {
        Ok(Medication::new(
            MedicationId::generate(),
            self.name.ok_or(DomainError::EmptyMedicationName)?,
            self.dosage.ok_or(DomainError::InvalidDosage)?,
            self.scheduled_time,
        ))
    }
}

// Usage
let medication = MedicationBuilder::new()
    .name(MedicationName::new("Aspirin")?)
    .dosage(Dosage::new(500)?)
    .scheduled_time(ScheduledTime::new(8, 0)?)
    .build()?;
```

---

## 8. Bounded Context

> An explicit boundary within which a particular domain model applies. Different contexts
> may model the same concept differently.

In Rust, bounded contexts map to **crates** (for strict separation) or **top-level
modules** with controlled visibility.

### Module-level context separation

```
src/
  medications/      ← Medication Management context
    domain/
    application/
    infrastructure/
  notifications/    ← Notification context (different model of Medication)
    domain/
    application/
    infrastructure/
```

### Context mapping with an Anti-Corruption Layer (ACL)

When two contexts need to communicate, an ACL translates between their models:

```rust
// notifications/infrastructure/acl/medication_acl.rs

use crate::medications::domain::entities::medication::Medication as MedContextMedication;
use crate::notifications::domain::entities::reminder_target::ReminderTarget;

pub struct MedicationAcl;

impl MedicationAcl {
    /// Translates a Medication from the Medication context into a ReminderTarget
    /// understood by the Notification context.
    pub fn to_reminder_target(med: &MedContextMedication) -> ReminderTarget {
        ReminderTarget {
            label: med.name().value().to_owned(),
            times: med.scheduled_time().to_vec(),
        }
    }
}
```

Neither context's domain model leaks into the other.

---

## 9. Ubiquitous Language

> Use the same language in code as domain experts use when speaking. If a pharmacist says
> "schedule a dose", the code should say `schedule_dose`, not `insert_record` or
> `add_item`.

### Naming examples

| Domain expert says | ❌ Generic code name | ✅ Ubiquitous language |
|---|---|---|
| "Mark the dose as taken" | `update_status(true)` | `mark_taken(at)` |
| "Prescribe a medication" | `add_medication()` | `prescribe()` |
| "The dose was missed" | `status = Missed` | `DoseMissed` (event) |
| "Scheduled administration time" | `time_slot` | `ScheduledTime` |
| "The prescribed amount" | `quantity` | `Dosage` |

### Enforce it in types

```rust
// ❌ Generic
pub fn update(&mut self, status: bool) { /* ... */ }

// ✅ Speaks the domain language
pub fn mark_taken(&mut self, at: NaiveDateTime) -> Result<(), DomainError> { /* ... */ }
pub fn mark_missed(&mut self) -> Result<(), DomainError> { /* ... */ }
```

Method names should read like sentences in the domain's vocabulary.

---

## 10. Summary

| DDD Concept | Rust mechanism | Location |
|---|---|---|
| **Value Object** | `struct` with private fields, `PartialEq`, `fn new() -> Result` | `domain/value_objects/` |
| **Entity** | `struct` with `id` field, `&mut self` state transitions | `domain/entities/` |
| **Aggregate Root** | Entity that owns child objects; sole mutation entry point | `domain/entities/` |
| **Domain Event** | `enum` variants, past tense, collected in aggregate | `domain/events.rs` |
| **Repository** | `trait` (port) + concrete `impl` (adapter) | port: `application/ports/`, impl: `infrastructure/persistence/` |
| **Domain Service** | Stateless `struct` with pure functions, no I/O | `domain/services/` |
| **Factory** | Free function or Builder struct for complex creation | `domain/factories/` |
| **Bounded Context** | Crate or top-level module with controlled `pub` visibility | crate root or `src/<context>/` |
| **Ubiquitous Language** | Type names, method names, enum variants match domain vocabulary | everywhere |

### The golden rules

1. **Domain types self-validate** — if a value can be constructed, it is valid.
2. **Aggregates own their boundaries** — external code uses IDs, not references.
3. **Repositories operate on whole aggregates** — never on individual child entities.
4. **Domain and application layers have no I/O** — infrastructure is always behind a trait.
5. **Speak the domain language** — name types and methods after domain concepts, not
   technical operations.
