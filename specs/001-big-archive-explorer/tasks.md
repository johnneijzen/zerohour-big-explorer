# Tasks for 001-big-archive-explorer

This file contains dependency-ordered, actionable tasks for the ZeroHour BIG Explorer feature. Each task includes owner (core/cli/gui), an estimate (S/M/L), and an explicit file path for the implementation target.

Phase 1 — Setup

 - [x] T001 Create repository workspace layout: add `big-core/`, `big-cli/`, `big-tauri/`, `tests/`, `specs/001-big-archive-explorer/samples/` (owner: core) Estimate: S — Path: `./`
 - [x] T002 Add workspace Cargo.toml and `big-tauri` frontend skeleton (owner: core/gui) Estimate: S — Path: `Cargo.toml`, `big-tauri/package.json`
- [ ] T003 Add initial CI skeleton and contributor guides (owner: core) Estimate: S — Path: `.github/workflows/ci.yml`, `CONTRIBUTING.md`
- [ ] T004 Add sample `.BIG` corpus for testing (small/medium/large/corrupt) (owner: core) Estimate: S — Path: `specs/001-big-archive-explorer/samples/`

Phase 2 — Foundational Core

 - [x] T005 [P] Implement data model scaffolding in `big-core`: `Archive`, `Index`, `Entry`, `RepackJob`, `ValidationResult` (owner: core) Estimate: M — Path: `big-core/src/models.rs`
 - [x] T006 [P] Implement `Archive` open/stream interface (lazy/open, read header) (owner: core) Estimate: M — Path: `big-core/src/archive.rs`
 - [x] T007 [P] Implement `Index` and `Entry` types and streaming readers (owner: core) Estimate: M — Path: `big-core/src/index.rs`, `big-core/src/entry.rs`
 - [ ] T008 [P] Add project formatting/linting and workspace CI hooks (owner: core) Estimate: S — Path: `rustfmt.toml`, `clippy.toml`, `.github/workflows/ci.yml`
 - [ ] T008 [P] Add project formatting/linting and workspace CI hooks (owner: core) Estimate: S — Path: `rustfmt.toml`, `clippy.toml`, `.github/workflows/ci.yml`

 - [x] T009 [US1] Implement archive parser (header + file index) (owner: core) Estimate: M — Path: `big-core/src/parser.rs`
 - [x] T010 [US1] Implement lazy entry iterator and metadata resolver (owner: core) Estimate: M — Path: `big-core/src/iterator.rs` — Depends: T009
 - [x] T011 [US1] Implement `big-cli list` command with `--json` and `--filter` (owner: cli) Estimate: S — Path: `big-cli/src/commands/list.rs` — Depends: T009,T010
 - [x] T009.1 [US1] Parser/index: implement simple on-disk header and index parsing; add tests (owner: core) Estimate: S — Path: `big-core/src/parser.rs`, `big-core/src/index.rs`, `big-core/tests/parser_tests.rs`
Phase 3 — User Story Phases

US1 — Open Archive (P1)

- [ ] T009 [US1] Implement archive parser (header + file index) (owner: core) Estimate: M — Path: `big-core/src/parser.rs` — Depends: T005,T006,T007
- [ ] T010 [US1] Implement lazy entry iterator and metadata resolver (owner: core) Estimate: M — Path: `big-core/src/iterator.rs` — Depends: T009
- [ ] T011 [US1] Implement `big-cli list` command with `--json` and `--filter` (owner: cli) Estimate: S — Path: `big-cli/src/commands/list.rs` — Depends: T009,T010
- [ ] T012 [US1] Implement Tauri command `list_archive` (RPC) invoking `big-core` (owner: gui) Estimate: S — Path: `big-tauri/src-tauri/commands.rs` — Depends: T009,T010
- [ ] T013 [US1] Add UI: open/archive select & initial tree view (owner: gui) Estimate: M — Path: `big-tauri/src/ui/Explorer.svelte` — Depends: T012
- [ ] T014 [US1] Unit tests for parser/index (owner: core) Estimate: M — Path: `tests/big-core/parser_tests.rs` — Depends: T009

US2 — Browse Contents (P1)

- [ ] T015 [US2] Implement search/filter functions in core (owner: core) Estimate: S — Path: `big-core/src/search.rs` — Depends: T009
- [ ] T016 [US2] Extend `big-cli list` to support `--filter` output (owner: cli) Estimate: S — Path: `big-cli/src/commands/list.rs` — Depends: T011,T015
- [ ] T017 [US2] Implement tree UI + search input and client-side filtering (owner: gui) Estimate: M — Path: `big-tauri/src/ui/Explorer.svelte` — Depends: T013,T015
- [ ] T018 [US2] Integration test: tree navigation and search (owner: core/gui) Estimate: M — Path: `tests/integration/tree_search.rs` — Depends: T011,T013,T015

US3 — Extract Files (P1)

- [ ] T019 [US3] Implement streaming read & safe extraction API (owner: core) Estimate: M — Path: `big-core/src/extract.rs` — Depends: T010
 - [x] T019 [US3] Implement streaming read & safe extraction API (owner: core) Estimate: M — Path: `big-core/src/extract.rs` — Depends: T010
- [ ] T020 [US3] Implement `big-cli extract` with `--dry-run` and `--preserve-permissions` (owner: cli) Estimate: S — Path: `big-cli/src/commands/extract.rs` — Depends: T019
- [ ] T021 [US3] Implement Tauri command `extract_entry` (owner: gui) Estimate: S — Path: `big-tauri/src-tauri/commands.rs` — Depends: T019
- [ ] T022 [US3] UI: extract UX, progress indicator, destination chooser (owner: gui) Estimate: M — Path: `big-tauri/src/ui/ExtractDialog.svelte` — Depends: T021
- [ ] T023 [US3] Unit + integration tests for extraction, including large-file streaming tests (owner: core) Estimate: M — Path: `tests/big-core/extract_tests.rs` — Depends: T019,T020

US4 — Repack / Insert Files (P2)

- [ ] T024 [US4] Implement `RepackJob`, deterministic packing, and write path (owner: core) Estimate: L — Path: `big-core/src/pack.rs` — Depends: T005,T007
- [ ] T025 [US4] Implement validation routines and `ValidationResult` (owner: core) Estimate: M — Path: `big-core/src/validate.rs` — Depends: T024
- [ ] T026 [US4] Implement `big-cli pack` and `big-cli validate` commands (owner: cli) Estimate: M — Path: `big-cli/src/commands/pack.rs`, `big-cli/src/commands/validate.rs` — Depends: T024,T025
- [ ] T027 [US4] Implement Tauri `pack_directory` command (owner: gui) Estimate: M — Path: `big-tauri/src-tauri/commands.rs` — Depends: T024,T025
- [ ] T028 [US4] UI: repack workflow and progress UI (owner: gui) Estimate: M — Path: `big-tauri/src/ui/Repack.svelte` — Depends: T027
- [ ] T029 [US4] Packer + validation test suite, including regression with known-good samples (owner: core) Estimate: L — Path: `tests/big-core/pack_tests.rs` — Depends: T024,T025

Preview Handlers (cross-cutting)

- [ ] T030 [P] Define preview handler trait and registry (owner: core) Estimate: M — Path: `big-core/src/preview.rs`
- [ ] T031 [P] Implement text, image, and basic audio preview handlers (owner: core) Estimate: M — Path: `big-core/src/preview_handlers/`
- [ ] T032 [P] UI: preview pane and handler integration (owner: gui) Estimate: S — Path: `big-tauri/src/ui/Preview.svelte` — Depends: T031

Packaging & Cross-Platform

- [ ] T033 Configure Tauri packaging and cross-compile scripts (owner: gui) Estimate: M — Path: `big-tauri/tauri.conf.json`, `scripts/package.sh` — Depends: T002
- [ ] T034 Add cross-compile script for `big-cli` (owner: core) Estimate: M — Path: `scripts/cross-compile.sh` — Depends: T003

CI & Releases

- [ ] T035 [P] Implement CI workflow: build + test Rust workspace and run frontend checks (owner: core) Estimate: M — Path: `.github/workflows/ci.yml` — Depends: T001,T002,T008
- [ ] T036 [P] Add release pipeline and artifact publishing (owner: core) Estimate: M — Path: `.github/workflows/release.yml` — Depends: T033,T034

Docs & Quickstart

- [ ] T037 [P] Write developer README and quickstart docs with build/run steps (owner: core) Estimate: S — Path: `specs/001-big-archive-explorer/README.md` — Depends: T001-T004
- [ ] T038 [P] Write user-facing docs for extract/pack/validate flows (owner: gui) Estimate: S — Path: `docs/user/extract_pack.md` — Depends: T019,T026

Final Phase — Polish & Cross-Cutting

- [ ] T039 Polish UI, accessibility tweaks, and error messaging (owner: gui) Estimate: M — Path: `big-tauri/src/ui/` — Depends: T013,T017,T022,T028
- [ ] T040 Run performance regression and memory profiling for large archives (owner: core) Estimate: M — Path: `benchmarks/` — Depends: T009,T019,T024

Dependency Graph (high level)

- Setup: T001 → T002 → T003 → T004
- Foundational: T005,T006,T007 -> enables parser/index (T009,T010)
- US1: T009 -> T010 -> T011,T012 -> T013 -> T014
- US2: T015 -> T016,T017 -> T018
- US3: T019 -> T020,T021 -> T022 -> T023
- US4: T024 -> T025 -> T026,T027 -> T028 -> T029
- Previews: T030 -> T031 -> T032
- Packaging/CI: T002,T003,T008 -> T033,T034 -> T035,T036

Parallelization hints

- Tasks marked with `[P]` are candidates for parallel work (different files or non-blocking). Examples: T005/T006/T007 can be developed in parallel by different engineers; preview handlers (T030-T032) can be implemented independently from packer (T024).

Task Ownership Conventions

- `core` = Rust library `big-core` and tests.
- `cli` = Rust binary `big-cli` and CLI-specific glue.
- `gui` = Tauri frontend (`big-tauri`) and UI components.

Notes

- Each task description includes the suggested file path where the implementation should live. Maintain API stability between `big-core`, `big-cli`, and `big-tauri` by following the contracts in `specs/001-big-archive-explorer/contracts/`.
- Where tests are listed, prefer small unit tests first, then focused integration tests using the `specs/.../samples/` corpus.
