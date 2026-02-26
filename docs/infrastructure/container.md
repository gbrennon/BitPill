# `Container` — `src/infrastructure/container.rs`

The **composition root** of the application. The only place that instantiates
concrete infrastructure adapters and wires them into application services via
`Arc<dyn Trait>`.

No other module should call `new()` on a concrete adapter directly.

---

## Fields

| Field | Type | Description |
|---|---|---|
| `create_medication_service` | `CreateMedicationService` | Register a new medication |
| `mark_dose_taken_service` | `MarkDoseTakenService` | Confirm a dose was taken |

---

## Wiring

```
Container::new()
  │
  ├─ Arc<InMemoryMedicationRepository>  ──▶  CreateMedicationService
  └─ Arc<InMemoryDoseRecordRepository>  ──▶  MarkDoseTakenService
```

All services receive their dependencies as `Arc<dyn Trait>` — they never
know the concrete type.

---

## Usage

```rust
use bitpill::infrastructure::container::Container;

let container = Container::new();

// Register a medication
let med = container
    .create_medication_service
    .execute("Aspirin", 500, vec![(8, 0), (20, 0)])
    .unwrap();

// Later: mark a dose as taken
// container.mark_dose_taken_service.execute(&record_id, taken_at).unwrap();
```

---

## Extending the Container

To add a new use-case service:

1. Define a port trait in `src/application/ports/`.
2. Implement it in `src/infrastructure/persistence/` (or another adapter module).
3. Add the `Arc<ConcreteImpl>` and the service as fields in `Container`.
4. Wire them inside `Container::new()`.

Nothing outside `container.rs` needs to change.

---

## Related

- [`CreateMedicationService`](../application/services/create_medication_service.md)
- [`MarkDoseTakenService`](../application/services/mark_dose_taken_service.md)
- [`MedicationRepository`](../application/ports/medication_repository.md)
- [`DoseRecordRepository`](../application/ports/dose_record_repository.md)
