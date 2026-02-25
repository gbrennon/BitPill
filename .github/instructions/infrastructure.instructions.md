---
applyTo: "src/infrastructure/**"
---

# Infrastructure Layer Rules

- `infrastructure/container.rs` is the **composition root** — the only place that calls `new` on concrete adapters and wires `Arc<dyn Trait>` into services.
- Concrete adapters implement port traits defined in `application/ports/`. They must not be referenced outside `infrastructure` except through their trait.
- Infrastructure errors must not leak into domain or application layers. Map them to the appropriate error type at the boundary.
- Never import `presentation` from `infrastructure`.
- Persistence implementations live in `infrastructure/persistence/`, one file per repository implementation.
