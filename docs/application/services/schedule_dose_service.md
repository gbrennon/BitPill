# `ScheduleDoseService` — `src/application/services/schedule_dose_service.rs`

Use-case service that drives the timer-based notification flow.
Call `execute()` once per minute to check which medications are due and
notify the user.

---

## Responsibility

For each registered medication, if the current clock time matches one of its
[`ScheduledTime`](../../domain/value_objects/scheduled_time.md) values, the
service:

1. Creates a [`DoseRecord`](../../domain/entities/dose_record.md) for that slot.
2. Persists the record via [`DoseRecordRepository`](../ports/dose_record_repository.md).
3. Fires a notification via [`NotificationPort`](../ports/notification_port.md).

---

## Constructor

```rust
pub fn new(
    medication_repository: Arc<dyn MedicationRepository>,
    dose_record_repository: Arc<dyn DoseRecordRepository>,
    notification_port: Arc<dyn NotificationPort>,
    clock: Arc<dyn ClockPort>,
) -> ScheduleDoseService
```

All four dependencies are injected — the service owns no concrete types.

---

## `execute` Method

```rust
pub fn execute(&self) -> Result<Vec<DoseRecord>, ScheduleDoseError>
```

Returns every `DoseRecord` created during this tick.
An empty `Vec` means no medication was due at the current minute.

### Error Type

```rust
pub enum ScheduleDoseError {
    MedicationRepository(RepositoryError),
    DoseRecordRepository(DoseRecordRepositoryError),
    Notification(NotificationError),
}
```

---

## Flow Diagram

```
execute() called (once per minute)
        │
        ├─ clock.now()                    → current NaiveDateTime
        ├─ medication_repo.find_all()      → all Medication records
        │
        └─ for each Medication:
               if ScheduledTime matches now.time()
                 │
                 ├─ DoseRecord::new(medication_id, now)
                 ├─ dose_record_repo.save(&record)
                 └─ notification_port.notify_dose_due(&medication, &record)
```

---

## Test Coverage

All branches are covered by unit tests using in-memory fakes (no I/O):

| Test | Verifies |
|---|---|
| `execute_with_no_medications_returns_empty_vec` | Empty store produces no records |
| `execute_with_matching_time_creates_dose_record_and_notifies` | Happy path |
| `execute_with_non_matching_time_creates_no_records` | Time mismatch is ignored |
| `execute_notifies_only_medications_due_at_current_time` | Partial match in multi-medication list |
| `execute_notifies_all_medications_due_at_same_time` | Multiple matches in one tick |
| `execute_created_record_links_to_correct_medication` | `medication_id` is preserved |
| `execute_created_record_scheduled_at_matches_clock_now` | `scheduled_at` comes from clock |
| `execute_medication_with_no_scheduled_times_is_ignored` | On-demand medications skipped |

---

## Related

- [`ClockPort`](../ports/clock_port.md)
- [`NotificationPort`](../ports/notification_port.md)
- [`DoseRecord`](../../domain/entities/dose_record.md)
- [`MarkDoseTakenService`](mark_dose_taken_service.md) — user confirms the notified dose
- [`Container`](../../infrastructure/container.md) — wires this service
