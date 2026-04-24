# Release notes — BIG Archive Interactive Extraction (specs/002)

Features:

- Interactive archive browsing UI (file listing, metadata)
- Single-file extraction from UI and CLI
- Full-archive unpacking from CLI and UI bindings
- `pack` and `append` support for creating and updating .BIG archives

Notes:

- See `specs/002-big-archive-extraction/quickstart.md` for usage examples.
- Integration and performance tests live under `tests/integration/` and are gated by `RUN_PERF=1` for optional perf runs.
