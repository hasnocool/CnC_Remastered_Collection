#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

STYLE="${1:-neon}"
OUT="${2:-assets}"

cargo run -q -p cnc-asset --bin cnc-asset-cli -- generate-atlas --out "$OUT" --style "$STYLE"
echo "sprite atlas written to $OUT"
