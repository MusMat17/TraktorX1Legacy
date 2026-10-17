# Installation

TraktorX1Legacy v0.1.0 targets **Intel x86_64 / macOS 12 Monterey or newer**.

## Install the app

1. Unzip the release archive.
2. Unzip `TraktorX1Legacy_x86_64-apple-darwin_macos12.0+.zip`.
3. Copy `TraktorX1Legacy.app` to `/Applications`.
4. Connect the Traktor Kontrol X1 Mk1 by USB.
5. Open `TraktorX1Legacy.app`.

The first launch may require Finder's **Open** command or approval in **System Preferences -> Security & Privacy -> General** because the build is ad-hoc signed.

If Gatekeeper still blocks the app:

```sh
xattr -dr com.apple.quarantine /Applications/TraktorX1Legacy.app
open /Applications/TraktorX1Legacy.app
```

## Traktor Pro 4

No custom TSI is required for normal operation.

In **Preferences -> Controller Manager**, use the built-in profile:

- Device: **TraktorKontrol X1**
- In-Port: **KontrolX1**
- Out-Port: **KontrolX1**

The tested v0.1.0 bridge provides working controls and full LED feedback through this built-in profile.

Do **not** install the obsolete Native Instruments X1 Mk1 kernel extension for this bridge.

## Diagnose

From a checkout of this repository:

```sh
chmod +x scripts/diagnose.sh
./scripts/diagnose.sh
```

The app itself also supports:

```sh
/Applications/TraktorX1Legacy.app/Contents/MacOS/traktor-x1-legacy --diagnose
```

## Optional autostart

After confirming that the app works manually:

```sh
chmod +x scripts/install-launchagent.sh
./scripts/install-launchagent.sh
```

Remove autostart with:

```sh
./scripts/uninstall-launchagent.sh
```

Logs are written to `~/Library/Logs/TraktorX1Legacy.log` when the LaunchAgent is used.
