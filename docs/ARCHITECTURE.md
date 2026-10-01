# Architecture

TraktorX1Legacy intentionally avoids the obsolete Native Instruments kernel extension.

```
Traktor Kontrol X1 Mk1
        |
        | USB 17cc:2305
        v
      libusb/rusb
        |
        v
 TraktorX1Legacy process
   |              ^
   | MIDI events  | LED feedback
   v              |
 CoreMIDI virtual input/output
        |
        v
   Traktor Pro
```

## Why userspace

The X1 Mk1 is not a class-compliant MIDI device. Native Instruments historically supplied a proprietary driver. A userspace libusb bridge can access the controller directly and expose virtual CoreMIDI endpoints without reviving the legacy kext.

## USB protocol

The current implementation follows the protocol mapping established by the upstream projects:

- vendor ID: `0x17cc`
- product ID: `0x2305`
- state packets: 24 bytes
- LED output: 32 bytes
- state endpoint: `0x84`
- LED output endpoint: `0x01`
- LED acknowledgement/unlock endpoint: `0x81`

The physical control-to-byte mapping lives in `board.yml`.

## MIDI layers

- base controls: channel status `0xB0`
- SHIFT controls: base channel + 1
- LED feedback: `0xB2`
- HOTCUE input/feedback: `0xB3` and shifted variant

This scheme is compatible with the upstream Traktor mapping and gives us a stable base for a Traktor Pro 4-specific mapping later.
