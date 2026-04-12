#!/usr/bin/env bash
set -euo pipefail

# Installs git hooks from .githooks into .git/hooks. Run this after cloning.
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
GITHOOKS_DIR="$ROOT/.githooks"
GIT_HOOKS="$ROOT/.git/hooks"

if [ ! -d "$GIT_HOOKS" ]; then
  echo "No .git/hooks directory found. Run from repo root after 'git clone'." >&2
  exit 1
fi

for f in "$GITHOOKS_DIR"/*; do
  name=$(basename "$f")
  dest="$GIT_HOOKS/$name"
  echo "Installing hook $name -> $dest"
  cp "$f" "$dest"
  chmod +x "$dest"
done

echo "Installed git hooks."
