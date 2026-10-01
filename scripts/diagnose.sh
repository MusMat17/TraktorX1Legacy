#!/usr/bin/env bash
set -u

echo "=== TraktorX1Legacy diagnostics ==="
date

echo
echo "=== macOS ==="
sw_vers 2>/dev/null || true
uname -a

echo
echo "=== Hardware ==="
system_profiler SPHardwareDataType 2>/dev/null | grep -E "Model Name|Model Identifier|Processor Name|Processor Speed|Memory" || true

echo
echo "=== SIP ==="
csrutil status 2>/dev/null || true

echo
echo "=== X1 USB device ==="
system_profiler SPUSBDataType 2>/dev/null | grep -B 5 -A 15 -Ei "Traktor|Kontrol X1|17cc|2305" || true

echo
echo "=== MIDI devices ==="
system_profiler SPAudioDataType 2>/dev/null | grep -Ei "MIDI|X1|Traktor" || true

echo
echo "=== Running bridge ==="
pgrep -alf "TraktorX1Legacy|traktor-x1-legacy" || true

echo
echo "=== App binary ==="
APP="/Applications/TraktorX1Legacy.app/Contents/MacOS/traktor-x1-legacy"
if [[ -x "$APP" ]]; then
  file "$APP"
  "$APP" --version || true
  "$APP" --diagnose || true
  echo
  echo "--- linked libraries ---"
  otool -L "$APP" || true
  echo
  echo "--- build version ---"
  vtool -show-build "$APP" 2>/dev/null || true
else
  echo "Not installed at /Applications/TraktorX1Legacy.app"
fi
