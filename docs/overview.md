# BitPill — Product Overview

BitPill is a medication management TUI application. Users register medications
and receive reminders at scheduled times.

> **Note:** Only the TUI is released. REST API is WIP.

---

## Project Structure

```
src/
├── domain/                    # Pure business logic (no I/O)
│   ├── entities/             # Medication, DoseRecord
│   ├── value_objects/        # Dosage, MedicationId, ScheduledTime, etc.
│   └── errors.rs             # Domain errors
│
├── application/              # Use-case orchestration
│   ├── dtos/                 # Request/Response DTOs
│   │   ├── requests.rs       # All request structs
│   │   └── responses.rs      # All response structs
│   ├── ports/                # Port traits
│   │   ├── inbound/          # Use-case ports (CreateMedicationPort, etc.)
│   │   ├── outbound/         # Repository/trait ports
│   │   └── fakes/            # Test doubles
│   ├── services/             # Use-case implementations
│   └── errors.rs             # Application errors
│
├── infrastructure/           # Concrete implementations
│   ├── clock/               # SystemClock
│   ├── notifications/       # ConsoleNotificationAdapter
│   ├── persistence/         # JSON repositories
│   └── container.rs         # Composition root
│
└── presentation/
    └── tui/                 # ratatui UI (this is what's released)
        ├── app.rs           # App state, event loop
        ├── presenters/      # Screen renderers
        ├── handlers/        # Event handlers
        ├── components/      # Reusable widgets
        └── templates/       # Layout templates
```

---

## Product Flow

```
┌─────────────────────────────────────────────────────────────────┐
│  1. User registers a medication                                 │
│     name: "Aspirin", dosage: 500 mg, times: [08:00, 20:00]     │
│                          │                                      │
│                          ▼                                      │
│  2. Medication is validated and persisted                       │
│     CreateMedicationService → MedicationRepository              │
│                          │                                      │
│                          ▼                                      │
│  3. At each scheduled time, a notification is dispatched        │
│     (timer / scheduler triggers for every ScheduledTime)        │
│                          │                                      │
│                          ▼                                      │
│  4. User takes the dose and confirms it                         │
│     MarkDoseTakenService → DoseRecordRepository                 │
│                          │                                      │
│                          ▼                                      │
│  5. DoseRecord is updated: taken_at = now                       │
└─────────────────────────────────────────────────────────────────┘
```

---

## Key Use Cases

### Register a Medication (`CreateMedicationService`)

1. Presentation collects: name, dosage in mg, list of (hour, minute) pairs.
2. Service validates each input via domain value objects.
3. A `Medication` aggregate root is created with a unique ID.
4. The medication is saved via `MedicationRepository`.
5. The saved `Medication` is returned to the caller.

**Error paths**
- Empty/whitespace name → `DomainError::EmptyMedicationName`
- Zero dosage → `DomainError::InvalidDosage`
- Invalid time (hour ≥ 24 or minute ≥ 60) → `DomainError::InvalidScheduledTime`
- Storage failure → `CreateMedicationError::Repository`

---

### Mark a Dose as Taken (`MarkDoseTakenService`)

Triggered when the user confirms they have taken a dose (e.g. after a notification).

1. A `DoseRecordId` and the actual `taken_at` timestamp are provided.
2. The service loads the `DoseRecord` from `DoseRecordRepository`.
3. `DoseRecord::mark_taken(taken_at)` is called — enforces single-take invariant.
4. The updated record is persisted.

**Error paths**
- Record not found → `MarkDoseTakenError::NotFound`
- Already taken → `MarkDoseTakenError::Domain(DoseAlreadyTaken)`
- Storage failure → `MarkDoseTakenError::Repository`

---

## Document Index

| File | Doc |
|---|---|
| `src/domain/errors.rs` | [domain/errors.md](domain/errors.md) |
| `src/domain/entities/medication.rs` | [domain/entities/medication.md](domain/entities/medication.md) |
| `src/domain/entities/dose_record.rs` | [domain/entities/dose_record.md](domain/entities/dose_record.md) |
| `src/domain/value_objects/dosage.rs` | [domain/value_objects/dosage.md](domain/value_objects/dosage.md) |
| `src/domain/value_objects/medication_name.rs` | [domain/value_objects/medication_name.md](domain/value_objects/medication_name.md) |
| `src/domain/value_objects/medication_id.rs` | [domain/value_objects/medication_id.md](domain/value_objects/medication_id.md) |
| `src/domain/value_objects/dose_record_id.rs` | [domain/value_objects/dose_record_id.md](domain/value_objects/dose_record_id.md) |
| `src/domain/value_objects/scheduled_time.rs` | [domain/value_objects/scheduled_time.md](domain/value_objects/scheduled_time.md) |
| `src/application/ports/clock_port.rs` | [application/ports/clock_port.md](application/ports/clock_port.md) |
| `src/application/ports/notification_port.rs` | [application/ports/notification_port.md](application/ports/notification_port.md) |
| `src/application/ports/medication_repository.rs` | [application/ports/medication_repository.md](application/ports/medication_repository.md) |
| `src/application/ports/dose_record_repository.rs` | [application/ports/dose_record_repository.md](application/ports/dose_record_repository.md) |
| `src/application/services/create_medication_service.rs` | [application/services/create_medication_service.md](application/services/create_medication_service.md) |
| `src/application/services/mark_dose_taken_service.rs` | [application/services/mark_dose_taken_service.md](application/services/mark_dose_taken_service.md) |
| `src/application/services/schedule_dose_service.rs` | [application/services/schedule_dose_service.md](application/services/schedule_dose_service.md) |
| `src/infrastructure/container.rs` | [infrastructure/container.md](infrastructure/container.md) |
| `src/infrastructure/clock/system_clock.rs` | [infrastructure/clock.md](infrastructure/clock.md) |
| `src/infrastructure/notifications/console_notification_adapter.rs` | [infrastructure/notifications.md](infrastructure/notifications.md) |
