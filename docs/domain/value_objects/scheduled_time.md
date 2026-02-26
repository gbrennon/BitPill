# `ScheduledTime` — `src/domain/value_objects/scheduled_time.rs`

A validated clock time (hour + minute) at which a medication dose is due.
Wraps `chrono::NaiveTime` with seconds fixed at zero.
A value object: sortable and comparable, so a medication's schedule can be
ordered chronologically.

---

## Invariants

- `hour` must be in `0..=23`.
- `minute` must be in `0..=59`.

---

## Constructor

```rust
pub fn new(hour: u32, minute: u32) -> Result<ScheduledTime, DomainError>
```

| Input | Result |
|---|---|
| Valid hour & minute | `Ok(ScheduledTime)` |
| `hour >= 24` | `Err(DomainError::InvalidScheduledTime)` |
| `minute >= 60` | `Err(DomainError::InvalidScheduledTime)` |

```rust
use bitpill::domain::value_objects::scheduled_time::ScheduledTime;

let morning = ScheduledTime::new(8, 0).unwrap();
let evening = ScheduledTime::new(20, 30).unwrap();

assert_eq!(morning.to_string(), "08:00");
assert!(morning < evening);             // ordering is supported
```

---

## Methods

| Method | Return | Description |
|---|---|---|
| `value()` | `NaiveTime` | Returns the underlying `chrono::NaiveTime` |
| `to_string()` | `String` | Formats as `"HH:MM"` (zero-padded) |

---

## Role in the Product Flow

Each `ScheduledTime` stored in a `Medication` is the trigger point for a
dose-reminder notification. When the system clock reaches a `ScheduledTime`,
a `DoseRecord` is created and the user is notified.

---

## Related

- [`DomainError::InvalidScheduledTime`](../errors.md)
- [`Medication`](../entities/medication.md) — carries `Vec<ScheduledTime>`
