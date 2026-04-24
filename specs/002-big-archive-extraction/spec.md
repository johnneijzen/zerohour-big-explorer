# Feature Specification: BIG Archive Interactive Browsing & Extraction

**Feature Short Name**: big-archive-extraction
**Feature Directory**: specs/002-big-archive-extraction
**Created**: 2026-04-24
**Status**: Draft
**Input**: User request to add interactive browsing, single-file extraction, full-archive unpacking (CLI), and WAV preview support to the BIG Explorer project.

## Overview *(mandatory)*

Extend the existing BIG Explorer to allow users to interactively browse archive contents in the UI, extract single files (UI + CLI), and unpack entire archives from the CLI while preserving directory structure. Preview features will be implemented in a later phase.

## Clarifications

### Session 2026-04-24

- Q: When appending a file into an archive where the target archive-internal path already exists, how should the operation behave? → A: Option B — Fail with an explicit error by default. Overwrite only when the user supplies an explicit confirm/`--force` (or equivalent) option.

## Goals *(mandatory)*

- Click and inspect files in the UI
- Extract a single file (UI + CLI)
- Unpack/Extract entire archive (CLI)
- Pack directory back into a `.BIG` archive (CLI + UI)
- Append a single file into an existing `.BIG` archive (CLI + UI)



## Architecture (high level)

- Reuse `big-core` as the single source of truth for archive parsing and extraction primitives.
- Implement CLI commands in `big-cli` that call `big-core` extraction APIs.
- Expose extraction commands to the Tauri frontend via `#[tauri::command]` wrappers.
- UI renders file list; preview loads bytes via Tauri command and plays via WebAudio using a Blob URL.

## User Scenarios & Testing *(mandatory)*

### Scenario 1 — Browse the archive
**Given** a `.BIG` file is opened, **When** the user views the file list, **Then** they can click entries to view metadata (name, size, type) and preview supported files.

Independent test: open representative archives (small, medium, large) and verify the UI lists entries and remains responsive.

### Scenario 2 — Extract a single file (UI)
**Given** an entry is selected, **When** the user chooses Extract and picks a destination, **Then** the file is written to disk preserving its path (relative to chosen folder) and integrity.

Independent test: extract a WAV from an archive and play the extracted file.

### Scenario 3 — Extract single file (CLI)
`big-cli extract <archive> "Data/Audio/.../file.wav" -o ./file.wav` should write the single file to the specified path.

Independent test: run the command and compare checksum with the archive-extracted bytes returned by `extract_file`.

### Scenario 4 — Unpack entire archive (CLI)
`big-cli unpack <archive> -o ./output` should iterate entries and write files under `./output/<entry.path>`.

Independent test: count entries and verify preserved folder hierarchy and sample file contents.

## big-core (Rust library) Responsibilities *(mandatory)*

- Parse the BIG archive index and provide metadata iterators.
- Provide streaming extraction APIs; avoid loading entire files or archive into memory.
- Validate paths and prevent path traversal when writing to disk.
- Recreate folder structure when extracting all files.

### Data Model

```rust
pub struct BigFileEntry {
    pub name: String,
    pub offset: u64,
    pub size: u64,
}
```

### New public functions (API surface)

```rust
pub fn extract_file(
    archive_path: &str,
    file_entry: &BigFileEntry
) -> Result<Vec<u8>, BigError>;

pub fn extract_to_path(
    archive_path: &str,
    file_entry: &BigFileEntry,
    output_path: &str
) -> Result<(), BigError>;

pub fn extract_all(
    archive_path: &str,
    output_dir: &str
) -> Result<(), BigError>;

/// Pack a directory (with preserved relative paths) into a new .BIG archive
pub fn pack_directory(
    input_dir: &str,
    output_archive: &str
) -> Result<(), BigError>;

/// Append a single file into an existing .BIG archive at the given archive-internal path
pub fn append_file_to_archive(
    archive_path: &str,
    source_file_path: &str,
    archive_target_path: &str,
) -> Result<(), BigError>;

// Behavior note: If `archive_target_path` already exists in the archive, the function MUST
// return an error and make no modification. Overwrite semantics must be opt-in (e.g. a
// higher-level API flag, CLI `--force`, or explicit UI confirmation). This prevents
// accidental corruption or silent replacement of existing entries.
```

#### `extract_all` behavior

- Iterate over entries using a streaming reader (no full-archive buffering).
- For each entry, recreate the parent directory using `std::fs::create_dir_all(parent_dir)?`.
- Write file contents by streaming from the archive to the output file to avoid high memory usage.
- Sanitize entry paths: normalize and reject entries that attempt to escape `output_dir` (e.g., include `..` or absolute paths).

Implementation notes:
- Use `std::fs::File::create` and `std::io::copy` from a limited `Read` slice over the archive to stream bytes.
- Validate and normalize paths using `std::path::Path` and ensure that the resolved path starts with the canonicalized `output_dir`.

## big-cli (Command Line Tool)

### Commands *(mandatory)*

- `big-cli list <archive>` — list entries with sizes (recommended)
- `big-cli extract <archive> <entry-path> -o <outpath>` — extract a single entry to disk
- `big-cli unpack <archive> -o <outdir>` — extract the entire archive preserving structure
 - `big-cli pack <input-dir> -o <archive.big>` — create a new .BIG from a directory tree
 - `big-cli append <archive> <file> --path <archive/internal/path>` — append a single file into an existing archive

CLI behavior notes:
- Use `clap` for argument parsing.
- Validate input archive path and output path arguments.
- Show progress on large archives; e.g., `[{index}/{total}] Extracting {entry.path}`.
- Append command behavior: if the target archive-internal path already exists, the CLI MUST fail with an explicit error unless the user passes `--force` to overwrite. Do not overwrite silently.

## big-tauri (Frontend)

### Tauri commands

```rust
#[tauri::command]
fn extract_file_bytes(path: String, entry: BigFileEntry) -> Result<Vec<u8>, String>;

#[tauri::command]
fn extract_file_to_disk(path: String, entry: BigFileEntry, output: String) -> Result<(), String>;

#[tauri::command]
fn pack_directory(path: String, input_dir: String, output: String) -> Result<(), String>;

#[tauri::command]
fn append_file(path: String, source: String, target_archive_path: String, force: bool) -> Result<(), String>;
```

Behavior:
- `extract_file_bytes` uses `big-core::extract_file` and returns bytes via Tauri IPC so the frontend can save or process the file.
- `extract_file_to_disk` calls `big-core::extract_to_path` and returns status; errors map to user-friendly messages.
- `append_file` should accept an explicit `force` boolean (or the UI must confirm overwrite) — when `force` is false and the target exists, the command returns an error and makes no change.

## UI Behavior *(mandatory)*

- File list: clickable rows showing Name, Size, Type.

- Actions: Extract, Pack, Append. Packing & append UI actions: provide an action to `Pack` a selected folder (or chosen directory) into a new `.BIG` and an action to `Append` a selected file into the currently opened archive. Show progress and review before finalizing.
    - Append confirmation: when appending into an existing archive path, the UI MUST display an overwrite confirmation modal and require explicit user confirmation or a `Force overwrite` toggle. The action should fail without confirmation.

## Security Considerations *(mandatory)*

- Prevent path traversal during `extract_to_path` and `extract_all` by normalizing and canonicalizing output paths.
- Reject entry names containing `..` or absolute paths; on ambiguous cases, skip the entry and log a warning.
- Handle corrupted archives: surface recoverable errors and fail gracefully without panics.

## Performance Requirements *(mandatory)*

- Support large archives (>1GB) by streaming reads and streaming writes.
- Avoid buffering entire files into memory except for small previews; the CLI `extract_all` must stream.

## Functional Requirements *(testable)*
 
- FR-001: CLI can unpack entire archive — run `big-cli unpack` and verify files written and structure preserved.
- FR-002: CLI can extract a single file — run `big-cli extract` and verify the file matches `extract_file` output (checksum equality).
- FR-003: Tauri UI can request file bytes and play WAV previews without freezing the UI (see performance criteria).
- FR-004: Extracted files cannot be written outside the chosen output directory (path traversal prevented).
- FR-005: CLI `big-cli pack` produces a valid `.BIG` that can be opened by the application and contains the expected files and relative paths.
- FR-006: CLI `big-cli append` successfully adds a file entry to an existing archive without corrupting existing entries; append must be atomic.
- FR-007: Tauri `pack_directory` and `append_file` commands invoke `big-core` APIs and report success/failure to the UI.

## Success Criteria *(mandatory & measurable)*

- `big-cli unpack` successfully writes all archive entries and preserves folder structure for canonical test archives included in the integration suite (checksum and file-count checks).
- `big-cli extract` returns the exact bytes for a selected entry (checksum match) for all canonical test entries used by the integration tests.
- WAV preview in UI plays without blocking rendering for single-file previews <= 50MB; playback start latency must be <= 200ms on a representative development machine (SSD, 4-core CPU). Add an integration performance test that measures playback start and UI responsiveness for a 50MB WAV sample.

## Key Entities

- `Archive` — representation of opened BIG file and metadata.
- `BigFileEntry` — metadata for each entry as shown above.

## Assumptions

- Output directories are writable and have sufficient disk space.
- For UI previews, returning file bytes for reasonably small files is acceptable; large files use extraction to disk.

## Non-functional notes

- Do not introduce breaking API changes to `big-core`; add new functions as non-breaking additions.

## Deliverables

- `big-core` exports `extract_file`, `extract_to_path`, and `extract_all`.
- `big-cli` implements `list`, `extract`, and `unpack` commands.
- `big-tauri` exposes Tauri commands for extraction and preview; UI supports clickable file list and WAV playback.
 - `big-core` also exports `pack_directory` and `append_file_to_archive` for packing and appending.
 - `big-cli` also implements `pack` and `append` commands.
 - `big-tauri` exposes `pack_directory` and `append_file` Tauri commands and UI actions to pack/append from the frontend.

<!-- End of spec.md for BIG Archive Interactive Browsing & Extraction -->
