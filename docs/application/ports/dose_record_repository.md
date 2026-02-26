# `DoseRecordRepository` — `src/application/ports/dose_record_repository.rs`

Port trait that abstracts all persistence operations for
[`DoseRecord`](../../domain/entities/dose_record.md) aggregates.
Defined in the application layer; implemented by infrastructure adapters.

---

## Trait Definition

```rust
pub trait DoseRecordRepository: Send + Sync {
    fn save(&self, record: &DoseRecord) -> Result<(), DoseRecordRepositoryError>;
    fn find_by_id(&self, id: &DoseRecordId)
        -> Result<Option<DoseRecord>, DoseRecordRepositoryError>;
    fn find_all_by_medication(&self, medication_id: &MedicationId)
        -> Result<Vec<DoseRecord>, DoseRecordRepositoryError>;
    fn delete(&self, id: &DoseRecordId) -> Result<(), DoseRecordRepositoryError>;
}
```

---

## Methods

### `save(record: &DoseRecord) -> Result<(), DoseRecordRepositoryError>`

Inserts or updates a `DoseRecord`. Called by
[`MarkDoseTakenService`](../services/mark_dose_taken_service.md) after the
`taken_at` timestamp is set.

### `find_by_id(id: &DoseRecordId) -> Result<Option<DoseRecord>, DoseRecordRepositoryError>`

Loads a single `DoseRecord` by its ID. Used by `MarkDoseTakenService` before
calling `mark_taken`.

### `find_all_by_medication(medication_id: &MedicationId) -> Result<Vec<DoseRecord>, DoseRecordRepositoryError>`

Returns every dose record associated with a given medication.
Useful for displaying a medication's dose history.

### `delete(id: &DoseRecordId) -> Result<(), DoseRecordRepositoryError>`

Removes a dose record from storage.

---

## Error Type

```rust
pub enum DoseRecordRepositoryError {
    NotFound,               // queried resource does not exist
    StorageError(String),   // underlying storage failure
}
```

---

## Current Implementation

`InMemoryDoseRecordRepository` — stores records in a `HashMap` behind a
`Mutex`. Wired in [`Container::new()`](../../infrastructure/container.md).

---

## Role in the Product Flow

When a notification fires and the user taps "taken", the presentation layer
calls `MarkDoseTakenService::execute(record_id, taken_at)`.
That service uses this repository to load the record, mutate it, and save it back.
