# Extract & Pack — User Guide (scaffold)

This document will describe how to extract files from archives and repack directories.

Commands (CLI):

- `big-cli list <archive>`
- `big-cli extract <archive> <entry> <dest>`
- `big-cli pack <src> <dest>`

Progress and streaming:

- `--progress`: both `big-cli pack` and `big-cli extract` support a `--progress` flag that will print progress events to stderr while the operation runs.
- The Tauri UI streams progress events from long-running operations (pack/extract) back to the frontend; the UI displays these as progress bars and messages.

UI: The Tauri app provides an Explorer, Extract dialog, and Pack workflow.
