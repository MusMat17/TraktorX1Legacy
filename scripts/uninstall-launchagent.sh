#!/usr/bin/env bash
set -euo pipefail

PLIST="$HOME/Library/LaunchAgents/io.github.musmat17.traktorx1legacy.plist"
launchctl bootout "gui/$(id -u)" "$PLIST" 2>/dev/null || true
rm -f "$PLIST"
echo "Removed TraktorX1Legacy LaunchAgent."
