# `MedicationRepository` — `src/application/ports/medication_repository.rs`

Port trait that abstracts all persistence operations for
[`Medication`](../../domain/entities/medication.md) aggregate roots.
Defined in the application layer; implemented by infrastructure adapters.

---

## Trait Definition

```rust
pub trait MedicationRepository: Send + Sync {
    fn save(&self, medication: &Medication) -> Result<(), RepositoryError>;
    fn find_by_id(&self, id: &MedicationId) -> Result<Option<Medication>, RepositoryError>;
    fn find_all(&self) -> Result<Vec<Medication>, RepositoryError>;
    fn delete(&self, id: &MedicationId) -> Result<(), RepositoryError>;
}
```

---

## Methods

### `save(medication: &Medication) -> Result<(), RepositoryError>`

Inserts or updates a `Medication`. Called by
[`CreateMedicationService`](../services/create_medication_service.md) after
building the aggregate root.

### `find_by_id(id: &MedicationId) -> Result<Option<Medication>, RepositoryError>`

Returns the medication with the given ID, or `None` if it does not exist.

### `find_all() -> Result<Vec<Medication>, RepositoryError>`

Returns all registered medications. Useful for listing stock on screen.

### `delete(id: &MedicationId) -> Result<(), RepositoryError>`

Removes a medication from storage. Silently succeeds if the ID is not found.

---

## Error Type

```rust
pub enum RepositoryError {
    NotFound,               // queried resource does not exist
    StorageError(String),   // underlying storage failure
}
```

---

## Current Implementation

[`InMemoryMedicationRepository`](../../infrastructure/container.md) — stores
medications in a `HashMap` behind a `Mutex`. Wired in `Container::new()`.

---

## How to Add a New Implementation

1. Create a struct in `src/infrastructure/persistence/`.
2. `impl MedicationRepository for YourStruct { ... }`.
3. Wrap it in `Arc::new(YourStruct::new(...))` inside `Container::new()`.
4. No other code needs to change.
