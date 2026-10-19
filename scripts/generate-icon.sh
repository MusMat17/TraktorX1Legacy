#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SOURCE="$ROOT/assets/AppIcon.webp"
OUTPUT="${1:-$ROOT/dist/AppIcon-v012.icns}"
EXPECTED_SHA256="19ced214ead961f10cca137da493a692273c94f4fc8e615dcdaa1a39cbfa7c59"
command -v dwebp >/dev/null || { echo "Install WebP decoder (brew install webp)" >&2; exit 1; }
command -v iconutil >/dev/null || { echo "iconutil is required on macOS" >&2; exit 1; }
command -v sips >/dev/null || { echo "sips is required on macOS" >&2; exit 1; }
echo "$EXPECTED_SHA256  $SOURCE" | shasum -a 256 -c -
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT
BASE="$TMP/original.png"
ICONSET="$TMP/AppIcon.iconset"
mkdir -p "$ICONSET"
dwebp "$SOURCE" -quiet -o "$BASE"
for entry in '16:icon_16x16.png' '32:icon_16x16@2x.png' '32:icon_32x32.png' '64:icon_32x32@2x.png' '128:icon_128x128.png' '256:icon_128x128@2x.png' '256:icon_256x256.png' '512:icon_256x256@2x.png' '512:icon_512x512.png' '1024:icon_512x512@2x.png'; do
  size="${entry%%:*}"
  filename="${entry#*:}"
  sips --resampleHeightWidth "$size" "$size" --out "$ICONSET/$filename" "$BASE" >/dev/null
done
mkdir -p "$(dirname "$OUTPUT")"
iconutil -c icns "$ICONSET" -o "$OUTPUT"
echo "Generated $OUTPUT from detailed X1/LP artwork"
