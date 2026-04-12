# Extract & Pack Quickstart

This document shows how to use the Tauri UI and CLI to extract files from and pack directories into `.big` archives.

Using the Tauri UI (big-tauri):

- Open the app and enter the path to a `.big` archive in the explorer input.
- Click `Open` to list entries. Use `Extract` to extract an entry to a destination path.
- Use the `Repack` UI to select a source directory and destination `.big` filename, then run.

Using the CLI (big-cli):

Run `cargo run -p big-cli -- list <archive.big> --json` to list entries.
Run `cargo run -p big-cli -- extract <archive.big> <entry> <dest>` to extract an entry.
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
