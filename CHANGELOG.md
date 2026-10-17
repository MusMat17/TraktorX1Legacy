# Changelog

## v0.1.1 — 2026-10-17

- Rebuilt the macOS app icon with sharp vector geometry, without Gaussian blur filters.
- Retained the X1 / orange circular arc / small LP design.
- Bumped CFBundleVersion to refresh macOS icon caches.


## v0.1.0 — 2026-10-17

First hardware-validated release.

### Added

- Monterey-style X1 / LP app icon, integrated into Finder and Dock

- userspace USB-to-CoreMIDI bridge for Traktor Kontrol X1 Mk1
- Intel x86_64 macOS application bundle
- macOS 12.0 deployment target validation
- bundled libusb 1.0.30 built for macOS 12.0
- hot-plug and reconnect handling
- SHIFT and HOTCUE layers
- LED feedback
- diagnostics and optional LaunchAgent
- license and third-party notices

### Validated

- MacBookPro9,2
- macOS Monterey 12.7.x via OCLP
- Traktor Pro 4
- built-in `TraktorKontrol X1` device profile
- `KontrolX1` MIDI In/Out
- no custom TSI required

### Fixed

- FX1/FX2 Effect 1/2/3 buttons now emit press and release events so the built-in Traktor X1 profile responds correctly.
