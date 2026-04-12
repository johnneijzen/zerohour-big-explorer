Decision: Use Rust for `big-core`, Tauri + SvelteKit for GUI, and a Rust CLI.

Rationale:
- Rust provides safe, high-performance, deterministic archive processing.
- Single shared `big-core` avoids duplicated parsing/repacking logic and enables identical behavior across CLI and GUI.
- Tauri + SvelteKit gives a small, native-feel desktop app with web UI tech and allows calling Rust commands directly.
- Streaming-first design prevents UI-blocking and supports large archives.

Alternatives considered:
- Implement core in C++ (rejected: higher maintenance, fewer Rust ecosystem benefits).
- Implement GUI in Electron (rejected: larger bundle, higher memory).

Decisions captured:
- `big-core` = Rust library, UI-agnostic, fully tested.
- `big-cli` = Rust standalone binary exposing list/extract/pack/validate.
- `big-tauri` = Tauri desktop app (SvelteKit + shadcn-svelte) calling `big-core` for all logic.

Next steps from research:
- Document data model and public contracts so CLI and Tauri implementations stay in sync.
