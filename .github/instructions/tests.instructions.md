---
applyTo: "**/*_test.rs, **/tests/**"
---

# Test Conventions

- Follow **Arrange / Act / Assert** (AAA) structure with a blank line between each section.
- One logical assertion per test — a test should have exactly one reason to fail.
- Name tests using: `method_name_state_under_test_expected_behavior` (e.g., `new_with_zero_dosage_returns_error`).
- Unit tests for domain and application code must use in-memory fakes, never real I/O or DB.
- Place unit tests in a `#[cfg(test)]` module at the bottom of the file under test.
- Integration tests go in `tests/` at the crate root and may use real infrastructure.
- Test domain invariants and observable behaviour — never test private implementation details.
- Fakes and stubs for port traits live in `src/application/ports/fakes.rs` — use them instead of defining inline fakes.
