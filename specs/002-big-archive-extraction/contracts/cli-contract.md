# CLI Contract

This document describes the CLI commands and their behavior for the `big-cli` tool used by the BIG Archive feature.

Commands:

- `list <archive>`
  - Output: list of archive-internal paths and sizes in human-readable text.
  - Exit codes: `0` success, `2` file not found, `3` parse error.

- `extract <archive> <entry-path> -o <outpath>`
  - Behavior: extracts the entry to `outpath`. Creates parent dirs as needed. Streams data.
  - Exit codes: `0` success, `2` entry not found, `3` read error, `4` write error.

- `unpack <archive> -o <outdir>`
  - Behavior: extracts all entries under `outdir`, preserving directory structure.
  - Exit codes: `0` success, `3` parse error, `4` write error.

- `pack <input-dir> -o <archive>`
  - Behavior: create new `.BIG` from directory tree. Overwrites existing output if `--force` provided.
  - Exit codes: `0` success, `4` write error.

- `append <archive> <file> --path <archive/internal/path> [--force]`
  - Behavior: append a file to an existing archive. Fails if archive already contains the target path unless `--force` is specified.
  - Exit codes: `0` success, `2` archive/file not found, `5` conflict (target exists), `4` write error.

Notes:
- CLI should support `--verbose` and `--progress` flags for long-running operations.
- All errors should print helpful messages to `stderr` and return non-zero exit codes as above.
