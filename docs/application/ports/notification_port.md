# `NotificationPort` — `src/application/ports/notification_port.rs`

Port trait that abstracts delivery of dose-reminder notifications to the user.
Implementations can target any channel: console, desktop popup, push notification, email, etc.

---

## Trait Definition

```rust
pub trait NotificationPort: Send + Sync {
    fn notify_dose_due(
        &self,
        medication: &Medication,
        record: &DoseRecord,
    ) -> Result<(), NotificationError>;
}
```

---

## Error Type

```rust
pub enum NotificationError {
    DeliveryError(String),   // could not reach the notification channel
}
```

---

## Implementations

| Type | Location | Behaviour |
|---|---|---|
| `ConsoleNotificationAdapter` | `infrastructure/notifications/console_notification_adapter.rs` | Prints `⏰ Time to take {name} — {dosage}` to stdout |
| `FakeNotificationPort` (test only) | inside `schedule_dose_service.rs` tests | Records calls for assertion |

---

## How to add a new channel

1. Create a struct in `src/infrastructure/notifications/`.
2. `impl NotificationPort for YourAdapter { ... }`.
3. Swap it into `Container::new()`.

No service code changes required.

---

## Related

- [`ScheduleDoseService`](../services/schedule_dose_service.md) — injects `NotificationPort`
- [`ConsoleNotificationAdapter`](../../infrastructure/notifications.md)
