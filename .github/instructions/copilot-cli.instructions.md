# Copilot CLI instructions

Purpose
- Rules and expectations for the Copilot CLI agent when making code changes or interacting with this repository.

Placement & filename
- File location: .github/instructions/
- Filename: copilot-cli.instructions.md

Core rules
- Never run or modify git in any form. The agent must not create commits, branches, pull requests, or inspect the .git directory.
- Make the smallest possible code changes required to satisfy a user request. Prefer surgical, single-purpose edits.
- Before changing code, surface a short plan when the change is non-trivial. Keep plans minimal and actionable.
- When modifying files, respect existing repository conventions and instruction files under .github/instructions/.

Architecture & SOLID enforcement
- This repository follows a Hexagonal Architecture (Ports & Adapters). The application layer defines inbound and outbound ports; concrete implementations of outbound ports live in the infrastructure layer; services implement inbound ports and live in the application layer.
- Dependency direction (explicit): the domain layer is the innermost layer and MUST NOT depend on application, infrastructure, or presentation. The application layer may depend only on domain. Presentation and infrastructure layers may depend on application (and therefore domain), but domain and application must never depend on them.
- Practical wording of dependency arrows: presentation -> application -> domain, and infrastructure -> application -> domain (infrastructure implements outbound ports defined by application).
- Presentation layer depends on application inbound ports (it calls services) and may use the dependency-injection/composition root provided by infrastructure to obtain configured services. The composition root lives in src/infrastructure/container.rs.
- Domain and application layers must be pure/agnostic: no file I/O, no network calls, and no direct dependency on infrastructure or presentation modules. These layers must contain only pure business logic and use-case orchestration and may only depend on std and domain-safe crates that do not introduce I/O.
- SOLID principles must never be violated. New or modified code must follow Single Responsibility, Open/Closed, Liskov Substitution, Interface Segregation, and Dependency Inversion design constraints.
- Ports must be defined under application/ports; outbound ports are implemented in src/infrastructure/, services live in src/application/services/, and the composition root is src/infrastructure/container.rs which wires concrete adapters into application services.

Tooling & workflows
- Prefer repository-provided tasks (e.g., just, cargo) for build, test, lint actions: use the existing "just" recipes.
- Do not add or run new global tooling without explicit user approval.
- If tests or formatting steps exist, run them only to validate the work being done and do not alter unrelated files.

Testing & fakes
- Unit tests for domain and application code must use in-memory fakes and must not perform I/O.
- Shared fakes should live under tests/fakes and be gated by #[cfg(any(test, feature = "test-helpers"))] when applicable.
- Integration tests that exercise infrastructure adapters should live under tests/infrastructure and may use real infrastructure.
- End-to-end / presentation integration tests should live under tests/presentation and reflect the presentation surface they exercise.
- Test files placed under tests/ should mirror the path or area of the code they are testing (e.g., tests/infrastructure/* tests infra adapters, tests/presentation/* tests UI/api endpoints).

Communication & intent
- On the first tool-using turn after a user message, report intent using the report_intent tool alongside other tool calls.
- Before invoking tools, state one short sentence describing the action.
- Keep assistant messages concise: prefer 1–3 short sentences when reporting results or asking a clarifying question.

Edits, commits & Git policy
- The agent may create or edit files only when explicitly requested; prefer minimal, surgical edits and avoid widespread refactors.
- The agent MUST NOT perform Git operations, invoke git tooling, or read the .git directory in any form.
- If Git operations are necessary, the agent should (1) explain why they are needed, (2) provide the exact commands for the human to run locally, and (3) ask the user to run them; the agent must not execute those commands itself or attempt to interact with Git programmatically.
- When providing suggested commit messages or git command snippets for the user to run, include the required Co-authored-by trailer in the suggested commit body:
  Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>

Safety & secrets
- Never disclose or commit secrets, credentials, or private data to the repository or external systems.

Exceptions & escalation
- If a task requires Git operations, request the user perform them or provide explicit permission; explain why the agent cannot proceed.
- Ask at most one clarifying question at a time using the ask_user tool when scope is ambiguous.

Maintenance
- Keep this file short and focused; prefer linking to longer policies elsewhere (docs/ or README) for rationale and diagrams.

Last updated: 2026-03-05
