# `MediationFrequency` -> `src/domain/value_objects/medication_frequency.rs`

A value object representing how often a medication should be taken.

---

## Invariants

- This follow an enum pattern with variants like `OnceDaily`, `TwiceDaily`, `EveryXHours(u8) or Custom(String)`.
- For `EveryXHours`, the number of hours must be between 1 and 24
