# macOS Monterey / MacBookPro9,2

Validated target for v0.1.0:

- MacBookPro9,2 (13-inch, Mid 2012)
- Intel x86_64
- macOS Monterey 12.7.x via OCLP
- Traktor Pro 4
- Native Instruments Traktor Kontrol X1 Mk1

## Compatibility policy

The CI build sets:

```
MACOSX_DEPLOYMENT_TARGET=12.0
```

and verifies both the executable and bundled libusb with `vtool -show-build`.

This prevents a build made on a modern macOS runner from accidentally depending on a newer macOS release.

## Security model

The bridge does not require the old Native Instruments kext.

Expected configuration:

- SIP can remain enabled, subject to OCLP's own requirements
- no Reduced Security setting is required by TraktorX1Legacy
- the app is ad-hoc signed, so Gatekeeper may require one-time approval

## Traktor configuration

Use the built-in controller profile:

- Device: `TraktorKontrol X1`
- In-Port: `KontrolX1`
- Out-Port: `KontrolX1`

A custom TSI is not required for the validated configuration.

## Hardware validation

Validated working on the real controller:

- FX section, including FX 1/2/3 buttons
- browse and loop encoders
- transport controls
- SHIFT layer
- HOTCUE mode
- LED feedback
- USB reconnect path

The FX 1/2/3 buttons on both sides must emit both press and release events. This is implemented as `Hold` semantics in `board.yml`.
