# Clock Adapters — `src/infrastructure/clock/`

Infrastructure implementations of [`ClockPort`](../application/ports/clock_port.md).

---

## `SystemClock` — `system_clock.rs`

Production adapter. Returns the host's current local time via `chrono::Local::now()`.

```rust
use bitpill::infrastructure::clock::system_clock::SystemClock;

let clock = SystemClock;
println!("{}", clock.now()); // e.g. "2025-06-01 08:00:00"
```

Wired into [`Container::new()`](container.md) as `Arc::new(SystemClock)`.

---

## Swapping the Clock

To use UTC instead of local time, create a `UtcClock` adapter:

```rust
pub struct UtcClock;
impl ClockPort for UtcClock {
    fn now(&self) -> NaiveDateTime { Utc::now().naive_utc() }
}
```

Update `Container::new()` — no other code changes required.
