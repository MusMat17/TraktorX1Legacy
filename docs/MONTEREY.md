# macOS Monterey / MacBookPro9,2

Target machine for the first release:

- MacBookPro9,2 (13-inch, Mid 2012)
- Intel x86_64
- macOS Monterey 12.7.x via OCLP
- Traktor Pro 4 release compatible with Monterey

## Compatibility policy

The CI build sets:

```
MACOSX_DEPLOYMENT_TARGET=12.0
```

and verifies the produced Mach-O metadata with `vtool -show-build`.

This matters because simply compiling an Intel binary on a modern macOS runner does **not** guarantee that the result will start on Monterey.

## Security model

The bridge does not require the old Native Instruments kext.

Expected configuration:

- SIP can remain enabled (subject to whatever OCLP itself requires on that machine)
- no Reduced Security setting is required by TraktorX1Legacy
- the app is currently ad-hoc signed, so Gatekeeper may require a one-time manual approval

## First machine test

1. Put `TraktorX1Legacy.app` in `/Applications`.
2. Connect the X1 Mk1 directly to the Mac for the first test.
3. Launch the app.
4. Run `scripts/diagnose.sh`.
5. Import the supplied/upstream-compatible TSI into Traktor.
6. In Controller Manager, select the virtual port named with the X1 serial number for both In-Port and Out-Port.
7. Test base buttons, SHIFT, both browse/loop encoders, FX knobs, HOTCUE mode and LED feedback.

If something fails, save the diagnostic output and `~/Library/Logs/TraktorX1Legacy.log`.
