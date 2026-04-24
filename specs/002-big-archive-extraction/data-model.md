# Data Model — BIG Archive Extraction & Packing

## Key Entities

- `Archive` — Represents a `.BIG` file. Fields: `path: String`, `entry_count: usize`, `size: u64`, `version: Option<u32>`.
- `BigFileEntry` — Metadata for each entry contained in the archive.

```rust
pub struct BigFileEntry {
    pub name: String,     // archive-internal path (e.g., Data/Audio/Sounds/foo.wav)
    pub offset: u64,      // byte offset in archive payload
    pub size: u64,        // uncompressed size
}
```

## Operations

- `extract_file(archive_path, entry) -> Vec<u8>` — returns entry bytes (for small files / preview).
- `extract_to_path(archive_path, entry, output_path)` — writes entry bytes to disk, streaming.
- `extract_all(archive_path, output_dir)` — iterate and write all entries preserving paths.
- `pack_directory(input_dir, output_archive)` — create new `.BIG` including all files under `input_dir` preserving relative paths.
- `append_file_to_archive(archive_path, source_file, archive_target_path, force=false)` — add file entry into archive; fails if target exists unless `force=true`.

## Constraints

- Paths are normalized to forward-slash archive-internal paths.
- On-disk operations must be atomic: write to temp file then rename.
- Streaming IO required for large files (>50MB) to avoid memory spikes.
