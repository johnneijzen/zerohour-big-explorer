# research.md — BIG Archive Extraction & Packing

Date: 2026-04-24

This document captures design decisions and outstanding clarifications for implementing extraction, packing, and append operations for `.BIG` archives.

Decisions

- Append semantics: Do not perform unsafe in-place appends. Implement `append_file_to_archive` by creating a new temporary archive that copies existing entries and inserts the new file at the requested archive path, then atomically replace the original archive file. This prevents corruption when the archive format stores indexes or headers that require rewriting. The CLI and Tauri layers expose `--force` / `force` to permit overwriting an existing archive-internal path; otherwise append fails.

- Atomicity & safety: Use a temporary file in the same directory (e.g., `archive.big.tmp`) and, after a successful write and validation, rename to the final path. This provides atomic replace on POSIX and minimizes corruption risk.

- Packing approach: `pack_directory` will walk the input directory (using `walkdir`), preserve relative paths, and write entries sequentially into a new `.BIG`. File metadata beyond name/size (timestamps, permissions) is out of scope for MVP unless required later.

- Streaming IO: All read/write operations must stream data (use `std::io::copy` between readers/writers) to support large archives without large memory usage.

- Path sanitization: When extracting or packing, validate paths to avoid directory traversal. For `extract_to_path` and `extract_all`, canonicalize the destination and ensure it starts with the canonicalized `output_dir`. Reject entries with `..` or absolute paths.

- Concurrency: Use advisory file locks (where available) or open files with exclusive access during write operations to reduce race conditions. Document that concurrent mutating operations are unsafe unless the caller ensures external locking.

Open questions (resolved here):

- Overwrite behavior on append: Default to failing with explicit error; `--force`/UI confirmation to overwrite — DECIDED.
- In-place vs rewrite append: Rewrite via temporary file to guarantee correctness — DECIDED.

Implementation notes

- `append_file_to_archive`: open original archive for reading, create temp archive writer, copy entries and insert new file; validate result and atomically replace original. Return descriptive errors on collision unless `force` is true.
- `pack_directory`: walk `input_dir`, compute relative paths, write new archive file. Provide `--compression` flag later if format supports compression.

Tests to add

- Round-trip pack → open → list → extract single file and checksum match.
- Append: append new file, then list and verify presence; test collision rejection and successful overwrite with `force`.
- Large archive streaming: create a large synthetic entry and verify extraction does not allocate O(size) memory.
