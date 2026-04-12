# Contributing

Thanks for your interest in contributing to ZeroHour BIG Explorer.

Getting started

- Fork the repository and create a feature branch.
- Run the test suite: `cargo test --workspace`.
- Format code with `cargo fmt` and check clippy with `cargo clippy`.

Submitting changes

- Open a pull request against the `main` branch.
- Include tests for new functionality where applicable.
- Keep changes focused and include a short description of the intent.

Code of Conduct

Please be respectful and follow standard community guidelines.
# Contributing

Thanks for your interest in contributing to ZeroHour BIG Explorer.

Guidelines:

- Run `cargo test --workspace` before opening PRs.
- Follow repository formatting; run `cargo fmt` and `cargo clippy` locally.
- For frontend changes, run the Tauri/Svelte dev server and test UI flows.
- Open small, focused PRs and reference relevant task IDs from `specs/001-big-archive-explorer/tasks.md`.

Reporting issues:

- Provide reproduction steps and sample `.BIG` files when possible.
# Contributing

Thank you for your interest in contributing to ZeroHour BIG Explorer.

Guidelines

- Fork the repository and open a pull request against `main`.
- Run `cargo build --workspace` and `cargo test --workspace` before submitting.
- For frontend changes, run `npm ci` and `npm run build` in `big-tauri`.
- Keep changes small and focused; include tests for core logic where applicable.

Code style

- Rust: run `cargo fmt` and address `clippy` warnings where reasonable.

Reporting bugs

- Open an issue with steps to reproduce and sample files if possible.
