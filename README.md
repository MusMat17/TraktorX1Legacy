# TraktorX1Legacy

Userspace compatibility bridge for **Native Instruments Traktor Kontrol X1 Mk1** on modern macOS.

## Target system

- Mac: **MacBookPro9,2** (13-inch, Mid 2012, Intel x86_64)
- macOS: **Monterey 12.7.x** installed via OpenCore Legacy Patcher
- DJ software: **Traktor Pro 4** (Monterey-compatible release)
- Controller: **Traktor Kontrol X1 Mk1**

## Goal

Restore practical X1 Mk1 support without loading Native Instruments' obsolete kernel extension:

```
X1 Mk1 USB -> userspace USB bridge -> CoreMIDI virtual ports -> Traktor Pro
                                      <- LED feedback <-
```

Planned v0.1 scope:

- Intel x86_64 build for macOS 12
- hot-plug/reconnect
- buttons, encoders and knobs
- SHIFT and HOTCUE layers
- LED feedback
- Traktor mapping
- diagnostics and install/run helpers
- no SIP disable and no legacy NI kext

## Development

Stable documentation lives on `main`. Active development is done on `dev` until a tested build is ready.

## Upstream

This project is based in part on the protocol work and implementation from:

- Opa-/x1-mk1-usb2midi
- joherold/traktor_x1

The upstream x1-mk1-usb2midi project is licensed under Apache License 2.0. Any reused or modified upstream source retains its original attribution and license terms.

## Status

Initial bring-up for MacBookPro9,2 / macOS Monterey / Traktor Pro 4.
