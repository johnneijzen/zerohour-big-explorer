# Tasks: BIG Archive Interactive Browsing & Extraction

## Phase 1 — Setup

- [ ] T001 Initialize feature branch and workspace scaffolding in specs/002-big-archive-extraction
- [ ] T002 [P] Add CI cargo test entry for feature in Cargo.toml (workspace root)
- [ ] T003 Create feature integration test harness in tests/integration/big-archive-extraction.rs

## Phase 2 — Foundational (big-core library)

- [ ] T004 Add `extract_file` API in big-core/src/extract.rs
- [ ] T005 Add `extract_to_path` API in big-core/src/extract.rs
- [ ] T006 Add `extract_all` API in big-core/src/extract.rs
- [ ] T007 Add `pack_directory` API in big-core/src/pack.rs
- [ ] T008 Add `append_file_to_archive` API in big-core/src/pack.rs
- [ ] T009 Implement path sanitization helpers in big-core/src/paths.rs
- [ ] T010 [P] Add unit tests for streaming extraction and path-sanitization in big-core/tests/

## Phase 3 — User Stories (priority order)

**User Story 1 — Browse the archive (UI file list & metadata)**

- [ ] T011 [US1] Render clickable file list in big-tauri/src/lib/ui/file_list.svelte
- [ ] T012 [US1] Implement Tauri command to fetch archive index in big-tauri/src-tauri/src/commands.rs
- [ ] T013 [US1] Wire Tauri command to `big-core` index iterator in big-tauri/src-tauri/src/commands.rs
- [ ] T014 [US1] Add integration test: open archive and assert file list loads in tests/integration/big-archive-extraction.rs

**User Story 2 — Extract a single file (UI)**

- [ ] T015 [US2] Add `extract_file_bytes` Tauri command in big-tauri/src-tauri/src/commands.rs
- [ ] T016 [US2] Add `extract_file_to_disk` Tauri command in big-tauri/src-tauri/src/commands.rs
- [ ] T017 [US2] Implement frontend Extract action and save dialog in big-tauri/src/lib/components/Extract.svelte
 - [ ] T018 [P] [US2] Integration test: request bytes via Tauri command and assert checksum in tests/integration/big-archive-extraction.rs

**User Story 3 — Extract single file (CLI)**

- [ ] T019 [US3] Add `extract` command scaffolding in big-cli/src/commands/extract.rs
- [ ] T020 [US3] Implement CLI `extract` to call `big-core::extract_to_path` and handle `-o` output path in big-cli/src/commands/extract.rs
- [ ] T021 [US3] Add integration test for `big-cli extract` in tests/big-core/pack_tests.rs or tests/integration/

**User Story 4 — Unpack entire archive (CLI)**

- [ ] T022 [US4] Add `unpack` command scaffolding in big-cli/src/commands/pack.rs
- [ ] T023 [US4] Implement `big-cli unpack` to call `big-core::extract_all` and recreate folder hierarchy in big-cli/src/commands/pack.rs
- [ ] T024 [US4] Add progress output and test for `big-cli unpack` in tests/integration/big-archive-extraction.rs

**User Story 5 — Pack directory into .BIG (CLI & library)**

- [ ] T025 [US5] Implement `pack_directory` wiring in big-cli/src/commands/pack.rs to call `big-core::pack_directory`
- [ ] T026 [US5] Add integration test: pack a sample folder and open produced .BIG with big-core in tests/integration/

**User Story 6 — Append single file into existing archive (CLI & UI)**

- [ ] T027 [US6] Implement `append` CLI command in big-cli/src/commands/append.rs to call `big-core::append_file_to_archive` with `--path` and `--force` handling
- [ ] T028 [US6] Add `append_file` Tauri command in big-tauri/src-tauri/src/commands.rs and UI confirmation modal in big-tauri/src/lib/components/Append.svelte
- [ ] T029 [US6] Add integration test for append behavior (fail when exists, succeed with force) in tests/integration/big-archive-extraction.rs

## Final Phase — Polish & Cross-Cutting Concerns

- [ ] T030 Add documentation updates: update README.md and specs/002-big-archive-extraction/quickstart.md with usage examples for `big-cli` and Tauri commands
- [ ] T031 Run workspace formatting and linting (rustfmt) and ensure tests pass
- [ ] T032 Prepare release notes entry and changelog fragment in docs/

## Additional Tasks (from spec coverage)

- [ ] T033 [US5] Add `pack_directory` Tauri command in big-tauri/src-tauri/src/commands.rs
- [ ] T034 [US5] Add Pack UI in big-tauri/src/lib/components/Pack.svelte
- [ ] T035 [P] [US2] Add WAV preview latency/performance test in tests/integration/big-archive-extraction_performance.rs

	- Acceptance: use a canonical 50MB WAV sample (place under `specs/002-big-archive-extraction/samples/50mb.wav`), invoke the Tauri `extract_file_bytes` command (or `big-core::extract_file`) and measure time from request start to when audio playback can start (decode+start). Pass if measured latency <= 200ms on a representative dev machine; record results as a CI artifact (JSON with timings). Include a small harness script that can be run locally to reproduce measurement steps.

## Dependencies

- Story execution order: Phase 1 → Phase 2 → Phase 3 (US1 → US2 → US3 → US4 → US5 → US6) → Final Phase
- Tasks marked `[P]` can run in parallel with unrelated code changes (e.g., tests, CI entries).

- CLI parsing crate: `clap` (v3+) — use `clap`'s derive (`clap_derive`) instead of `structopt` (structopt's functionality is integrated into `clap`).

## Parallel execution examples

- Example A: Implement `extract_file`/`extract_to_path` (`T004/T005/T009/T010`) in parallel with adding CLI scaffolding for `extract` (`T019`) because the CLI can be wired once APIs exist.
- Example B: Frontend UI work (`T011/T017/T018`) can proceed in parallel with `big-core` implementation (`T004-T006`) using mocked Tauri commands until library APIs are ready.

## Independent test criteria (per story)

- US1: UI lists entries for provided sample archives and shows metadata without freezing (tests: T014).
- US2: `extract_file_bytes` returns identical byte sequence to `big-core::extract_file` and UI can save playable WAV (tests: T018).
- US3: `big-cli extract` writes exact bytes to `-o` path (tests: T021).
- US4: `big-cli unpack` writes all entries and preserves folder hierarchy (tests: T024).
- US5: `big-cli pack` produces a valid .BIG that `big-core` can parse (tests: T026).
- US6: `append` fails when target exists and succeeds only with `--force` (tests: T029).

## Implementation strategy (MVP first)

- MVP scope: Deliver US1, US2, US3, and US4 with working `big-core` APIs (`extract_file`, `extract_to_path`, `extract_all`) and basic Tauri commands. Defer UI preview UX polish and advanced progress indicators until polish phase.

## Files created/modified by tasks (quick reference)

- specs/002-big-archive-extraction/tasks.md
- big-core/src/extract.rs
- big-core/src/pack.rs
- big-core/src/paths.rs
- big-core/tests/*
- big-cli/src/commands/extract.rs
- big-cli/src/commands/pack.rs
- big-cli/src/commands/append.rs
- big-tauri/src-tauri/src/commands.rs
- big-tauri/src/lib/components/*.svelte
- tests/integration/big-archive-extraction.rs

---

Generated on: 2026-04-24
