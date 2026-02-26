# `DoseRecord` — `src/domain/entities/dose_record.rs`

Tracks whether a single scheduled dose of a medication was actually taken.
It starts in an **untaken** state and transitions to **taken** exactly once.

---

## State Machine

```
       new()
         │
         ▼
   ┌──────────┐     mark_taken(at)     ┌─────────┐
   │ untaken  │ ──────────────────────▶ │  taken  │
   │taken_at  │                         │taken_at │
   │= None    │                         │= Some(t)│
   └──────────┘                         └─────────┘
                                              │
                                    mark_taken again
                                              │
                                              ▼
                                    Err(DoseAlreadyTaken)
```

---

## Fields

| Field | Type | Description |
|---|---|---|
| `id` | `DoseRecordId` | UUID v7, auto-generated on `new`. |
| `medication_id` | `MedicationId` | References the parent `Medication`. |
| `scheduled_at` | `NaiveDateTime` | When the dose was due. |
| `taken_at` | `Option<NaiveDateTime>` | When the user took the dose. `None` until marked. |

---

## Invariants

- `taken_at` starts as `None`.
- `taken_at` can be set **once only** — calling `mark_taken` a second time
  returns [`DomainError::DoseAlreadyTaken`](../errors.md).
- `id` is generated internally; callers cannot supply it.

---

## Constructor

```rust
pub fn new(medication_id: MedicationId, scheduled_at: NaiveDateTime) -> DoseRecord
```

Creates an untaken dose record. Typically called by a scheduler when it
generates a new dose slot for a registered medication.

```rust
use bitpill::domain::{
    entities::dose_record::DoseRecord,
    value_objects::medication_id::MedicationId,
};
use chrono::NaiveDate;

let scheduled_at = NaiveDate::from_ymd_opt(2025, 6, 1)
    .unwrap()
    .and_hms_opt(8, 0, 0)
    .unwrap();

let record = DoseRecord::new(MedicationId::create(), scheduled_at);
assert!(!record.is_taken());
```

---

## Methods

### `mark_taken(at: NaiveDateTime) -> Result<(), DomainError>`

Transitions the record from **untaken** to **taken**.

```rust
record.mark_taken(taken_at).unwrap();
assert!(record.is_taken());
assert_eq!(record.taken_at(), Some(taken_at));
```

**Error:** returns `Err(DomainError::DoseAlreadyTaken)` if already taken.

---

### `is_taken() -> bool`

Returns `true` after `mark_taken` has been called successfully.

---

## Accessors

| Method | Return type | Description |
|---|---|---|
| `id()` | `&DoseRecordId` | Unique identifier of this record |
| `medication_id()` | `&MedicationId` | ID of the parent medication |
| `scheduled_at()` | `NaiveDateTime` | When this dose was due |
| `taken_at()` | `Option<NaiveDateTime>` | When it was taken, or `None` |

---

## Role in the Product Flow

`DoseRecord` is loaded and updated by
[`MarkDoseTakenService`](../../application/services/mark_dose_taken_service.md)
when the user confirms a dose. It is persisted via
[`DoseRecordRepository`](../../application/ports/dose_record_repository.md).

---

## Related

- [`DoseRecordId`](../value_objects/dose_record_id.md)
- [`MedicationId`](../value_objects/medication_id.md)
- [`Medication`](medication.md) — the medication this record belongs to
