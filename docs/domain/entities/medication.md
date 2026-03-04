# `Medication` — `src/domain/entities/medication.rs`

Aggregate root that represents a medication registered by the user.
It groups everything needed to describe *what* is to be taken: identity,
name, dosage, and scheduled administration times.

---

## Fields

| Field | Type | Description |
|---|---|---|
| `id` | `MedicationId` | UUID v7, supplied by the caller on `new`. Unique per instance. |
| `name` | `MedicationName` | Validated, trimmed medication name. |
| `dosage` | `Dosage` | Prescribed amount in milligrams (> 0). |
| `scheduled_time` | `Vec<ScheduledTime>` | Ordered list of daily administration times. May be empty. |

---

## Invariants

- `id` is supplied by the caller — use [`MedicationId::create()`](../value_objects/medication_id.md) to generate a fresh UUID v7.
- `name` and `dosage` are pre-validated value objects; illegal values are
  rejected before `Medication::new` is called.
- `scheduled_time` may be empty (unscheduled/on-demand medication is valid).
- The struct is immutable after creation — no setters are exposed.

---

## Constructor

```rust
pub fn new(
    id: MedicationId,
    name: MedicationName,
    dosage: Dosage,
    scheduled_time: Vec<ScheduledTime>,
) -> Medication
```

Builds a new `Medication` with the supplied `id`.
Generate the identifier with `MedicationId::create()` before calling this:

```rust
use bitpill::domain::{
    entities::medication::Medication,
    value_objects::{
        dosage::Dosage,
        medication_id::MedicationId,
        medication_name::MedicationName,
        scheduled_time::ScheduledTime,
    },
};

let medication = Medication::new(
    MedicationId::create(),
    MedicationName::new("Aspirin").unwrap(),
    Dosage::new(500).unwrap(),
    vec![
        ScheduledTime::new(8, 0).unwrap(),
        ScheduledTime::new(20, 0).unwrap(),
    ],
);
```

---

## Accessors

| Method | Return type | Description |
|---|---|---|
| `id()` | `&MedicationId` | The medication's unique identifier |
| `name()` | `&MedicationName` | The validated name |
| `dosage()` | `&Dosage` | The prescribed dosage |
| `scheduled_time()` | `&[ScheduledTime]` | All scheduled administration times |

---

## Role in the Product Flow

`Medication` is created by [`CreateMedicationService`](../../application/services/create_medication_service.md)
and persisted via [`MedicationRepository`](../../application/ports/medication_repository.md).
Each `ScheduledTime` it carries is the trigger point for dose-reminder notifications.

---

## Related

- [`MedicationId`](../value_objects/medication_id.md)
- [`MedicationName`](../value_objects/medication_name.md)
- [`Dosage`](../value_objects/dosage.md)
- [`ScheduledTime`](../value_objects/scheduled_time.md)
- [`DoseRecord`](dose_record.md) — tracks whether a specific dose was taken
