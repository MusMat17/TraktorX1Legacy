# Launch failure diagnosis (macOS Monterey)

**v0.1.1 known limitation:** CI confirmed a valid Mach-O and bundled `.icns` on macOS 15 Intel, but the app has **not** been verified to launch on a freshly installed Monterey machine. Some users report Finder's "Application can't be opened" error; exact error text / diagnostic is needed to distinguish Gatekeeper from load failures.

## Check the installed copy

```bash
bash scripts/diagnose-launch.sh
```

The script does not change anything. It prints the actual installed app version, executable permissions, architecture, library dependencies, signing verification, quarantine flag, icon checksum, version startup, and any duplicate copies indexed by Spotlight.

On trusted, locally installed builds **only**, macOS Gatekeeper may require Finder `right-click > Open` or an explicit one-time approval in System Preferences > Security & Privacy > General. Do not disable SIP or globally disable Gatekeeper. If a quarantined ad-hoc-signed app is still blocked and you personally trust the download, the local quarantine flag can be removed from **that one app**:

```bash
xattr -dr com.apple.quarantine /Applications/TraktorX1Legacy.app
```

Try `open -a /Applications/TraktorX1Legacy.app` only after verifying its signature.

If the version reported is old, replace the old `.app` in `/Applications` and retest. If the executable runs with `--version` but Finder refuses to open the app, suspect Gatekeeper or LaunchServices. If the executable itself fails, collect the complete terminal error and check linked dynamic libraries.

The X1 bridge is a menu-bar-only application (LSUIElement=true); it normally **does not show a Dock icon** while running.
