# `ClockPort` — `src/application/ports/clock_port.rs`

Port trait that abstracts the system clock, making time-dependent services
fully testable without real time progression.

---

## Trait Definition

```rust
pub trait ClockPort: Send + Sync {
    fn now(&self) -> NaiveDateTime;
}
```

---

## Why this exists

`ScheduleDoseService` must compare the current time against each medication's
`ScheduledTime`. If it called `chrono::Local::now()` directly, tests would
become time-dependent and flaky. By injecting `Arc<dyn ClockPort>`, tests
supply a `FakeClock` with a fixed datetime.

---

## Implementations

| Type | Location | Behaviour |
|---|---|---|
| `SystemClock` | `infrastructure/clock/system_clock.rs` | Returns `Local::now().naive_local()` |
| `FakeClock` (test only) | inside `schedule_dose_service.rs` tests | Returns a fixed `NaiveDateTime` |

---

## Related

- [`ScheduleDoseService`](../services/schedule_dose_service.md) — injects `ClockPort`
- [`SystemClock`](../../infrastructure/clock.md)
