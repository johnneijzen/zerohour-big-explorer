Entities

- Archive
  - Fields: `path`, `size`, `format_version`, `index: Index`, `entries_count`
  - Behavior: open stream, read index, iterate entries lazily

- Index
  - Fields: `entries: Vec<Entry>`, ordering, offsets
  - Behavior: resolve entry metadata without fully materializing file payloads

- Entry
  - Fields: `name`, `offset`, `length`, `compressed: bool`, `type`, `hash`, `metadata`
  - Behavior: provide streaming reader, support partial reads, type hints for preview handlers

- RepackJob
  - Fields: `source_dir`, `options {compression, alignment, timestamp_policy}`
  - Behavior: deterministic packing, validation, dry-run mode

- ValidationResult
  - Fields: `errors`, `warnings`, `info`
  - Behavior: structured output suitable for CLI exit codes and GUI display

Notes
- Keep the index and entry models minimal and stream-first to handle very large archives.
