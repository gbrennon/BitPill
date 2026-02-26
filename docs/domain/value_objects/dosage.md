# `Dosage` — `src/domain/value_objects/dosage.rs`

Represents the prescribed amount of a medication in milligrams.
A value object: two instances with the same `amount_mg` are equal.

---

## Invariant

`amount_mg` must be **greater than zero**. A zero dosage is meaningless and
is rejected at construction time.

---

## Constructor

```rust
pub fn new(amount_mg: u32) -> Result<Dosage, DomainError>
```

| Input | Result |
|---|---|
| `amount_mg > 0` | `Ok(Dosage)` |
| `amount_mg == 0` | `Err(DomainError::InvalidDosage)` |

```rust
use bitpill::domain::{value_objects::dosage::Dosage, errors::DomainError};

let ok  = Dosage::new(500);   // Ok(Dosage { amount_mg: 500 })
let err = Dosage::new(0);     // Err(InvalidDosage)
```

---

## Methods

| Method | Return | Description |
|---|---|---|
| `amount_mg()` | `u32` | Returns the dosage in milligrams |
| `to_string()` | `String` | Formats as `"500mg"` |

---

## Related

- [`DomainError::InvalidDosage`](../errors.md)
- [`Medication`](../entities/medication.md) — carries a `Dosage`
