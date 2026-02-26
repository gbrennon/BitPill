# `MarkDoseTakenService` — `src/application/services/mark_dose_taken_service.rs`

Use-case service that records the moment a user takes a scheduled dose.
Triggered when the user confirms a dose-reminder notification.

---

## Responsibility

Loads the `DoseRecord`, calls `mark_taken`, and persists the updated record.
Contains no I/O — all side effects go through the injected repository.

---

## Constructor

```rust
pub fn new(repository: Arc<dyn DoseRecordRepository>) -> MarkDoseTakenService
```

---

## `execute` Method

```rust
pub fn execute(
    &self,
    record_id: &DoseRecordId,
    taken_at: NaiveDateTime,
) -> Result<DoseRecord, MarkDoseTakenError>
```

### Steps

```
1. repository.find_by_id(record_id)   → load record (None → NotFound error)
2. record.mark_taken(taken_at)         → enforce single-take invariant
3. repository.save(&record)            → persist updated state
4. Return Ok(record)
```

### Error Type

```rust
pub enum MarkDoseTakenError {
    NotFound,                              // DoseRecord with that ID does not exist
    Domain(DomainError),                   // dose was already taken
    Repository(DoseRecordRepositoryError), // storage failure
}
```

| Situation | Error |
|---|---|
| Unknown `record_id` | `NotFound` |
| Dose already marked taken | `Domain(DoseAlreadyTaken)` |
| Storage layer failure | `Repository(StorageError(...))` |

---

## Example

```rust
use std::sync::Arc;
use bitpill::application::services::mark_dose_taken_service::MarkDoseTakenService;
use bitpill::domain::{
    entities::dose_record::DoseRecord,
    value_objects::medication_id::MedicationId,
};
use chrono::Utc;

// Assuming a DoseRecord was previously saved to the repository:
let service   = MarkDoseTakenService::new(repo.clone());
let taken_at  = Utc::now().naive_utc();
let result    = service.execute(&record_id, taken_at);

assert!(result.unwrap().is_taken());
```

---

## Role in the Product Flow

```
Notification fires at ScheduledTime
        │
        ▼
User taps "I took it"
        │
        ▼
Presentation calls execute(record_id, now)
        │
        ├─ record not found → show error
        ├─ already taken    → show error
        └─ success          → DoseRecord.taken_at = now, notify user
```

---

## Related

- [`DoseRecord`](../../domain/entities/dose_record.md)
- [`DoseRecordRepository`](../ports/dose_record_repository.md)
- [`DomainError`](../../domain/errors.md)
- [`Container`](../../infrastructure/container.md) — wires this service
