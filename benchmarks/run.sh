#!/usr/bin/env bash
set -euo pipefail
if [ -z "${1-}" ]; then
  echo "Usage: $0 path/to/archive.big [iterations]"
  exit 2
fi
pushd "$(dirname "$0")" >/dev/null
cargo run --release --manifest-path Cargo.toml -- "$@"
popd >/dev/null
