# Feature Specification: ZeroHour BIG Explorer

**Feature Branch**: `001-big-archive-explorer`
**Created**: 2026-04-12
**Status**: Draft
**Input**: User description: "ZeroHour BIG Explorer is a desktop application that allows users to inspect, extract, and modify .BIG archive files used in Command & Conquer: Generals – Zero Hour. The application provides a file-tree-based explorer UI with optional preview capabilities for supported asset types."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Open Archive (Priority: P1)
A user opens a `.BIG` file to inspect its contents.

**Why this priority**: Core read-only inspection is the primary value.

**Independent Test**: Open a sample `.BIG` file and verify file tree is populated and responsive.

**Acceptance Scenarios**:
1. **Given** the app is launched, **When** the user selects or drops a `.BIG` file, **Then** the system parses the archive and displays a folder/file tree.
2. **Given** a large archive (>100MB), **When** opened, **Then** a loading state is shown and UI remains responsive.

---

### User Story 2 - Browse Contents (Priority: P1)
A user navigates the archive structure and searches for files.

**Why this priority**: Navigation and search are essential to find assets quickly.

**Independent Test**: Use tree navigation and search filter to locate files by name or path.

**Acceptance Scenarios**:
1. **Given** a parsed archive, **When** the user expands folders, **Then** file names, sizes, and detected types are visible.
2. **Given** a search query, **When** the user types a filename fragment, **Then** matching files are shown in results.

---

### User Story 3 - Extract Files (Priority: P1)
A user extracts individual files, folders, or the entire archive to disk.

**Why this priority**: Extraction is a primary action users need for modding or inspection.

**Independent Test**: Extract a single file, a folder, and the entire archive; verify destination structure and file integrity.

**Acceptance Scenarios**:
1. **Given** selection of one or more files, **When** the user chooses Extract and picks a destination, **Then** files are written preserving folder paths.
2. **Given** extraction of entire archive, **When** process completes, **Then** destination contains all files and folder structure.

---

### User Story 4 - Repack / Insert Files (Priority: P2)
A user modifies an extracted structure and creates a new `.BIG` archive.

**Why this priority**: Important for modders but lower risk than read/extract.

**Independent Test**: Replace a file in an extracted tree, repack to a new `.BIG`, open the new archive, and verify the replacement.

**Acceptance Scenarios**:
1. **Given** a modified file set, **When** the user selects Repack, **Then** a new `.BIG` is created and original is not overwritten by default.
2. **Given** an inconsistent file table or missing entries, **When** repacking is requested, **Then** the system validates and warns the user.

---

### Edge Cases
- Corrupt or truncated `.BIG` header: system must fail gracefully and show readable error.
- Extremely large archives (≈1GB): parsing must be streamed and non-blocking.
- Name collisions on extract destination: prompt user for overwrite behavior.

## Requirements *(mandatory)*

### Functional Requirements
- **FR-001**: System MUST open a `.BIG` file and parse its header and file index.
- **FR-002**: System MUST display files in a folder-tree UI with name, size, and detected type.
- **FR-003**: System MUST provide search/filter support for file names and paths.
- **FR-004**: System MUST allow extracting a single file, selected folder(s), or the entire archive while preserving folder structure.
- **FR-005**: System MUST allow adding new files, replacing existing files, and deleting files in an extracted structure.
- **FR-006**: System MUST create a new `.BIG` when repacking; never overwrite the original by default.
- **FR-007**: System MUST validate file table consistency before writing a new archive and warn on inconsistencies.
- **FR-008**: System MUST stream and lazy-load file data to prevent UI blocking for large archives.
- **FR-009**: System MUST show progress and loading states for long-running operations.

### Key Entities
- **Archive**: Represents a .BIG file and its metadata.
- **FileEntry**: `{ path: string, offset: number, size: number, raw_data: buffer reference }` — describes each file inside the archive.

## Success Criteria *(mandatory)*

### Measurable Outcomes
- **SC-001**: Users can open and view the file tree of a 1GB archive without UI freeze (operation may stream); UI remains responsive.
- **SC-002**: 95% of single-file extractions complete successfully and preserve folder paths in acceptance tests.
- **SC-003**: Repacked archives open and list the same number of entries as expected for a standard test set.
- **SC-004**: Search returns matching results within 1 second for typical archive sizes (<200MB).

## Assumptions
- Target platform is desktop; support for Windows and Linux is expected for MVP.
- The feature will not modify original `.BIG` files in-place.
- Supported preview types are limited for MVP to text, images, and basic audio playback.
- No online or cloud features are required for MVP.

## Testing & Verification
- Provide a test corpus of representative `.BIG` files (small, medium, large, and intentionally corrupted) to validate parsing, extraction, and repack.
- Unit tests for parser and packer validating offsets, sizes, and entry tables.
- Manual UI acceptance tests for extraction, repack, search, and error handling.

## Non-Goals (MVP)
- Full 3D model preview or animation timeline — placeholders only.
- In-place modification of original archives by default.

## Risks & Mitigations
- Risk: Large archives may exhaust memory if preloaded. Mitigation: stream and lazy-load data.
- Risk: Repacked archives could be incompatible. Mitigation: validate entries and add regression tests using known-good `.BIG` samples.

## Deliverables (MVP)
- Open `.BIG` file and display tree.
- Extract files/folders/archive with preserved structure.
- Repack to a new valid `.BIG` file.
- Search and basic previews for supported types.


<!-- End of spec.md -->