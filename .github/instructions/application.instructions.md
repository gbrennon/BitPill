---
applyTo: "src/application/**"
---

# Application Layer Rules

- Application services live in `application/services/`. They orchestrate domain objects and call ports — no business logic of their own.
- Port traits live in `application/ports/`. Every external capability (persistence, notifications, clocks) must be expressed as a `trait` here.
- Services receive dependencies via constructor injection (`Arc<dyn PortTrait>`). Never instantiate concrete adapters inside a service.
- Services must not import anything from `infrastructure` or `presentation`.
- Application errors are separate from domain errors. Define them in the service file or a sibling `errors.rs`.
- Port traits must derive `Send + Sync` bounds so they can be used behind `Arc`.

```rust
pub trait PillRepository: Send + Sync {
    async fn save(&self, pill: &Pill) -> Result<(), RepositoryError>;
    async fn find_by_id(&self, id: &PillId) -> Result<Option<Pill>, RepositoryError>;
    async fn find_all(&self) -> Result<Vec<Pill>, RepositoryError>;
    async fn delete(&self, id: &PillId) -> Result<(), RepositoryError>;
}
```
