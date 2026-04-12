Quickstart (local development)

1. Build core and CLI (Rust)

```bash
cargo build --workspace --release
# Run CLI locally
./target/release/big-cli list path/to/data.big
```

2. Run Tauri app in dev

```bash
# from project root
cd big-tauri
pnpm install
pnpm dev
# Tauri dev will spawn the backend Rust commands via `cargo` during development
```

Packaging notes
- Cross-compile `big-cli` for Windows to produce `big-cli.exe` for distribution.
- Use Tauri's bundler for native installers/portable apps.
