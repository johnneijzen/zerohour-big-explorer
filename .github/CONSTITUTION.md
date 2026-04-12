# ZeroHour BIG Explorer — Constitution

## Purpose
This project is a modern, cross-platform Tauri tool for exploring, extracting, and repacking `.BIG` archive files used by Command & Conquer: Generals – Zero Hour. It provides a fast, dependable, and extensible interface for modders and developers to inspect and manipulate game assets safely.

## Scope
- Browse and inspect `.BIG` archive contents and internal structure.
- Safely extract files with non-destructive defaults and clear destination prompts.
- Repack `.BIG` archives with validation, integrity checks, and optional backups.
- View file metadata and headers for analysis and debugging.

Future extensions may include:
- Model previewers (W3D), texture viewers, and audio playback for common formats.
- Conversion helpers and export pipelines for asset reuse.

## Non-Goals
- Modifying game engine logic or enabling cheats.
- Supporting online multiplayer exploitation.
- Replacing dedicated 3D modeling or audio authoring tools.

## Design Principles
- Performance first: handle large archives efficiently and incrementally.
- Read-only safety by default: avoid accidental writes to original files.
- Modular architecture: clear separation between core parser, IO layer, UI, and preview plugins.
- Deterministic parsing and testability: make parsing logic simple to reason about and unit-test.
- Extensibility: plugin-friendly parsers and preview handlers for new asset types.

## File Safety Rules
- Never overwrite original `.BIG` files without explicit user confirmation.
- Recommend and create backups before repacking or destructive operations.
- Validate archive structure and file checksums before any write operations.
- Provide safe extraction defaults (extract to separate folders, preserve timestamps).
- Offer explicit checksums, dry-run modes, and verify steps for repacking operations.

## Code Philosophy
- Keep parsing logic deterministic and small, prefer correctness over cleverness.
- Avoid undocumented game-specific hacks; document and gate exceptions when necessary.
- Maintain a strict separation of concerns: `parser`, `io`, `ui`, `preview`.
- Write unit and integration tests for all parsing and IO operations.
- Prioritize maintainability: clear APIs, consistent error handling, and minimal global state.

## Modding Ethics
This tool exists for legitimate modding, learning, and archival purposes. It must not be used for cheating, piracy facilitation, or malicious exploitation. Contributors should avoid enabling workflows that facilitate unethical behavior.

---
For contributions, follow the repository's contribution guidelines and include tests for parser or IO changes.
