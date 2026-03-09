# `MedicationFrequency` -> `src/domain/value_objects/medication_frequency.rs`

A value object representing how often a medication should be taken.

---

## Invariants

- This follow an enum pattern with variants like `OnceDaily`, `TwiceDaily`, `EveryXHours(u8) or Custom(String)`.
- For `EveryXHours`, the number of hours must be between 1 and 24


## Usage

```rust
use bitpill::domain::value_objects::medication_frequency::MedicationFrequency;

let freq1 = MedicationFrequency::OnceDaily;
let freq2 = MedicationFrequency::EveryXHours(8);
let freq3 = MedicationFrequency::Custom("Every Monday".to_string());

assert_eq!(freq1.to_string(), "Once daily");
assert_eq!(freq2.to_string(), "Every 8 hours");
assert_eq!(freq3.to_string(), "Every Monday");
```

`MedicationFrequency` is used as a field in the [`Medication`](../entities/medication.md) entity to specify how often the medication should be taken.
