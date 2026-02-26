# Notification Adapters — `src/infrastructure/notifications/`

Infrastructure implementations of [`NotificationPort`](../application/ports/notification_port.md).

---

## `ConsoleNotificationAdapter` — `console_notification_adapter.rs`

Prints a dose reminder to standard output. Suitable for CLI and TUI delivery.

```
⏰ Time to take Aspirin — 500mg
```

Wired into [`Container::new()`](container.md) as `Arc::new(ConsoleNotificationAdapter)`.

---

## Adding a New Notification Channel

Create a struct in this module and implement `NotificationPort`:

```rust
pub struct DesktopNotificationAdapter;

impl NotificationPort for DesktopNotificationAdapter {
    fn notify_dose_due(
        &self,
        medication: &Medication,
        _record: &DoseRecord,
    ) -> Result<(), NotificationError> {
        // call OS notification API …
        Ok(())
    }
}
```

Then swap it in `Container::new()`. No service code changes needed.
