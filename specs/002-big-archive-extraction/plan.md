# Implementation Plan: [FEATURE]

**Branch**: `[###-feature-name]` | **Date**: [DATE] | **Spec**: [link]
**Input**: Feature specification from `/specs/[###-feature-name]/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/plan-template.md` for the execution workflow.

## Summary
Add interactive browsing, single-file extraction (UI + CLI), full-archive unpacking (CLI), packing a directory into a .BIG archive, and appending a single file to an existing .BIG archive. Implementation will re-use `big-core` parsing/IO primitives; `big-cli` will provide `list`, `extract`, `unpack`, `pack`, and `append` commands; `big-tauri` will expose Tauri commands to call the same `big-core` APIs from the UI. The frontend UI must also implement interactive `extract` and `unpack` features so users can extract single files from the explorer and perform full-archive unpacking from the UI.

## Technical Context

<!-- Technical context intentionally summarized above; replace with more details if needed. -->

**Language/Version**: Rust (workspace crates use edition = "2024"). Use the project's existing toolchain; prefer the stable Rust toolchain that supports edition 2024.  
**Primary Dependencies**: `big-core` (internal), `tauri` (big-tauri), `clap` (big-cli uses `clap` v3+ with derive), `serde`, `walkdir`, `anyhow`, `tauri` plugins (dialog, fs).  
**Rationale**: `clap` v3+ integrates the `structopt` derive API (see clap documentation). Prefer `clap` for unified maintenance, documentation parity, and future-proof derive support.
**Storage**: Filesystem-based archives (`.BIG`) — no network storage required.  
**Testing**: `cargo test` for unit tests; integration tests under `tests/` for CLI and archive round-trips.  
**Target Platform**: Desktop (Linux, Windows supported via Tauri).  
**Project Type**: Multi-crate Rust workspace (library + CLI + Tauri frontend).  
**Performance Goals**: Support streaming operations on archives >1GB without loading entire archive into memory; extraction throughput reasonable for sequential disk IO.  
**Constraints**: API additions must be non-breaking for `big-core`; append semantics must be safe (no in-place corruption).  
**Scale/Scope**: Single feature for the explorer; affects `big-core`, `big-cli`, and `big-tauri` crates.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*
*GATE: Constitution file is placeholder; no formal gates enforced automatically. Manual review required for project governance items (testing, release practices).*

## Project Structure

### Documentation (this feature)

```text
specs/002-big-archive-extraction/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
└── contracts/
  └── cli-contract.md
```

### Source Code (repository root)

This repository is a multi-crate Rust workspace. Relevant crates for this feature:

```text
big-core/
├── Cargo.toml
└── src/
  ├── lib.rs
  ├── archive.rs
  ├── parser.rs
  ├── index.rs
  ├── models.rs
  ├── extract.rs
  ├── pack.rs
  ├── iterator.rs
  ├── progress.rs
  ├── preview.rs
  └── preview_handlers/
    ├── audio.rs
    ├── image.rs
    └── text.rs

big-cli/
├── Cargo.toml
└── src/
  ├── main.rs
  └── commands/
    ├── list.rs
    ├── extract.rs
    ├── pack.rs
    ├── append.rs      # new
    └── validate.rs

big-tauri/
├── package.json (frontend)
└── src-tauri/
  ├── Cargo.toml
  └── src/
    ├── main.rs
    ├── lib.rs
    └── commands.rs    # Tauri command bindings for extract/pack/append
  - frontend/           # Svelte UI; add explorer actions for extract/unpack
    - routes/           # UI routes or components for archive browsing
    - components/       # Add `ExtractButton`, `UnpackDialog` components

tests/
├── big-core/ (integration tests)
└── integration/
```

**Structure Decision**: Use the existing multi-crate layout. Implement library logic in `big-core`, expose CLI features in `big-cli` commands, and wire UI actions in `big-tauri/src-tauri/src/commands.rs`.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
