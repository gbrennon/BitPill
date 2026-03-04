# SOLID Principles in Rust

A practical guide to applying the five SOLID design principles using Rust's type system,
traits, and module model.

---

## S — Single Responsibility Principle

> A type should have only one reason to change.

Each struct, trait, and module should own exactly one concern. Rust enforces this naturally
through its module visibility rules and the expectation of one primary type per file.

### ❌ Violation

```rust
struct UserService {
    db: DbConnection,
}

impl UserService {
    fn create_user(&self, name: &str) { /* saves to DB */ }
    fn send_welcome_email(&self, email: &str) { /* sends SMTP */ }
    fn hash_password(&self, raw: &str) -> String { /* hashes */ }
}
```

`UserService` has three reasons to change: storage strategy, notification strategy, and
hashing algorithm.

### ✅ Applying SRP

```rust
struct UserRepository { db: DbConnection }
struct EmailNotifier  { smtp: SmtpClient  }
struct PasswordHasher;

impl UserRepository { fn save(&self, user: &User) { /* ... */ } }
impl EmailNotifier  { fn notify(&self, email: &str) { /* ... */ } }
impl PasswordHasher { fn hash(&self, raw: &str) -> String { /* ... */ } }
```

Each type has one reason to change. Composing them in a use-case service keeps
orchestration separate from any single concern.

### Module layout

```
src/
  domain/
    user.rs          ← entity + invariants only
    password.rs      ← hashing value object
  application/
    create_user.rs   ← orchestration use-case
  infrastructure/
    postgres_users.rs
    smtp_notifier.rs
```

### Signals of a violation

| Smell | Fix |
|---|---|
| Struct with 10+ methods across concerns | Split into focused types |
| `impl` block mixing domain logic and I/O | Extract an infrastructure adapter |
| Single error enum spanning all layers | One error type per layer |
| Module file containing unrelated types | One primary type per file |

---

## O — Open/Closed Principle

> Software entities should be open for extension but closed for modification.

Prefer abstractions (traits) over concrete types. Add new behaviour by implementing a
trait — never by modifying existing code.

### ❌ Violation

```rust
fn send_notification(channel: &str, message: &str) {
    match channel {
        "email" => send_email(message),
        "sms"   => send_sms(message),
        // Adding "push" requires modifying this function
        _ => {}
    }
}
```

Every new channel requires modifying `send_notification`, risking regressions.

### ✅ Applying OCP

```rust
pub trait Notifier: Send + Sync {
    fn notify(&self, message: &str) -> Result<(), NotifierError>;
}

pub struct EmailNotifier { /* ... */ }
pub struct SmsNotifier   { /* ... */ }
pub struct PushNotifier  { /* ... */ }  // ← extension, no modification

impl Notifier for EmailNotifier { fn notify(&self, msg: &str) -> Result<(), NotifierError> { /* ... */ } }
impl Notifier for SmsNotifier   { fn notify(&self, msg: &str) -> Result<(), NotifierError> { /* ... */ } }
impl Notifier for PushNotifier  { fn notify(&self, msg: &str) -> Result<(), NotifierError> { /* ... */ } }

fn send_notification(notifier: &dyn Notifier, message: &str) {
    notifier.notify(message).ok();
}
```

Adding a new channel is a new `impl` block — existing code is untouched.

### Enum extension via trait objects

When you need runtime dispatch over a closed set, enums work well. For open-ended
extension across crates, prefer `Box<dyn Trait>` or `Arc<dyn Trait>`.

---

## L — Liskov Substitution Principle

> Subtypes must be substitutable for their base types without altering correctness.

In Rust this applies to trait implementations: every type implementing a trait must honour
the contract the trait implies, including preconditions and postconditions.

### ❌ Violation

```rust
pub trait Clock {
    fn now(&self) -> DateTime<Utc>;
}

struct BrokenClock;

impl Clock for BrokenClock {
    fn now(&self) -> DateTime<Utc> {
        // Returns a time in the past — violates the implicit contract
        // that `now()` returns the current instant
        DateTime::<Utc>::MIN_UTC
    }
}
```

Callers depending on `Clock` cannot substitute `BrokenClock` without broken behaviour.

### ✅ Applying LSP

```rust
pub trait Clock: Send + Sync {
    /// Returns the current UTC instant.
    fn now(&self) -> DateTime<Utc>;
}

pub struct SystemClock;
pub struct FakeClock { pub fixed: DateTime<Utc> }

impl Clock for SystemClock {
    fn now(&self) -> DateTime<Utc> { Utc::now() }
}

impl Clock for FakeClock {
    fn now(&self) -> DateTime<Utc> { self.fixed }  // still a valid instant
}
```

`FakeClock` is a correct substitute: it returns a valid `DateTime<Utc>` and callers
need not know which implementation they hold.

### LSP checklist for trait implementations

- Never panic where the trait signature says `Result` or `Option`.
- Honour documented invariants (ordering, range, monotonicity).
- Do not strengthen preconditions or weaken postconditions.

---

## I — Interface Segregation Principle

> Clients must not be forced to depend on methods they do not use.

Prefer several narrow traits over one large "fat" trait. Rust's trait system makes this
cost-free — types can implement as many focused traits as needed.

### ❌ Violation

```rust
pub trait MedicationRepository: Send + Sync {
    async fn save(&self, med: &Medication) -> Result<(), RepositoryError>;
    async fn find_by_id(&self, id: &MedicationId) -> Result<Option<Medication>, RepositoryError>;
    async fn find_all(&self) -> Result<Vec<Medication>, RepositoryError>;
    async fn delete(&self, id: &MedicationId) -> Result<(), RepositoryError>;
    async fn export_csv(&self) -> Result<String, RepositoryError>;   // ← unrelated concern
    async fn send_reminder(&self, id: &MedicationId) -> Result<(), RepositoryError>; // ← wrong layer
}
```

A read-only use-case is now forced to depend on `export_csv` and `send_reminder`.

### ✅ Applying ISP

```rust
pub trait MedicationReader: Send + Sync {
    async fn find_by_id(&self, id: &MedicationId) -> Result<Option<Medication>, RepositoryError>;
    async fn find_all(&self) -> Result<Vec<Medication>, RepositoryError>;
}

pub trait MedicationWriter: Send + Sync {
    async fn save(&self, med: &Medication) -> Result<(), RepositoryError>;
    async fn delete(&self, id: &MedicationId) -> Result<(), RepositoryError>;
}

pub trait MedicationExporter: Send + Sync {
    async fn export_csv(&self) -> Result<String, ExportError>;
}
```

Each use-case depends only on the interface slice it actually needs:

```rust
struct ListMedicationsService {
    reader: Arc<dyn MedicationReader>,   // not MedicationWriter
}
```

A concrete repository can implement all three traits without forcing callers to see all of them.

---

## D — Dependency Inversion Principle

> High-level modules must not depend on low-level modules. Both must depend on abstractions.

In Rust this means: application services depend on `Arc<dyn PortTrait>` injected at
construction time, never on concrete infrastructure types.

### ❌ Violation

```rust
use crate::infrastructure::PostgresMedicationRepository; // ← concrete import

struct CreateMedicationService {
    repo: PostgresMedicationRepository,  // ← bound to Postgres forever
}

impl CreateMedicationService {
    fn new() -> Self {
        Self { repo: PostgresMedicationRepository::connect("...") } // ← hidden dependency
    }
}
```

The application service is coupled to Postgres. Swapping storage or testing in isolation
requires changing the service itself.

### ✅ Applying DIP

```rust
// application/ports/medication_repository.rs
pub trait MedicationRepository: Send + Sync {
    async fn save(&self, med: &Medication) -> Result<(), RepositoryError>;
    async fn find_by_id(&self, id: &MedicationId) -> Result<Option<Medication>, RepositoryError>;
}

// application/services/create_medication.rs
pub struct CreateMedicationService {
    repo: Arc<dyn MedicationRepository>,  // depends on abstraction
}

impl CreateMedicationService {
    pub fn new(repo: Arc<dyn MedicationRepository>) -> Self {
        Self { repo }
    }
}

// infrastructure/container.rs  ← the only place that wires concrete types
pub fn build_container() -> CreateMedicationService {
    let repo = Arc::new(PostgresMedicationRepository::new(/* config */));
    CreateMedicationService::new(repo)
}
```

The service never knows about Postgres. The composition root (`container.rs`) is the
single place that wires abstractions to concrete adapters.

### Testing benefit

```rust
#[cfg(test)]
mod tests {
    use crate::application::ports::fakes::FakeMedicationRepository;

    #[tokio::test]
    async fn create_medication_saves_to_repository() {
        let repo = Arc::new(FakeMedicationRepository::new());
        let service = CreateMedicationService::new(repo.clone());

        service.execute(CreateMedicationCommand { /* ... */ }).await.unwrap();

        assert_eq!(repo.count(), 1);
    }
}
```

No database, no network, no filesystem — pure logic tested in microseconds.

---

## Summary

| Principle | Rust mechanism | Key rule |
|---|---|---|
| **SRP** | Structs, modules, one file per type | One reason to change |
| **OCP** | Traits + `impl` blocks | Extend by adding, not modifying |
| **LSP** | Trait contracts | Every `impl` must honour the full contract |
| **ISP** | Multiple narrow traits | Depend only on what you use |
| **DIP** | `Arc<dyn Trait>` + constructor injection | High-level code never imports concrete adapters |

Rust's ownership model, trait system, and strict module visibility make SOLID principles
not just achievable but enforceable at compile time.
