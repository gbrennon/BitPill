# `MedicationName` — `src/domain/value_objects/medication_name.rs`

A validated, trimmed display name for a medication (e.g. `"Aspirin"`).
A value object: two instances with the same string content are equal.

---

## Invariants

- The stored string is **never empty** after trimming.
- **Surrounding whitespace is stripped** on construction — `"  Aspirin  "` → `"Aspirin"`.

---

## Constructor

```rust
pub fn new(name: impl Into<String>) -> Result<MedicationName, DomainError>
```

| Input | Result |
|---|---|
| Non-empty after trim | `Ok(MedicationName)` |
| Empty string `""` | `Err(DomainError::EmptyMedicationName)` |
| Whitespace-only `"   "` | `Err(DomainError::EmptyMedicationName)` |

```rust
use bitpill::domain::value_objects::medication_name::MedicationName;

let name = MedicationName::new("  Ibuprofen  ").unwrap();
assert_eq!(name.value(), "Ibuprofen");   // whitespace trimmed
```

---

## Methods

| Method | Return | Description |
|---|---|---|
| `value()` | `&str` | Returns the stored name as a string slice |
| `to_string()` | `String` | Formats as the name string |

---

## Related

- [`DomainError::EmptyMedicationName`](../errors.md)
- [`Medication`](../entities/medication.md) — carries a `MedicationName`
