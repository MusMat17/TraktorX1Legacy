#!/usr/bin/env bash
set -euo pipefail

APP="/Applications/TraktorX1Legacy.app"
EXEC="$APP/Contents/MacOS/traktor-x1-legacy"
PLIST="$HOME/Library/LaunchAgents/io.github.musmat17.traktorx1legacy.plist"

if [[ ! -x "$EXEC" ]]; then
  echo "Install TraktorX1Legacy.app into /Applications first." >&2
  exit 1
fi

mkdir -p "$HOME/Library/LaunchAgents"

cat > "$PLIST" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>Label</key>
  <string>io.github.musmat17.traktorx1legacy</string>
  <key>ProgramArguments</key>
  <array>
    <string>$EXEC</string>
  </array>
  <key>RunAtLoad</key>
  <true/>
  <key>KeepAlive</key>
  <false/>
  <key>StandardOutPath</key>
  <string>$HOME/Library/Logs/TraktorX1Legacy.log</string>
  <key>StandardErrorPath</key>
  <string>$HOME/Library/Logs/TraktorX1Legacy.log</string>
</dict>
</plist>
PLIST

launchctl bootout "gui/$(id -u)" "$PLIST" 2>/dev/null || true
launchctl bootstrap "gui/$(id -u)" "$PLIST"

echo "Installed LaunchAgent: $PLIST"
