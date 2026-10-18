#!/usr/bin/env bash
# Generate the AI sprite atlas used by the Rust remaster.
#
#   scripts/generate_ai_assets.sh            # render + assemble
#   scripts/generate_ai_assets.sh generate   # SDXL renders into assets/ai_raw
#   scripts/generate_ai_assets.sh assemble   # pack into assets/atlas.png/json
#
# Requires the local image generator (default /mnt/aux/projects/image-generator);
# override with CNC_IMAGE_GENERATOR.
set -euo pipefail

here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
root=$(dirname "$here")
generator="${CNC_IMAGE_GENERATOR:-/mnt/aux/projects/image-generator}"
python="$generator/.venv/bin/python"

if [[ ! -x "$python" ]]; then
  echo "error: no python at $python (set CNC_IMAGE_GENERATOR)" >&2
  exit 1
fi

cd "$root"

case "${1:-all}" in
  generate) exec "$python" scripts/ai_sprites.py generate "${@:2}" ;;
  assemble) exec "$python" scripts/ai_sprites.py assemble "${@:2}" ;;
  all)      "$python" scripts/ai_sprites.py generate
            exec "$python" scripts/ai_sprites.py assemble ;;
  *) echo "usage: $0 [generate|assemble|all]" >&2; exit 2 ;;
esac
