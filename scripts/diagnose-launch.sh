#!/bin/bash
# macOS Monterey launch diagnosis for TraktorX1Legacy; read-only.
set -u
APP="${1:-/Applications/TraktorX1Legacy.app}"
BIN="$APP/Contents/MacOS/traktor-x1-legacy"
PLIST="$APP/Contents/Info.plist"
ICON="$APP/Contents/Resources/AppIcon.icns"

section() { printf '\n=== %s ===\n' "$1"; }
section "SYSTEM"
sw_vers 2>&1
uname -m
section "INSTALLED APP"
printf 'Path: %s\n' "$APP"
if [[ ! -d "$APP" ]]; then echo "MISSING APP"; exit 2; fi
ls -ld "$APP" "$APP/Contents" "$APP/Contents/MacOS" 2>&1
section "BUNDLE METADATA"
plutil -lint "$PLIST" 2>&1
defaults read "$APP/Contents/Info" CFBundleShortVersionString 2>&1
/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$PLIST" 2>&1
/usr/libexec/PlistBuddy -c 'Print :CFBundleIconFile' "$PLIST" 2>&1
section "EXECUTABLE"
ls -l "$BIN" 2>&1
file "$BIN" 2>&1
otool -L "$BIN" 2>&1
vtool -show-build "$BIN" 2>&1
section "ICON"
ls -lh "$ICON" 2>&1
if [[ -s "$ICON" ]]; then
  file "$ICON"
  shasum -a 256 "$ICON"
fi
section "CODESIGN"
codesign --verify --deep --strict --verbose=3 "$APP" 2>&1
section "GATEKEEPER"
spctl --assess --verbose=3 --type execute "$APP" 2>&1 || true
section "QUARANTINE"
xattr -l "$APP" 2>&1
section "BINARY STARTUP (no GUI)"
if [[ -x "$BIN" ]]; then "$BIN" --version 2>&1; fi
section "RUNNING INSTANCES"
pgrep -fl 'TraktorX1Legacy|traktor-x1-legacy' 2>&1 || true
section "MULTIPLE COPIES IN SPOTLIGHT"
mdfind 'kMDItemCFBundleIdentifier == "io.github.musmat17.traktorx1legacy"' 2>&1 || true
echo "Diagnostics complete. No settings, permissions or security controls changed."
