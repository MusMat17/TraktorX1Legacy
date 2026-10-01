# Installation and first test

This build targets **Intel x86_64 / macOS 12 Monterey or newer**.

## Install

1. Unzip the CI artifact.
2. Copy `TraktorX1Legacy.app` to `/Applications`.
3. Connect the Traktor Kontrol X1 Mk1.
4. Open the app. The first launch may require Finder's **Open** command or approval in Privacy & Security because development builds are ad-hoc signed.
5. Import `DecksAB-upstream-1.3.3.tsi` in Traktor Controller Manager.
6. Select the virtual MIDI port named after the X1 serial number as both **In-Port** and **Out-Port**.

Do not install the obsolete Native Instruments X1 Mk1 kernel extension for this bridge.

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

After the app works manually:

```sh
chmod +x scripts/install-launchagent.sh
./scripts/install-launchagent.sh
```

Remove autostart with:

```sh
./scripts/uninstall-launchagent.sh
```

Logs are written to `~/Library/Logs/TraktorX1Legacy.log` when the LaunchAgent is used.
