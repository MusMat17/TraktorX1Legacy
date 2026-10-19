# TraktorX1Legacy

Userspace compatibility bridge for **Native Instruments Traktor Kontrol X1 Mk1** on modern macOS.

## Tested target

- Mac: **MacBookPro9,2** (13-inch, Mid 2012, Intel x86_64)
- macOS: **Monterey 12.7.x** installed via OpenCore Legacy Patcher
- DJ software: **Traktor Pro 4**
- Controller: **Traktor Kontrol X1 Mk1**

## What it does

```
X1 Mk1 USB -> userspace USB bridge -> CoreMIDI virtual ports -> Traktor Pro
                                      <- LED feedback <-
```

The bridge restores practical X1 Mk1 support without loading the obsolete Native Instruments kernel extension.

## v0.1.0 status

Validated on real hardware:

- Intel x86_64 / macOS 12+
- hot-plug and reconnect
- buttons, encoders and knobs
- SHIFT and HOTCUE layers
- full LED feedback
- built-in Traktor device profile: **TraktorKontrol X1**
- In-Port / Out-Port: **KontrolX1**
- no custom TSI required for normal operation
- no SIP disable and no legacy NI kext required by TraktorX1Legacy

The six FX 1/2/3 buttons use momentary press/release semantics so the built-in Traktor X1 profile handles them correctly.

## macOS application icon

The approved detailed X1/LP artwork is preserved in [`assets/AppIcon.webp`](assets/AppIcon.webp) (SHA-256 pinned), not redrawn as a simplified SVG. The macOS build generates `Contents/Resources/AppIcon-v012.icns` from that actual image.

## Menu bar

Click the 🎛️ icon and choose **Quit TraktorX1Legacy** to close the background bridge. Quit remains available after USB disconnect/reconnect.

## Install

See [docs/INSTALL.md](docs/INSTALL.md).

## Development

- `main`: released/stable code
- `dev`: active development

## Upstream

This project is based in part on protocol work and implementation from:

- Opa-/x1-mk1-usb2midi
- joherold/traktor_x1

The upstream x1-mk1-usb2midi project is licensed under Apache License 2.0. Any reused or modified upstream source retains its original attribution and license terms.
