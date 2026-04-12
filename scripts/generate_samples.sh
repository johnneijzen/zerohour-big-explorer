#!/usr/bin/env bash
set -euo pipefail

# Generate sample .BIG archives using the workspace packer (big-cli).
# Requires Rust toolchain available and cargo in PATH.

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT_DIR/specs/001-big-archive-explorer/samples"
mkdir -p "$OUT_DIR"

echo "Generating small sample..."
TMP_SRC="$OUT_DIR/tmp_small_src"
rm -rf "$TMP_SRC" && mkdir -p "$TMP_SRC"
echo "hello" > "$TMP_SRC/a.txt"
echo "world" > "$TMP_SRC/b.txt"
cargo run -p big-cli --release -- pack "$TMP_SRC" "$OUT_DIR/small.big"

echo "Generating medium sample..."
TMP_SRC2="$OUT_DIR/tmp_medium_src"
rm -rf "$TMP_SRC2" && mkdir -p "$TMP_SRC2"
for i in $(seq 1 200); do printf "file %s\n" "$i" > "$TMP_SRC2/$i.txt"; done
cargo run -p big-cli --release -- pack "$TMP_SRC2" "$OUT_DIR/medium.big"

echo "Generating corrupt sample..."
dd if=/dev/urandom of="$OUT_DIR/corrupt.big" bs=1 count=128 >/dev/null 2>&1 || true

echo "Samples generated in $OUT_DIR"
