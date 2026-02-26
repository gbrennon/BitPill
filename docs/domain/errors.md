# `DomainError` — `src/domain/errors.rs`

All domain-level errors in one enum. Every public constructor and mutating method
that enforces a business rule returns `Result<_, DomainError>`.

---

## Variants

| Variant | Raised by | Rule violated |
|---|---|---|
| `InvalidDosage` | `Dosage::new` | Amount must be > 0 mg |
| `EmptyMedicationName` | `MedicationName::new` | Name must not be empty or whitespace-only |
| `InvalidScheduledTime` | `ScheduledTime::new` | Hour must be 0–23, minute 0–59 |
| `DoseAlreadyTaken` | `DoseRecord::mark_taken` | A dose can only be marked taken once |

---

## Usage Pattern

```rust
use bitpill::domain::errors::DomainError;

match DomainError::InvalidDosage {
    DomainError::InvalidDosage       => { /* dosage was 0 */ }
    DomainError::EmptyMedicationName => { /* name was blank */ }
    DomainError::InvalidScheduledTime => { /* hour/minute out of range */ }
    DomainError::DoseAlreadyTaken    => { /* tried to mark taken twice */ }
}
```

All variants implement `std::error::Error` via [`thiserror`](https://docs.rs/thiserror).
Application-layer error enums wrap `DomainError` with `#[from]` to propagate it
transparently (see [`CreateMedicationError`](../application/services/create_medication_service.md)
and [`MarkDoseTakenError`](../application/services/mark_dose_taken_service.md)).
