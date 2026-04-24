# Project Constitution

## Core Principles (Ratified)

1. Library-First (MUST)
	- All feature functionality should be implemented first in library crates (e.g., `big-core`) and be independently testable.

2. CLI Binding (MUST)
	- Public library functionality MUST be exposed via CLI tools where applicable (e.g., `big-cli`) with clear argument validation and exit codes.

3. Test-First (MUST)
	- New features require tests (unit + integration) covering behavior and security edge-cases before or alongside implementation.

4. Integration Testing (SHOULD)
	- Integration tests must validate cross-crate contracts (CLI ↔ library ↔ Tauri) for new features.

5. Safety & Observability (MUST)
	- Security checks (e.g., path sanitization) and clear user-facing error messages are required. Logging or telemetry sufficient for debugging should be added where appropriate.

6. Non-Breaking Additions (MUST)
	- Public API additions should avoid breaking changes; when unavoidable, document and follow semantic versioning.

## Governance
- Amendments require a short PR describing the change and at least one approver from the maintainers.
- Feature implementation must reference the relevant constitution items in its PR description when a principle is affected.

**Version**: 0.1.0 | **Ratified**: 2026-04-24 | **Last Amended**: 2026-04-24
