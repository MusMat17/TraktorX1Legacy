#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SVG="$ROOT/assets/AppIcon.svg"
OUTPUT="${1:-$ROOT/dist/AppIcon.icns}"
command -v rsvg-convert >/dev/null || { echo "Install librsvg (brew install librsvg)" >&2; exit 1; }
command -v iconutil >/dev/null || { echo "iconutil is required (macOS)" >&2; exit 1; }
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT
ICONSET="$TMP/AppIcon.iconset"
mkdir -p "$ICONSET"
for entry in '16:icon_16x16.png' '32:icon_16x16@2x.png' '32:icon_32x32.png' '64:icon_32x32@2x.png' '128:icon_128x128.png' '256:icon_128x128@2x.png' '256:icon_256x256.png' '512:icon_256x256@2x.png' '512:icon_512x512.png' '1024:icon_512x512@2x.png'; do
  size="${entry%%:*}"
  filename="${entry#*:}"
  rsvg-convert --width "$size" --height "$size" --format png "$SVG" > "$ICONSET/$filename"
done
mkdir -p "$(dirname "$OUTPUT")"
iconutil -c icns "$ICONSET" -o "$OUTPUT"
echo "Generated $OUTPUT"
