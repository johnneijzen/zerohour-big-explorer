# ZeroHour BIG Explorer — Developer Quickstart

Build and run the workspace:

```bash
cargo build --workspace
cargo test --workspace
```

Run the CLI:

```bash
./target/debug/big-cli list path/to/archive.big
```

Run the Tauri frontend (dev):

```bash
cd big-tauri
npm ci
npm run dev
```
