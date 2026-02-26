# `CreateMedicationService` — `src/application/services/create_medication_service.rs`

Use-case service that registers a new medication into the system.
This is the **primary entry point** for the "register a medication" product flow.

---

## Responsibility

Validates user input, constructs the `Medication` aggregate root, and persists
it. Contains no I/O — all side effects are delegated to the injected repository.

---

## Constructor

```rust
pub fn new(repository: Arc<dyn MedicationRepository>) -> CreateMedicationService
```

`repository` is injected — the service never instantiates storage directly.

---

## `execute` Method

```rust
pub fn execute(
    &self,
    name: impl Into<String>,
    amount_mg: u32,
    scheduled_times: Vec<(u32, u32)>,  // (hour, minute) pairs
) -> Result<Medication, CreateMedicationError>
```

### Steps

```
1. MedicationName::new(name)         → validates non-empty
2. Dosage::new(amount_mg)            → validates > 0
3. ScheduledTime::new(h, m) × N      → validates each time
4. Medication::new(name, dosage, times)   → builds aggregate, generates UUID
5. repository.save(&medication)       → persists
6. Return Ok(medication)
```

### Error Type

```rust
pub enum CreateMedicationError {
    Domain(DomainError),         // validation failed
    Repository(RepositoryError), // storage failed
}
```

| Situation | Error |
|---|---|
| Empty/whitespace name | `Domain(EmptyMedicationName)` |
| Zero dosage | `Domain(InvalidDosage)` |
| `hour >= 24` or `minute >= 60` | `Domain(InvalidScheduledTime)` |
| Storage layer failure | `Repository(StorageError(...))` |

---

## Example

```rust
use std::sync::Arc;
use bitpill::application::services::create_medication_service::CreateMedicationService;
use bitpill::infrastructure::persistence::in_memory_medication_repository::InMemoryMedicationRepository;

let repo    = Arc::new(InMemoryMedicationRepository::new());
let service = CreateMedicationService::new(repo);

let med = service
    .execute("Aspirin", 500, vec![(8, 0), (20, 0)])
    .unwrap();

println!("{} — {}", med.name(), med.dosage()); // "Aspirin — 500mg"
```

---

## Related

- [`Medication`](../../domain/entities/medication.md)
- [`MedicationRepository`](../ports/medication_repository.md)
- [`DomainError`](../../domain/errors.md)
- [`Container`](../../infrastructure/container.md) — wires this service
