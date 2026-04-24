# Quickstart — BIG Archive Extraction & Packing

1. Open feature spec: [spec.md](specs/002-big-archive-extraction/spec.md)
2. Build workspace:

```bash
cargo build --workspace
```

3. Run CLI examples:

```bash
# List entries
cargo run -p big-cli -- list AudioZH.big

# Extract single file
cargo run -p big-cli -- extract AudioZH.big "Data/Audio/Sounds/foo.wav" -o ./foo.wav

# Unpack entire archive
cargo run -p big-cli -- unpack AudioZH.big -o ./output

# Pack a directory into new archive
cargo run -p big-cli -- pack ./extracted -o ./new.big

# Append a file to archive (requires --force to overwrite)
cargo run -p big-cli -- append ./new.big ./extra/bar.wav --path Data/Audio/Sounds/bar.wav --force
```

4. UI integration (big-tauri): Tauri commands expose pack/append/extract operations; use file pickers to select input and outputs.
